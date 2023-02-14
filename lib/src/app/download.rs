//! This module can be used to download courses from the Ravensburger API.
//! The response is a JSON file which mostly just includes a base64 encoded binary file which itself contains the course data.
use crate::common::CourseCode;
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to download course [{course_code}]"))]
    DownloadError {
        course_code: String,
        source: reqwest::Error,
    },

    #[snafu(display("Failed to decode base64 course"))]
    Base64DecodeError { source: base64::DecodeError },

    #[snafu(display("Failed to parse JSON for course [{course_code}]"))]
    JsonParseError {
        course_code: String,
        source: reqwest::Error,
    },
}

#[derive(Debug, Deserialize)]
pub enum SharingCourseType {
    User = 0,
    Editorial = 1,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDownloadResponse {
    pub course_type: SharingCourseType,
    pub course_bytes: String,
}

impl CourseDownloadResponse {
    pub fn decode_base64_file(&self) -> Result<Vec<u8>, Error> {
        general_purpose::STANDARD
            .decode(&self.course_bytes)
            .context(Base64DecodeSnafu)
    }
}

/// Tries downloading a course from Ravensburger.
///
/// Will return errors on any return code that is not 200 or 404.
/// Will return `Ok(None)` for a 404 return code.
pub async fn download_course(code: &CourseCode) -> Result<Option<CourseDownloadResponse>, Error> {
    let url = format!("https://gravitrax.link.ravensburger.com/api/download/{code}");
    let response = reqwest::get(url).await.context(DownloadSnafu {
        course_code: code.to_string(),
    })?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let response_json =
        response
            .json::<CourseDownloadResponse>()
            .await
            .context(JsonParseSnafu {
                course_code: code.to_string(),
            })?;
    Ok(Some(response_json))
}
