use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::bom::BillOfMaterial;

// TODO: Straight rails, Marbles, Walls
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Inventory {
    #[serde(default)]
    pub sets: Vec<String>,

    #[serde(default)]
    pub bill_of_materials: BillOfMaterial
}
