//! Pillars are the things holding up additional layers (e.g. the hexagonal clear plates)
use deku::prelude::*;
use serde::Serialize;

use crate::app::course::HexVector;

#[derive(Debug, DekuRead, Serialize)]
pub struct PillarConstructionData {
    pub lower_layer_id: i32,
    pub lower_cell_local_position: HexVector,

    pub upper_layer_id: i32,
    pub upper_cell_local_position: HexVector,
}
