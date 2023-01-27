use metrics::increment_counter;
use snafu::{ResultExt, Snafu};
use murmelbahn_lib::common::CourseCode;
use sqlx::{Pool, Postgres};
use tokio::task::{JoinError, spawn_blocking};
use tracing::{debug, info};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Database error: {}", source))]
    DatabaseError { source: sqlx::Error },
    DownloadError { source: JoinError }
}

pub struct CourseRepo {
    db: Pool<Postgres>
}

impl CourseRepo {
    pub fn new(db: Pool<Postgres>) -> CourseRepo {
        CourseRepo {
            db
        }
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

        let course = self.course_from_db(&course_code).await?;

        match course {
            None => {
                info!("{} not found in cache, downloading", course_code);
                increment_counter!("murmelbahn.course.cache.miss");

                Ok(Some(self.download_and_cache_course(&course_code).await?))
            }
            Some(course) => {
                debug!("Serving {} from cache", course_code);
                increment_counter!("murmelbahn.course.cache.hit");
                Ok(Some(course))
            }
        }
    }

    pub async fn course_from_db(&self, course_code: &CourseCode) -> Result<Option<Vec<u8>>, Error>{
        // Check if we already have it in the database
        debug!("Trying to retrieve course {} from DB", course_code);

            Ok(sqlx::query_as("SELECT serialized_bytes FROM courses WHERE code = $1")
                .bind(course_code.as_str())
                .fetch_optional(&self.db)
                .await.context(DatabaseSnafu)?.map(|x: (Vec<u8>,)| x.0))
    }

    // TODO: A 404 on download should not be an error but a None
    pub async fn download_and_cache_course(&self, course_code: &CourseCode) -> Result<Vec<u8>, Error> {
        // Because murmelbahn_lib uses the blocking version, we'll have to wrap it here
        // I'm sure there are better ways to do this
        let code = course_code.clone();
        match spawn_blocking(move || {
            murmelbahn_lib::course::download::download_course(&code)
                .unwrap() // TODO
                .decode_base64_file()
        })
        .await
        {
            Ok(course) => {
                debug!("Successfully downloaded {}", course_code);
                increment_counter!("murmelbahn.course.downloads.success");

                sqlx::query("INSERT INTO courses (code, serialized_bytes) VALUES ($1, $2)")
                    .bind(&course_code.to_string())
                    .bind(&course)
                    .execute(&self.db)
                    .await.context(DatabaseSnafu)?;

                Ok(course)
            }
            Err(err) => {
                info!("Download not successful for {}", course_code);
                increment_counter!("murmelbahn.course.downloads.error");
                Err(err).context(DownloadSnafu)?
            }
        }
    }
}
