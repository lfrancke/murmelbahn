use crate::course::common::HexVector;
use deku::prelude::*;
use serde::Serialize;

#[derive(Debug, DekuRead, Serialize)]
pub struct PillarConstructionData {
    pub lower_layer_id: i32,
    pub lower_cell_local_position: HexVector,

    pub upper_layer_id: i32,
    pub upper_cell_local_position: HexVector,
}
