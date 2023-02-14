use deku::prelude::*;
use serde::Serialize;

use crate::app::course::{CourseElementGeneration, CourseMetaData, CourseSaveDataVersion};
use crate::app::layer::LayerConstructionData;
use crate::app::pillar::PillarConstructionData;
use crate::app::rail::RailConstructionData;
use crate::app::wall::WallConstructionData;

#[deku_derive(DekuRead)]
#[derive(Debug, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct Course {
    pub meta_data: CourseMetaData,

    #[deku(temp)]
    layer_construction_data_size: u32,
    #[deku(ctx = "version")]
    #[deku(count = "layer_construction_data_size")]
    pub layer_construction_data: Vec<LayerConstructionData>,

    #[deku(temp)]
    rail_construction_data_size: u32,
    #[deku(ctx = "version")]
    #[deku(count = "rail_construction_data_size")]
    pub rail_construction_data: Vec<RailConstructionData>,

    #[deku(temp)]
    pillar_construction_data_size: u32,
    #[deku(count = "pillar_construction_data_size")]
    pub pillar_construction_data: Vec<PillarConstructionData>,

    pub generation: CourseElementGeneration,

    #[deku(temp)]
    wall_construction_data_size: i32,
    #[deku(ctx = "version")]
    #[deku(count = "wall_construction_data_size")]
    pub wall_construction_data: Vec<WallConstructionData>,
}
