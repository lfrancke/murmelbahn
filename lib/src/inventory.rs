use std::collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::bom::BillOfMaterial;
use crate::error::{MurmelbahnError, MurmelbahnResult};
use crate::set::{SetContentElement, SetRepo};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Inventory {
    #[serde(default)]
    pub sets: HashMap<String, i32>,

    #[serde(default)]
    pub extra_elements: HashMap<SetContentElement, i32>
}

pub struct SummarizedInventory {

    pub elements: HashMap<SetContentElement, i32>

}

impl SummarizedInventory {
    fn from_inventory(inventory: &Inventory, set_repo: SetRepo) -> MurmelbahnResult<SummarizedInventory> {

        let mut elements = HashMap::new();

        for (set_name, set_count) in inventory.sets.iter() {
            match set_repo.sets.get(set_name) {
                None => {
                    return Err(MurmelbahnError::MiscError { msg: format!("Set [{}] is not known, can't summarize", set_name)})
                }
                Some(set) => {
                    for (element, element_count) in set.content.iter() {
                        let entry = elements.entry(element.clone()).or_insert(0);
                        *entry += element_count * set_count;
                    }
                }
            }
        }

        for (extra_element, element_count) in inventory.extra_elements.iter() {
            let entry = elements.entry(extra_element.clone()).or_insert(0);
            *entry += element_count;
        }

        Ok(SummarizedInventory {
            elements
        })
    }


    // TODO: Return missing elements
    fn is_buildable_with(&self, bom: &BillOfMaterial) -> bool {
        for (layer_kind, layer_count) in bom.layers.iter() {
            if self.elements.get(&SetContentElement::element_for_layerkind(layer_kind)).unwrap_or(&0) < layer_count {
                return false;
            }
        }

        for (wall_kind, wall_count) in bom.walls.iter() {
            if self.elements.get(&SetContentElement::element_for_wallkind(wall_kind)).unwrap_or(&0) < wall_count {
                return false;
            }
        }

        for (rail_kind, rail_count) in bom.rails.iter() {
            if self.elements.get(&SetContentElement::element_for_railkind(rail_kind)).unwrap_or(&0) < rail_count {
                return false;
            }
        }


        true


    }

}
