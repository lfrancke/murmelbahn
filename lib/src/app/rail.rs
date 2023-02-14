use deku::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::app::course::{CourseSaveDataVersion, HexVector};

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
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct RailConstructionExitIdentifier {
    pub retainer_id: i32,
    pub cell_local_hex_pos: HexVector,
    pub side_hex_rot: i32,
    #[deku(
        cond = "version == CourseSaveDataVersion::Power2022 || version == CourseSaveDataVersion::Pro2020",
        default = "None"
    )]
    pub exit_local_pos_y: Option<f32>,
}

#[derive(Debug, DekuRead, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct RailConstructionData {
    #[deku(ctx = "version")]
    pub exit_1_identifier: RailConstructionExitIdentifier,
    #[deku(ctx = "version")]
    pub exit_2_identifier: RailConstructionExitIdentifier,
    pub rail_kind: RailKind,
    #[deku(
        cond = "version == CourseSaveDataVersion::ZiplineAdded2019",
        default = "None"
    )]
    pub materialized: Option<bool>,
}
