use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::error::{DownloadFailedSnafu, MurmelbahnResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDownloadResponse {
    pub course_type: String,
    pub course_file: String,
}

impl CourseDownloadResponse {
    pub fn decode_base64_file(&self) -> Vec<u8> {
        general_purpose::STANDARD.decode(&self.course_file).unwrap()
    }
}

pub fn download_course(code: &str) -> MurmelbahnResult<CourseDownloadResponse> {
    let url = format!(
        "https://gravitrax.link.ravensburger.com/api/download/{}",
        code
    );
    reqwest::blocking::get(url)
        .context(DownloadFailedSnafu {
            course: code.to_string(),
        })?
        .json::<CourseDownloadResponse>()
        .context(DownloadFailedSnafu {
            course: code.to_string(),
        })
}
