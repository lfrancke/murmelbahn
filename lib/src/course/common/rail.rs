use crate::course::common::HexVector;
use deku::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, DekuRead, Hash, JsonSchema, PartialEq, Serialize)]
#[deku(type = "u32")]
pub enum RailKind {
    Straight = 0,
    Bernoulli = 1,
    DropHill = 3,
    DropValley = 4,
    UTurn = 5,
    Narrow = 6,
    Slow = 7,
    BernoulliSmallStraight = 8,
    BernoulliSmallLeft = 9,
    BernoulliSmallRight = 10,
    FlexTube0 = 11,
    FlexTube60 = 12,
    FlexTube120 = 13,
    FlexTube180 = 14,
    FlexTube240 = 15,
    FlexTube300 = 16,
}

#[derive(Debug, DekuRead, Serialize)]
pub struct RailConstructionExitIdentifier {
    pub retainer_id: i32,
    pub cell_local_hex_pos: HexVector,
    pub side_hex_rot: i32,
    pub exit_local_pos_y: f32,
}

#[derive(Debug, DekuRead, Serialize)]
pub struct RailConstructionData {
    pub exit_1_identifier: RailConstructionExitIdentifier,
    pub exit_2_identifier: RailConstructionExitIdentifier,
    pub rail_kind: RailKind,
}
