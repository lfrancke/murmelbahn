use crate::course::Error::CourseNotFound;
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use csv::Writer;
use metrics::increment_counter;
use murmelbahn_lib::app::course::SavedCourse;
use murmelbahn_lib::app::BillOfMaterials;
use murmelbahn_lib::common::CourseCode;
use murmelbahn_lib::gravisheet::GraviSheetOutput;
use serde::Deserialize;
use snafu::prelude::*;
use std::string::FromUtf8Error;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to deserialize course [{course_code}]"))]
    DeserializationFailedError {
        course_code: CourseCode,
        source: murmelbahn_lib::app::course::Error,
    },

    #[snafu(display("Error in CourseRepo"))]
    #[snafu(context(false))]
    CourseRepoError { source: crate::course_repo::Error },

    #[snafu(display("Course [{course_code} not found"))]
    CourseNotFound { course_code: CourseCode },

    #[snafu(display("Error serializing response to CSV"))]
    #[snafu(context(false))]
    CsvSerializationError { source: csv::Error },

    #[snafu(display("Error serializing response to CSV"))]
    #[snafu(context(false))]
    CsvSerializationError2 {
        source: csv::IntoInnerError<Writer<Vec<u8>>>,
    },

    #[snafu(display("Could not convert serialized CSV to UTF-8"))]
    #[snafu(context(false))]
    Utf8Error { source: FromUtf8Error },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{:?}", self);
        todo!()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BomFormat {
    Csv,
    Json,
    Rust,
}

#[derive(Default, Deserialize)]
pub(crate) struct BomParams {
    format: Option<BomFormat>,
}

// TODO: Needs to return a 404
pub(crate) async fn course_bom(
    Path(course): Path<String>,
    Query(BomParams { format }): Query<BomParams>,
    State(state): State<Arc<AppState>>,
) -> Result<Response, Error> {
    increment_counter!("murmelbahn.bom.requests");
    let course_code = CourseCode::new(course);
    debug!("Request for BOM for course [{course_code}]");

    let course_bytes = state.course_repo.get_course_bytes(&course_code).await?;
    let Some(course_bytes) = course_bytes else {
        return Ok((
            StatusCode::NOT_FOUND,
            format!("Course [{}] could not be found", course_code),
        )
            .into_response());
    };
    let course = SavedCourse::from_bytes(&course_bytes)
        .context(DeserializationFailedSnafu {
            course_code: course_code.clone(),
        })?
        .course;

    let title = course.meta_data().title.clone();
    let bom = BillOfMaterials::from(course);

    Ok(match format {
        Some(BomFormat::Csv) => {
            let mut wtr = Writer::from_writer(Vec::new());
            let mut output = GraviSheetOutput::from(bom);
            output.title = title;
            output.course_code = course_code.to_string();
            wtr.serialize(output)?;
            let bytes = wtr.into_inner()?;
            String::from_utf8(bytes)?.into_response()
        }
        None | Some(BomFormat::Json) => Json(bom).into_response(),
        Some(BomFormat::Rust) => format!("{bom:#?}").into_response(),
    })
}

/// Dumps a course in JSON format
// TODO: Needs to return a 404
pub async fn course_dump(
    Path(course): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SavedCourse>, Error> {
    increment_counter!("murmelbahn.dump.requests");

    // Could write a custom Axum extractor at some point
    let course_code = CourseCode::new(course);
    let course_bytes = state.course_repo.get_course_bytes(&course_code).await?;

    let Some(course_bytes) = course_bytes else {
        return Err(CourseNotFound { course_code });
    };

    let course = SavedCourse::from_bytes(&course_bytes)
        .context(DeserializationFailedSnafu { course_code })?;
    Ok(Json(course))
}

/// Dumps the raw course data as they come from Ravensburger
/// The only thing we do is to decode base64
pub async fn course_raw_download(
    Path(course): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Vec<u8>, Error> {
    increment_counter!("murmelbahn.raw_download.requests");

    // Could write a custom Axum extractor at some point
    let course_code = CourseCode::new(course);
    let course_bytes = state.course_repo.get_course_bytes(&course_code).await?;

    let Some(course_bytes) = course_bytes else {
        return Err(CourseNotFound { course_code });
    };

    Ok(course_bytes)
}
