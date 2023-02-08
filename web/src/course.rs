use crate::{AppError, AppState};
use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use metrics::increment_counter;
use murmelbahn_lib::bom::AppBillOfMaterials;
use murmelbahn_lib::common::{CourseCode, GraviSheetOutput};
use murmelbahn_lib::course::common::{Course, SavedCourse};
use serde::Deserialize;
use std::sync::Arc;

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
) -> Result<Response, AppError> {
    increment_counter!("murmelbahn.bom.requests");
    let course_code = CourseCode::new(course);
    let course_bytes = state
        .course_repo
        .get_course_bytes(&course_code)
        .await?
        .unwrap();
    let course = SavedCourse::from_bytes(&course_bytes)?.course;

    return match course {
        Course::ZiplineAdded2019(_) => {
            increment_counter!("murmelbahn.bom.requests.ziplineadded2019");
            Err(AppError::ZiplineAdded2019Unsupported)
        }
        Course::Power2022(course) | Course::Pro2020(course) => {
            let title = course.meta_data.title.clone();
            let bom = AppBillOfMaterials::try_from(course)?;

            Ok(match format {
                Some(BomFormat::Csv) => {
                    let mut wtr = csv::Writer::from_writer(Vec::new());
                    let mut output = GraviSheetOutput::from(bom);
                    output.title = title;
                    output.course_code = course_code.to_string();
                    wtr.serialize(output).unwrap();
                    String::from_utf8(wtr.into_inner().unwrap())
                        .unwrap()
                        .into_response()
                }
                None | Some(BomFormat::Json) => Json(bom).into_response(),
                Some(BomFormat::Rust) => format!("{:#?}", bom).into_response(),
            })
        }
    };
}

/// Dumps a course in JSON format
// TODO: Needs to return a 404
pub async fn course_dump(
    Path(course): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SavedCourse>, AppError> {
    increment_counter!("murmelbahn.dump.requests");

    // Could write a custom Axum extractor at some point
    let course_code = CourseCode::new(course);
    let course_bytes = state
        .course_repo
        .get_course_bytes(&course_code)
        .await?
        .unwrap(); // TODO
    let course = SavedCourse::from_bytes(&course_bytes).unwrap(); // TODO
    Ok(Json(course))
}
