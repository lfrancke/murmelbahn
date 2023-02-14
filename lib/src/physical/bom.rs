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

    // TODO: The below is old and unused code, but there is some logic in here which needs to be ported to the new one
    /*
    /// This checks whether a certain BOM is buildable with an inventory of elements
    // TODO: Return missing elements
    pub fn is_buildable_with(&self, bom: &AppBillOfMaterials) -> bool {
        for (layer_kind, layer_count) in bom.layers.iter() {
            if self
                .elements
                .get(&SetContentElement::element_for_layerkind(layer_kind))
                .unwrap_or(&0)
                < layer_count
            {
                return false;
            }
        }

        for (wall_kind, wall_count) in bom.walls.iter() {
            if self
                .elements
                .get(&SetContentElement::element_for_wallkind(wall_kind))
                .unwrap_or(&0)
                < wall_count
            {
                return false;
            }
        }

        for (rail_kind, rail_count) in bom.rails.iter() {
            if self
                .elements
                .get(&SetContentElement::element_for_railkind(rail_kind))
                .unwrap_or(&0)
                < rail_count
            {
                return false;
            }
        }

        // TODO: This can become even more sophisticated as many of the curves are interchangeable, ESPECIALLY not all endpoints are actually connected to something else, but that is more complicated to calculate

        // This counts how many basic tiles are needed where it doesn't matter if they are open or closed
        let mut needed_basic_tiles = 0;
        // This counts how many open basic tiles are needed
        let mut needed_basic_open_tiles = 0;

        let mut needed_small_stackers = 0;
        let mut needed_stackers = 0;
        let mut needed_stackers_angled = 0;
        let mut needed_stacker_towers_closed = 0;
        let mut needed_stacker_towers_opened = 0;

        let mut needed_two_ways = 0;
        let mut needed_switch_inserts = 0;

        let mut needed_trampolines = 0;

        let mut needed_lift_entrance = 0;
        let mut needed_lift_heighttube = 0;
        let mut needed_lift_exit = 0;

        let mut needed_screw_entrance = 0;
        let mut needed_screw_base = 0;
        let mut needed_screw_curve = 0;

        for (tile_kind, tile_count) in bom.tiles.iter() {
            match SetContentElement::element_for_tilekind(&tile_kind) {
                // TODO: This is sloppy, move the non case outside and handle it separately (i.e. handle_screws(bom))
                None => match tile_kind {
                    TileKind::Catch => {
                        needed_basic_tiles += 1;
                        if self.elements.get(&SetContentElement::Catch).unwrap_or(&0) < tile_count {
                            return false;
                        }
                    }
                    TileKind::Splash => {
                        needed_basic_tiles += 1;
                        if self.elements.get(&SetContentElement::Splash).unwrap_or(&0) < tile_count
                        {
                            return false;
                        }
                    }
                    TileKind::GoalBasin => {
                        needed_basic_tiles += 1;
                        if self
                            .elements
                            .get(&SetContentElement::GoalBasin)
                            .unwrap_or(&0)
                            < tile_count
                        {
                            return false;
                        }
                    }
                    TileKind::Drop => {
                        needed_basic_open_tiles += 1;
                        if self.elements.get(&SetContentElement::Drop).unwrap_or(&0) < tile_count {
                            return false;
                        }
                    }
                    TileKind::Stacker => needed_stackers += tile_count,
                    TileKind::StackerSmall => needed_small_stackers += tile_count,
                    TileKind::SwitchLeft => {
                        needed_switch_inserts += 1;
                        needed_two_ways += 1;
                    }
                    TileKind::SwitchRight => {
                        needed_switch_inserts += 1;
                        needed_two_ways += 1;
                    }
                    TileKind::TwoWay => needed_two_ways += 1,
                    TileKind::StraightTunnel => {
                        if self
                            .elements
                            .get(&SetContentElement::StraightTunnel)
                            .unwrap_or(&0)
                            < tile_count
                        {
                            return false;
                        }
                        if self
                            .elements
                            .get(&SetContentElement::BasicStraight)
                            .unwrap_or(&0)
                            < tile_count
                        {
                            return false;
                        }
                    }
                    TileKind::CurveTunnel => {
                        needed_basic_tiles += 1;
                        if self
                            .elements
                            .get(&SetContentElement::CurveTunnel)
                            .unwrap_or(&0)
                            < tile_count
                        {
                            return false;
                        }
                    }
                    TileKind::SwitchTunnel => {
                        needed_basic_tiles += 1;
                        if self
                            .elements
                            .get(&SetContentElement::SwitchTunnel)
                            .unwrap_or(&0)
                            < tile_count
                        {
                            return false;
                        }
                    }
                    TileKind::Trampolin0 => needed_trampolines += 1,
                    TileKind::Trampolin1 => {
                        needed_trampolines += 1;
                        needed_stackers_angled += 1;
                    }
                    TileKind::Trampolin2 => {
                        needed_trampolines += 1;
                        needed_stackers_angled += 2;
                    }
                    TileKind::LiftSmall => {
                        needed_lift_entrance += 1;
                        needed_lift_exit += 1;
                        needed_lift_heighttube += 1;
                    }
                    TileKind::LiftLarge => {
                        needed_lift_entrance += 1;
                        needed_lift_exit += 1;
                        needed_lift_heighttube += 2;
                    }
                    TileKind::ZiplineStart => {
                        if self.elements.get(&SetContentElement::Zipline).unwrap_or(&0) < tile_count
                        {
                            return false;
                        }
                    }
                    TileKind::ZiplineEnd => {}
                    TileKind::ScrewSmall => {
                        needed_screw_base += 1;
                        needed_screw_entrance += 1
                    }
                    TileKind::ScrewMedium => {
                        needed_screw_entrance += 1;
                        needed_screw_base += 1;
                        needed_screw_curve += 5;
                    }
                    TileKind::ScrewLarge => {
                        needed_screw_entrance += 1;
                        needed_screw_base += 1;
                        needed_screw_curve += 5;
                    }
                    TileKind::MixerOffsetExits => {}
                    TileKind::StackerTowerClosed => needed_stacker_towers_closed += 1,
                    TileKind::StackerTowerOpened => needed_stacker_towers_opened += 1,
                    TileKind::MixerSameExits => {}
                    TileKind::DipperLeft => {}
                    TileKind::DipperRight => {}
                    TileKind::ColorSwapEmpty => {}
                    TileKind::ColorSwapPreloaded => {}
                    TileKind::CarouselSameExits => {}
                    TileKind::CarouselOffsetExits => {}
                    TileKind::DropdownSwitchLeft => {}
                    TileKind::DropdownSwitchRight => {}
                    _ => unreachable!(),
                },
                Some(element) => {
                    if self.elements.get(&element).unwrap_or(&0) < tile_count {
                        return false;
                    }
                }
            }
        }

        // Open & Closed Basic Tiles
        // We definitely need all open tiles, but then we can add the remaining open tiles to all our closed tiles to check if we have enough
        let mut inventory_open_basic = self.elements.get(&BasicOpen).unwrap_or(&0).clone();
        if inventory_open_basic < needed_basic_open_tiles {
            return false;
        }

        // Here we can add the remaining open tiles to the closed tiles and check them against how many basic tiles are needed
        let inventory_basic = self.elements.get(&BasicClosed).unwrap_or(&0).clone()
            + (inventory_open_basic - needed_basic_open_tiles);
        if inventory_basic < needed_basic_tiles {
            return false;
        }

        // Angled stackers
        if self.count_of(&SetContentElement::StackerAngled) < needed_stackers_angled {
            return false;
        }

        // Stackers - Small & Normal & Towers
        // Open towers & small stackers cannot be replaced by something else so we check them first
        // TODO: Later a stacker tower might be replaced by stackers IF and only if the tower does not have walls
        let mut inventory_small_stacker = self.count_of(&SetContentElement::StackerSmall);
        if inventory_small_stacker < needed_small_stackers {
            return false;
        }
        inventory_small_stacker -= needed_small_stackers;

        if self.count_of(&SetContentElement::Stacker) + (inventory_small_stacker / 2)
            < needed_stackers
        {
            return false;
        }

        let mut inventory_stacker_tower_opened =
            self.count_of(&SetContentElement::StackerTowerOpened);
        if inventory_stacker_tower_opened < needed_stacker_towers_opened {
            return false;
        }
        inventory_stacker_tower_opened -= needed_stacker_towers_opened;
        if self.count_of(&SetContentElement::StackerTowerClosed) + inventory_stacker_tower_opened
            < needed_stacker_towers_closed
        {
            return false;
        }

        // Now the switches, we couldn't do them earlier because the App splits them in two (left & right)
        if self.count_of(&SetContentElement::TwoWay) < needed_two_ways {
            return false;
        }
        if self.count_of(&SetContentElement::SwitchInsert) < needed_switch_inserts {
            return false;
        }

        // Trampolines are also split, check here
        if self.count_of(&SetContentElement::Trampoline) < needed_trampolines {
            return false;
        }

        if self.count_of(&SetContentElement::LiftEntrance) < needed_lift_entrance {
            return false;
        }
        if self.count_of(&SetContentElement::LiftExit) < needed_lift_exit {
            return false;
        }
        if self.count_of(&SetContentElement::LiftHeightTube) < needed_lift_heighttube {
            return false;
        }

        if self.count_of(&SetContentElement::SpiralBase) < needed_screw_base {
            return false;
        }
        if self.count_of(&SetContentElement::SpiralEntrance) < needed_screw_entrance {
            return false;
        }
        if self.count_of(&SetContentElement::SpiralCurve) < needed_screw_curve {
            return false;
        }

        true
    }

    fn check_basic_tiles(&self, bom: &AppBillOfMaterials) {
        let catches_required = bom.tile_kind(TileKind::Catch);
        let catches_available = self.count_of(SetContentElement::Catch);

        /*
        TileKind::Catch => {
            needed_basic_tiles += 1;
            if self.elements.get(&SetContentElement::Catch).unwrap_or(&0) < tile_count {
                return false;
            }
        }
        TileKind::Splash => {
            needed_basic_tiles += 1;
            if self.elements.get(&SetContentElement::Splash).unwrap_or(&0) < tile_count {
                return false;
            }
        }
        TileKind::GoalBasin => {
            needed_basic_tiles += 1;
            if self.elements.get(&SetContentElement::GoalBasin).unwrap_or(&0) < tile_count {
                return false;
            }
        }
        TileKind::Drop => {
            needed_basic_open_tiles += 1;
            if self.elements.get(&SetContentElement::Drop).unwrap_or(&0) < tile_count {
                return false;
            }
        }

         */
    }



    pub fn count_of(&self, element: Element) -> i32 {
        *self.elements.get(&element).unwrap_or(&0)
    }

     */
}