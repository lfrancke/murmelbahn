use crate::course::common;
use crate::course::common::layer;
use crate::course::common::layer::TileKind;
use crate::error::MurmelbahnError;
use crate::error::MurmelbahnError::UnsupportedPiece;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize)]
pub enum LayerKind {
    Base,
    SmallClear,
    LargeClear,
}

impl TryFrom<layer::LayerKind> for LayerKind {
    type Error = MurmelbahnError;

    fn try_from(value: layer::LayerKind) -> Result<Self, Self::Error> {
        Ok(match value {
            layer::LayerKind::BaselayerPiece => LayerKind::Base,
            layer::LayerKind::LargeLayer => LayerKind::LargeClear,
            layer::LayerKind::SmallLayer => LayerKind::SmallClear,
            _ => return Err(UnsupportedPiece),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize)]
pub enum RailKind {
    StraightSmall,
    StraightMedium,
    StraightLarge,
    Bernoulli,
    DropHill,
    DropValley,
    UTurn,
    Narrow,
    Slow,
    BernoulliSmallStraight,
    BernoulliSmallLeft,
    BernoulliSmallRight,
    FlexTube0,
    FlexTube60,
    FlexTube120,
    FlexTube180,
    FlexTube240,
    FlexTube300,
}

impl TryFrom<common::rail::RailKind> for RailKind {
    type Error = MurmelbahnError;

    fn try_from(value: common::rail::RailKind) -> Result<Self, Self::Error> {
        let value_new = match value {
            common::rail::RailKind::Bernoulli => RailKind::Bernoulli,
            common::rail::RailKind::DropHill => RailKind::DropHill,
            common::rail::RailKind::DropValley => RailKind::DropValley,
            common::rail::RailKind::UTurn => RailKind::UTurn,
            common::rail::RailKind::Narrow => RailKind::Narrow,
            common::rail::RailKind::Slow => RailKind::Slow,
            common::rail::RailKind::BernoulliSmallStraight => RailKind::BernoulliSmallStraight,
            common::rail::RailKind::BernoulliSmallLeft => RailKind::BernoulliSmallLeft,
            common::rail::RailKind::BernoulliSmallRight => RailKind::BernoulliSmallRight,
            common::rail::RailKind::FlexTube0 => RailKind::FlexTube0,
            common::rail::RailKind::FlexTube60 => RailKind::FlexTube60,
            common::rail::RailKind::FlexTube120 => RailKind::FlexTube120,
            common::rail::RailKind::FlexTube180 => RailKind::FlexTube180,
            common::rail::RailKind::FlexTube240 => RailKind::FlexTube240,
            common::rail::RailKind::FlexTube300 => RailKind::FlexTube300,
            common::rail::RailKind::Straight => return Err(UnsupportedPiece),
        };

        Ok(value_new)
    }
}

// TODO: Marbles
#[derive(Debug, Default, Deserialize, JsonSchema, Serialize)]
pub struct BillOfMaterial {
    pub layers: HashMap<LayerKind, i32>,
    pub tiles: HashMap<TileKind, i32>, // TODO: This is still the wrong one, we want one that includes stackers and balconies etc.
    pub small_stacker: i32,
    pub large_stacker: i32,
    pub rails: HashMap<RailKind, i32>,
    pub walls: HashMap<WallKind, i32>,
    pub balconies: usize,
}

impl BillOfMaterial {
    pub fn layer_kind(&self, kind: LayerKind) -> Option<i32> {
        self.layers.get(&kind).copied()
    }

    pub fn tile_kind(&self, kind: TileKind) -> Option<i32> {
        self.tiles.get(&kind).copied()
    }

    pub fn wall_kind(&self, kind: WallKind) -> Option<i32> {
        self.walls.get(&kind).copied()
    }

    pub fn rail_kind(&self, kind: RailKind) -> Option<i32> {
        self.rails.get(&kind).copied()
    }

    pub fn marbles(&self) -> (i32, i32) {
        let zipline = self.tile_kind(TileKind::ZiplineStart);
        let color_change = self.tile_kind(TileKind::ColorSwapPreloaded);
        let cannon = self.tile_kind(TileKind::Cannon);
        let bridge = self.tile_kind(TileKind::Bridge);
        let catapult = self.tile_kind(TileKind::Catapult);
        let dome_starter = self.tile_kind(TileKind::DomeStarter);
        let starter = self.tile_kind(TileKind::Starter);
        let lift_small = self.tile_kind(TileKind::LiftSmall);
        let lift_large = self.tile_kind(TileKind::LiftLarge);
        // TODO: Tiptube?

        // TODO: To get better number we should check how many rails/adjacent tiles there are
        // for this next group
        let splash = self.tile_kind(TileKind::Splash);
        let volcano = self.tile_kind(TileKind::Volcano);
        let spinner = self.tile_kind(TileKind::Spinner);

        /*
                let min_marbles = cannon * 2
                    + zipline
                    + color_change
                    + bridge * 2
                    + catapult * 4
                    + splash
                    + volcano
                    + spinner
                    + dome_starter
                    + starter
                    + lift_small * 5
                    + lift_large * 8;


                let max_marbles = cannon * 2
                    + zipline
                    + color_change
                    + bridge * 2
                    + catapult * 4
                    + splash * 3
                    + volcano * 3
                    + spinner * 6
                    + dome_starter * 7
                    + starter * 3
                    + lift_small * 5
                    + lift_large * 8;
        */
        (0, 0)
    }
}

/*
impl BillOfMaterial {
    pub fn add(&mut self, other: &BillOfMaterial) {
        merge_counter_maps(&mut self.layers, &other.layers);
        merge_counter_maps(&mut self.tiles, &other.tiles);
        merge_counter_maps(&mut self.rails, &other.rails);
    }

    pub fn can_be_built_with(&self, other: &BillOfMaterial) -> bool {
        is_superset_of_and_has_higher_counts(&self.layers, &other.layers)
            && is_superset_of_and_has_higher_counts(&self.tiles, &other.tiles)
            && is_superset_of_and_has_higher_counts(&self.rails, &other.rails)
    }
}

fn is_superset_of_and_has_higher_counts<T: Hash + Eq>(
    map_1: &HashMap<T, i32>,
    map_2: &HashMap<T, i32>,
) -> bool {
    //iterate over the items of map1
    for (key, value) in map_1.iter() {
        // check if the key exists in map2 and its value is greater or equal to map1
        if !map_2.contains_key(key) || *map_2.get(key).unwrap() < *value {
            return false;
        }
    }
    true
}

fn merge_counter_maps<T: Clone + Hash + Eq>(first: &mut HashMap<T, i32>, other: &HashMap<T, i32>) {
    for (key, value) in other {
        let entry = first.entry(key.clone()).or_insert(0);
        *entry += value
    }
}


 */
