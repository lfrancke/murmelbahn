//! SkyTrax courses, save format version 7.
//!
//! SkyTrax connects pieces at hex vertices, so the body carries a connector
//! array that earlier formats do not have, and a layer stores its height as an
//! integer count of small stackers rather than a float. The rail, pillar, wall,
//! cell, and tile-tower structures are shared with the earlier format.

use deku::prelude::*;
use serde::Serialize;

use crate::app::course::{CourseElementGeneration, CourseMetaData, CourseSaveDataVersion};
use crate::app::layer::{CellConstructionData, LayerKind};
use crate::app::pillar::PillarConstructionData;
use crate::app::rail::RailConstructionData;
use crate::app::wall::WallConstructionData;

#[deku_derive(DekuRead)]
#[derive(Debug, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct Course {
    pub meta_data: CourseMetaData,
    pub generation: CourseElementGeneration,

    #[deku(temp)]
    layer_count: i32,
    #[deku(ctx = "version")]
    #[deku(count = "layer_count")]
    pub layers: Vec<Layer>,

    #[deku(temp)]
    rail_count: i32,
    #[deku(ctx = "version")]
    #[deku(count = "rail_count")]
    pub rails: Vec<RailConstructionData>,

    #[deku(temp)]
    pillar_count: i32,
    #[deku(count = "pillar_count")]
    pub pillars: Vec<PillarConstructionData>,

    #[deku(temp)]
    wall_count: i32,
    #[deku(ctx = "version")]
    #[deku(count = "wall_count")]
    pub walls: Vec<WallConstructionData>,

    #[deku(temp)]
    connector_count: i32,
    #[deku(count = "connector_count")]
    pub connectors: Vec<Connector>,
}

/// A layer in a SkyTrax course. The height is an integer count of small
/// stackers, where earlier formats store a float layer height.
#[deku_derive(DekuRead)]
#[derive(Debug, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct Layer {
    pub layer_id: i32,
    pub layer_kind: LayerKind,
    pub pos_x: i32,
    pub pos_y: i32,
    pub small_stacker_height: i32,

    #[deku(temp)]
    cell_count: i32,
    #[deku(ctx = "version")]
    #[deku(count = "cell_count")]
    pub cells: Vec<CellConstructionData>,
}

/// A connector joins two cells at a shared hex vertex.
#[derive(Debug, DekuRead, Serialize)]
pub struct Connector {
    pub pos_x: i32,
    pub pos_y: i32,
    pub height: i32,
}

#[cfg(test)]
mod tests {
    use crate::app::course::{Course, SavedCourse};
    use std::path::PathBuf;

    /// A real SkyTrax course parses end to end: only the trailing sha256 is
    /// left unread, and the connector count matches the known value for this
    /// course (its joining-piece quantity is 10).
    #[test]
    fn skytrax_course_parses_fully() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/test-data/SV46HTJVFS.course");
        if !path.exists() {
            eprintln!("SV46HTJVFS fixture absent, skipping (local-only fixtures)");
            return;
        }
        let bytes = std::fs::read(&path).expect("read fixture");

        use deku::prelude::DekuContainerRead;
        let ((rest, _), course) = <SavedCourse as DekuContainerRead>::from_bytes((&bytes, 0))
            .expect("SkyTrax course parses");

        let Course::SkyTrax(body) = course.course else {
            panic!("expected a SkyTrax course");
        };
        assert_eq!(body.connectors.len(), 10, "connector (joining piece) count");
        assert!(!body.layers.is_empty(), "course has layers");
        assert!(
            rest.len() < 64,
            "only the trailing sha256 should remain, got {} bytes",
            rest.len()
        );
    }

    /// The bill of materials for a SkyTrax course counts its tiles and its 10
    /// connectors, and resolves rail/retainer positions without panicking.
    #[test]
    fn skytrax_course_produces_bill_of_materials() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/test-data/SV46HTJVFS.course");
        if !path.exists() {
            eprintln!("SV46HTJVFS fixture absent, skipping (local-only fixtures)");
            return;
        }
        let bytes = std::fs::read(&path).expect("read fixture");

        let course = SavedCourse::from_bytes(&bytes).expect("parses");
        let bom = crate::app::BillOfMaterials::from(course.course);

        assert_eq!(bom.connectors, 10, "connector count");
        assert!(
            bom.tiles.values().sum::<i32>() > 0,
            "course has counted tiles"
        );
    }
}
