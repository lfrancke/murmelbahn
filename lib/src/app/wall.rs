use deku::prelude::*;
use serde::Serialize;

use crate::app::course::{CourseSaveDataVersion, HexVector};
use crate::app::layer::CellConstructionData;

#[derive(Debug, Eq, Hash, PartialEq, Serialize)]
pub enum WallKind {
    StraightSmall,
    StraightMedium,
    StraightLarge,
}

impl WallKind {
    pub fn straight_of_length(length: i32) -> WallKind {
        match length {
            1 => WallKind::StraightSmall,
            2 => WallKind::StraightMedium,
            3 => WallKind::StraightLarge,
            _ => panic!("Unsupported wall length"),
        }
    }
}

#[derive(Debug, DekuRead, Serialize)]
#[deku(id_type = "u32")]
pub enum WallSide {
    West = 0,
    East = 1,
}

#[derive(Debug, DekuRead, Serialize)]
pub struct WallCoordinate {
    pub column: i32,
    pub row: i32,
}

#[deku_derive(DekuRead)]
#[derive(Debug, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct WallConstructionData {
    pub lower_stacker_tower_1_retainer_id: i32,
    pub lower_stacker_tower_1_local_hex_pos: HexVector,
    pub lower_stacker_tower_2_retainer_id: i32,
    pub lower_stacker_tower_2_local_hex_pos: HexVector,

    #[deku(temp)]
    balcony_construction_data_size: i32,

    #[deku(ctx = "version")]
    #[deku(count = "balcony_construction_data_size")]
    pub balcony_construction_datas: Vec<WallBalconyConstructionData>,
}

#[deku_derive(DekuRead)]
#[derive(Debug, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct WallBalconyConstructionData {
    pub retainer_id: i32,
    pub wall_side: WallSide,
    pub wall_coordinate: WallCoordinate,

    #[deku(temp)]
    has_cell_construction_data: bool,
    #[deku(cond = "*has_cell_construction_data")]
    #[deku(ctx = "version")]
    pub cell_construction_datas: Option<CellConstructionData>,
}
