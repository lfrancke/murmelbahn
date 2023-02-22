use crate::app::layer::LayerKind;
use crate::app::rail::RailKind;
use crate::app::BillOfMaterials as AppBillOfMaterials;
use crate::physical::set::SetRepo;
use crate::physical::{Element, Inventory};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::collections::HashMap;
use tracing::trace;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Set [{id}] mentioned in inventory could not be found"))]
    SetUnknown { id: String },

    #[snafu(display(
        "LayerKind [{layer_kind:?}] could not be converted into an element, this should not happen"
    ))]
    UnknownLayerKind {
        layer_kind: LayerKind,
        source: crate::physical::element::Error,
    },

    #[snafu(display(
        "RailKind [{rail_kind:?}] could not be converted into an element, this should not happen"
    ))]
    UnknownRailKind {
        rail_kind: RailKind,
        source: crate::physical::element::Error,
    },
}

/// This is the physical counterpart to [`app::bom::BillOfMaterials`].
/// It contains a list of physical elements that you own or that are needed to build a track.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct BillOfMaterials {
    pub elements: HashMap<Element, i32>,
}

impl TryFrom<AppBillOfMaterials> for BillOfMaterials {
    type Error = Error;

    fn try_from(bom: AppBillOfMaterials) -> Result<Self, Error> {
        let mut elements: HashMap<Element, i32> = HashMap::new();

        // Convert all layers to elements
        // This can, in theory, fail if we don't keep the LayerKind enum in sync with the Element enum
        for (layer_kind, layer_count) in bom.layers.iter() {
            let entry = elements
                .entry(
                    Element::try_from(layer_kind).context(UnknownLayerKindSnafu {
                        layer_kind: layer_kind.clone(),
                    })?,
                )
                .or_insert(0);
            *entry += layer_count;
        }

        for (wall_kind, wall_count) in bom.walls.iter() {
            let entry = elements.entry(Element::from(wall_kind)).or_insert(0);
            *entry += wall_count;
        }

        for (rail_kind, rail_count) in bom.rails.iter() {
            let entry = elements
                .entry(Element::try_from(rail_kind).context(UnknownRailKindSnafu {
                    rail_kind: rail_kind.clone(),
                })?)
                .or_insert(0);
            *entry += rail_count;
        }

        for (tile_kind, tile_count) in bom.tiles.iter() {
            let converted_element = Element::elements_for_tilekind(tile_kind);
            for element in converted_element {
                let entry = elements.entry(element).or_insert(0);
                *entry += tile_count;
            }
        }

        Ok(BillOfMaterials { elements })
    }
}

impl BillOfMaterials {
    /// This sums up all elements from an inventory
    pub fn from_inventory(
        inventory: &Inventory,
        set_repo: &SetRepo,
    ) -> Result<BillOfMaterials, Error> {
        let mut elements = HashMap::new();

        for (set_name, set_count) in inventory.sets.iter() {
            match set_repo.sets.get(set_name) {
                None => {
                    return Err(Error::SetUnknown {
                        id: set_name.to_string(),
                    })
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

        Ok(BillOfMaterials { elements })
    }

    pub fn subtract(&self, other: &BillOfMaterials) -> BillOfMaterials {
        let mut inventory = self.clone();
        for (element, element_count) in other.elements.iter() {
            let entry = inventory.elements.entry(element.clone()).or_insert(0);
            *entry -= element_count;
        }
        inventory
    }

    pub fn any_missing(&self) -> bool {
        for (element, element_count) in self.elements.iter() {
            if element_count < &0 {
                trace!("{:?} is missing {}", element, element_count.abs());
                return true;
            }
        }
        false
    }

}
