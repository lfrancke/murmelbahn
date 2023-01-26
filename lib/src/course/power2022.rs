use deku::prelude::*;
use serde::Serialize;
use crate::course::common::layer::LayerConstructionData;
use crate::course::common::pillar::PillarConstructionData;
use crate::course::common::rail::RailConstructionData;
use crate::course::common::wall::WallConstructionData;
use crate::course::common::{
    CourseElementGeneration, CourseMetaData, CourseSaveDataVersion,
};

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
