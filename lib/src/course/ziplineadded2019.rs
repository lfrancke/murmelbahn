use deku::prelude::*;
use serde::Serialize;
use crate::course::common::{CourseElementGeneration, CourseMetaData, HexVector};
use crate::course::common::layer::{LayerKind, TileKind};
use crate::course::common::pillar::PillarConstructionData;
use crate::course::common::rail::RailKind;

// TODO: TileTowerTreeNodeData FromPreProSaveGameData(PreProTileTowerConstructionData oldData)
// That would be a fn from_ziplineadded2019_constructiondata(construction_data: CellConstructionDataZiplineAdded2019) -> CellConstructionData but we'd also need on for Course itself...


#[derive(Debug, DekuRead, Serialize)]
#[deku(type = "u32")]
pub enum RopeKind {
    None = 0,
    Straight = 1,
    TODO = 3
}

#[deku_derive(DekuRead)]
#[derive(Debug, Serialize)]
pub struct Course {
    pub meta_data: CourseMetaData,

    #[deku(temp)]
    layer_construction_data_size: u32,
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


#[derive(Debug, DekuRead, Serialize)]
pub struct RailConstructionExitIdentifier {
    pub retainer_id: u32,
    pub cell_local_hex_pos: HexVector,
    pub side_hex_rot: u32,
}

#[derive(Debug, DekuRead, Serialize)]
pub struct RailConstructionData {
    pub exit_1_identifier: RailConstructionExitIdentifier,
    pub exit_2_identifier: RailConstructionExitIdentifier,
    pub rail_kind: RailKind,
    pub materialized: bool
}
