use crate::physical::Element;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Inventory {
    #[serde(default)]
    pub sets: HashMap<String, i32>,

    #[serde(default)]
    pub extra_elements: HashMap<Element, i32>,
}
