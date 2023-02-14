use deku::prelude::*;
use serde::Serialize;

use crate::app::course::{
    CourseElementGeneration, CourseMetaData, CourseSaveDataVersion, HexVector,
};
use crate::app::layer::{LayerKind, TileKind};
use crate::app::pillar::PillarConstructionData;
use crate::app::rail::RailConstructionData;

#[derive(Debug, DekuRead, Serialize)]
#[deku(type = "u32")]
pub enum RopeKind {
    None = 0,
    Straight = 1,
    TODO = 3,
}

#[deku_derive(DekuRead)]
#[derive(Debug, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct Course {
    pub meta_data: CourseMetaData,

    #[deku(temp)]
    layer_construction_data_size: u32,
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

    #[deku(temp)]
    rope_construction_data_size: u32,
    #[deku(count = "rope_construction_data_size")]
    pub rope_construction_data: Vec<RopeConstructionData>,

    pub generation: CourseElementGeneration,
}

#[deku_derive(DekuRead)]
#[derive(Debug, Serialize)]
pub struct CellConstructionData {
    pub hex_rotation: i32,
    pub local_hex_position: HexVector,

    #[deku(temp)]
    tile_kind_size: i32,
    #[deku(count = "tile_kind_size")]
    pub tile_kinds: Vec<TileKind>,
}

// TODO: Can this at least be generic over CellConstructionData? Check deku
#[deku_derive(DekuRead)]
#[derive(Debug, Serialize)]
pub struct LayerConstructionData {
    pub layer_id: i32,
    pub layer_kind: LayerKind,
    pub layer_height: f32,
    pub hex_vector: HexVector,

    #[deku(temp)]
    cell_construction_data_size: i32,
    #[deku(count = "cell_construction_data_size")]
    pub cell_construction_data: Vec<CellConstructionData>,
}

#[derive(Debug, DekuRead, Serialize)]
pub struct RopeConstructionData {
    pub start_tile_layer_index: u32,
    pub start_tile_local_hex_pos: HexVector,
    pub end_tile_layer_index: u32,
    pub end_tile_local_hex_pos: HexVector,
    pub rope_kind: RopeKind,
}
