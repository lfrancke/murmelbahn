use chrono::NaiveDateTime;
use futures::TryStreamExt;
use metrics::counter;
use serde::Serialize;

use murmelbahn_lib::app::course::SavedCourse;
use murmelbahn_lib::app::BillOfMaterials as AppBillOfMaterials;
use murmelbahn_lib::common::CourseCode;
use murmelbahn_lib::physical::{BillOfMaterials as PhysicalBillOfMaterials, Inventory, SetRepo};
use snafu::{ResultExt, Snafu};
use sqlx::{Pool, Postgres, Row};
use tracing::{debug, info};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Database error: {}", source))]
    DatabaseError { source: sqlx::Error },
    DownloadError {
        source: murmelbahn_lib::app::download::Error,
    },
    #[snafu(display("Deserialization failed for course with code '{}': {}", code, source))]
    DeserializationError {
        code: String,
        source: murmelbahn_lib::app::course::Error,
    },
    #[snafu(display("Invalid course metadata: {}", message))]
    InvalidMetadata { message: String },
}

pub struct CourseRepo {
    db: Pool<Postgres>,
}

#[derive(Serialize)]
pub struct StoredCourseMetadata {
    pub date_added_to_db: NaiveDateTime,
    /// This is extracted from the metadata of the file itself
    pub creation_timestamp: NaiveDateTime,
    /// This is extracted from the metadata of the file itself
    pub title: String,
    pub course_code: String,
}

impl CourseRepo {
    pub fn new(db: Pool<Postgres>) -> CourseRepo {
        CourseRepo { db }
    }

    /// This gets the bytes for a course from the database or tries to download it from the
    /// Ravensburger API if we haven't cached it yet.
    ///
    /// We treat all courses as immutable and assume that they will never change once downloaded.
    pub async fn get_course_bytes(
        &self,
        course_code: &CourseCode,
    ) -> Result<Option<Vec<u8>>, Error> {
        // Check if we already have it in the database
        info!("Request for course: {}", course_code);

        let course = self.course_from_db(course_code).await?;

        match course {
            None => {
                info!("{} not found in cache, downloading", course_code);
                counter!("murmelbahn.course.cache.miss").increment(1);

                Ok(Some(self.download_and_cache_course(course_code).await?))
            }
            Some(course) => {
                debug!("Serving {} from cache", course_code);
                counter!("murmelbahn.course.cache.hit").increment(1);
                Ok(Some(course))
            }
        }
    }

    pub async fn course_from_db(&self, course_code: &CourseCode) -> Result<Option<Vec<u8>>, Error> {
        // Check if we already have it in the database
        debug!("Trying to retrieve course {} from DB", course_code);

        Ok(
            sqlx::query_as("SELECT serialized_bytes FROM courses WHERE code = $1")
                .bind(course_code.as_str())
                .fetch_optional(&self.db)
                .await
                .context(DatabaseSnafu)?
                .map(|x: (Vec<u8>,)| x.0),
        )
    }

    // TODO: A 404 on download should not be an error but a None
    pub async fn download_and_cache_course(
        &self,
        course_code: &CourseCode,
    ) -> Result<Vec<u8>, Error> {
        // Because murmelbahn_lib uses the blocking version, we'll have to wrap it here
        // I'm sure there are better ways to do this
        let code = course_code.clone();
        let course_bytes = murmelbahn_lib::app::download::download_course(&code)
            .await
            .unwrap()
            .unwrap()
            .decode_base64_file();

        match course_bytes {
            Ok(course) => {
                debug!("Successfully downloaded {}", course_code);
                counter!("murmelbahn.course.downloads.success").increment(1);

                sqlx::query("INSERT INTO courses (code, serialized_bytes) VALUES ($1, $2)")
                    .bind(&course_code.to_string())
                    .bind(&course)
                    .execute(&self.db)
                    .await
                    .context(DatabaseSnafu)?;

                Ok(course)
            }
            Err(err) => {
                info!("Download not successful for {}", course_code);
                counter!("murmelbahn.course.downloads.error").increment(1);
                Err(err).context(DownloadSnafu)?
            }
        }
    }

    /// This checks all courses in the database against the inventory provided and returns all those
    /// that can be built with the inventory.
    pub async fn process_all(
        &self,
        repo: &SetRepo,
        inventory: Inventory,
    ) -> Result<Vec<StoredCourseMetadata>, Error> {
        let mut rows = sqlx::query(
            "SELECT code, serialized_bytes, created_at FROM courses ORDER BY created_at",
        )
        .fetch(&self.db);

        let summarized_inventory =
            PhysicalBillOfMaterials::from_inventory(&inventory, repo).unwrap();

        let mut courses = Vec::new();
        while let Ok(Some(row)) = rows.try_next().await {
            let code: String = match row.try_get("code") {
                Ok(c) => c,
                Err(e) => {
                    info!("Failed to get 'code', skipping: {}", e);
                    continue;
                }
            };


            let bytes: Vec<u8> = match row.try_get("serialized_bytes") {
                Ok(b) => b,
                Err(e) => {
                    info!("Failed to get 'serialized_bytes' for code '{}', skipping: {}", code, e);
                    continue;
                }
            };


            let created_at: NaiveDateTime = match row.try_get("created_at") {
                Ok(c) => c,
                Err(e) => {
                    info!("Failed to get 'created_at' for code '{}', skipping: {}", code, e);
                    continue;
                }
            };

            match SavedCourse::from_bytes(&bytes)
                .context(DeserializationSnafu { code: code.clone() })
                .and_then(|saved_course| {
                    let course = saved_course.course;
                    let metadata = course.meta_data().clone();
                    let app_bom = AppBillOfMaterials::try_from(course)
                        .map_err(|_| Error::InvalidMetadata {
                            message: "Invalid AppBillOfMaterials".to_string(),
                        })?;
                    let physical_bom = PhysicalBillOfMaterials::try_from(app_bom)
                        .map_err(|_| Error::InvalidMetadata {
                            message: "Invalid PhysicalBillOfMaterials".to_string(),
                        })?;
                    Ok((metadata, physical_bom))
                }) {
                Ok((metadata, physical_bom)) => {
                    let diff_bom = summarized_inventory.subtract(&physical_bom);
                    if !diff_bom.any_missing() {
                        courses.push(StoredCourseMetadata {
                            date_added_to_db: created_at,
                            creation_timestamp: NaiveDateTime::from_timestamp_millis(
                                metadata.creation_timestamp as i64,
                            )
                            .unwrap_or_else(|| {
                                info!("Invalid timestamp for course code {}", code);
                                NaiveDateTime::from_timestamp_opt(0, 0).unwrap()
                            }),
                            title: metadata.title,
                            course_code: code,
                        });
                    }
                }
                Err(e) => {
                    info!("Failed to process course '{}': {}", code, e);
                }
            }
        }

        Ok(courses)
    }
}
