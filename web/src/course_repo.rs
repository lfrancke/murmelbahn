use futures::TryStreamExt;
use metrics::increment_counter;

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
}

pub struct CourseRepo {
    db: Pool<Postgres>,
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
                increment_counter!("murmelbahn.course.cache.miss");

                Ok(Some(self.download_and_cache_course(course_code).await?))
            }
            Some(course) => {
                debug!("Serving {} from cache", course_code);
                increment_counter!("murmelbahn.course.cache.hit");
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
        let foo = murmelbahn_lib::app::download::download_course(&code)
            .await
            .unwrap()
            .unwrap()
            .decode_base64_file();

        match foo {
            Ok(course) => {
                debug!("Successfully downloaded {}", course_code);
                increment_counter!("murmelbahn.course.downloads.success");

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
                increment_counter!("murmelbahn.course.downloads.error");
                Err(err).context(DownloadSnafu)?
            }
        }
    }

    pub async fn process_all(
        &self,
        repo: &SetRepo,
        inventory: Inventory,
    ) -> Result<Vec<String>, Error> {
        let mut rows = sqlx::query("SELECT code, serialized_bytes FROM courses").fetch(&self.db);

        let summarized_inventory =
            PhysicalBillOfMaterials::from_inventory(&inventory, repo).unwrap();

        let mut courses = Vec::new();
        while let Some(row) = rows.try_next().await.unwrap() {
            let bytes: Vec<u8> = row.try_get("serialized_bytes").unwrap();
            let code: &str = row.try_get("code").unwrap();
            let course = SavedCourse::from_bytes(&bytes).unwrap().course;

            let app_bom = AppBillOfMaterials::try_from(course).unwrap();
            let physical_bom = PhysicalBillOfMaterials::try_from(app_bom).unwrap();

            let diff_bom = summarized_inventory.subtract(&physical_bom);
            if !diff_bom.any_missing() {
                courses.push(code.to_string());
            }
        }

        Ok(courses)
    }
}
