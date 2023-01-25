use crate::course::common::{CourseSaveDataVersion, HexVector};
use deku::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, DekuRead, Hash, JsonSchema, PartialEq, Serialize)]
#[deku(type = "u32")]
pub enum LayerKind {
    BaselayerPiece = 0,
    Baselayer = 1,
    LargeLayer = 2,
    LargeGhostLayer = 3,
    SmallLayer = 4,
}

#[derive(Clone, Debug, Deserialize, Eq, DekuRead, Hash, JsonSchema, PartialEq, Serialize)]
#[deku(type = "u32")]
pub enum TileKind {
    None = 0,
    Starter = 1,
    Curve = 2,
    Catch = 3,
    GoalBasin = 4,
    Drop = 5,
    Hammer = 6,
    Catapult = 7,
    // Junction
    Cross = 8,
    Threeway = 9,
    TwoWay = 10,
    Spiral = 11,
    Splash = 12,
    Loop = 13,
    Cannon = 14,
    Stacker = 15,
    StackerSmall = 16,
    SwitchLeft = 17,
    SwitchRight = 18,
    GoalRail = 19,
    StackerBatch = 20,
    Cascade = 21,
    StraightTunnel = 22,
    CurveTunnel = 23,
    SwitchTunnel = 24,
    Trampolin0 = 25,
    Trampolin1 = 26,
    Trampolin2 = 27,
    LiftSmall = 28,
    LiftLarge = 29,
    Flip = 30,
    TipTube = 31,
    Volcano = 32,
    Jumper = 33,
    Transfer = 34,
    ZiplineStart = 35,
    ZiplineEnd = 36,
    Bridge = 37,
    ScrewSmall = 38,
    ScrewMedium = 39,
    ScrewLarge = 40,
    MixerOffsetExits = 41,
    Splitter = 42,
    StackerTowerClosed = 43,
    StackerTowerOpened = 44,
    DoubleBalcony = 45,
    MixerSameExits = 46,
    DipperLeft = 47,
    DipperRight = 48,
    Helix = 49,
    Turntable = 50,
    Spinner = 51,
    TwoInOneSmallCurveA = 52,
    TwoInOneSmallCurveB = 53,
    FlexibleTwoInOneB = 54,
    RibbonCurve = 55,
    ThreeEntranceFunnel = 56,
    CurveCrossing = 57,
    DoubleBigCurve = 58,
    DoubleSmallCurve = 59,
    MultiJunction = 60,
    StraightCurveCrossing = 61,
    TripleSmallCurve = 62,
    FlexibleTwoInOneA = 63,
    ColorSwapEmpty = 64,
    ColorSwapPreloaded = 65,
    CarouselSameExits = 66,
    CarouselOffsetExits = 67,
    DomeStarter = 68,
    FinishTrigger = 69,
    FinishArena = 70,
    Trigger = 71,
    DropdownSwitchLeft = 72,
    DropdownSwitchRight = 73,
    Queue = 74,
    Lever = 75,
    Elevator = 77,
}

#[derive(Debug, DekuRead, Serialize)]
#[deku(type = "u32")]
pub enum PowerSignalMode {
    Off = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
    Automatic = 4,
}

#[deku_derive(DekuRead)]
#[derive(Debug, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct LayerConstructionData {
    pub layer_id: i32,
    pub layer_kind: LayerKind,
    /// This is in multiples of 0.36 and because it's a float it's not exact.
    /// -0.2 is the layer height for all base plates
    /// 0.36 is one small stacker, 0.72 is two small ones or one large one etc.
    pub layer_height: f32,

    /// This is the absolute position of this layer in the world
    /// For baselayers this is the position of the one green cell in the corner (there is only one)
    /// For the clear layers it is the cell in the middle
    /// This position is also the reference point (0/0) for the `local_hex_positions` from the `CellConstructionData`.
    pub world_hex_position: HexVector,

    #[deku(temp)]
    cell_construction_datas_size: i32,

    #[deku(count = "cell_construction_datas_size")]
    #[deku(ctx = "version")]
    pub cell_construction_datas: Vec<CellConstructionData>,
}

#[derive(Debug, DekuRead, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct CellConstructionData {
    /// This position is relative to the 0/0 position of the current layer
    /// To make it absolute those two need to be added together
    pub local_hex_position: HexVector,

    #[deku(ctx = "version")]
    pub tree_node_data: TileTowerTreeNodeData,
}

#[deku_derive(DekuRead)]
#[derive(Debug, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct TileTowerTreeNodeData {
    pub index: i32,

    #[deku(temp)]
    pub children_count: i32,

    #[deku(ctx = "version")]
    pub construction_data: TileTowerConstructionData,

    #[deku(count = "children_count")]
    #[deku(ctx = "version")]
    pub children: Vec<TileTowerTreeNodeData>,
}

#[derive(Debug, DekuRead, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct TileTowerConstructionData {
    pub kind: TileKind,
    pub height_in_small_stacker: i32,
    pub hex_rotation: i32,
    #[deku(
        map = "|field: i32| -> Result<_, DekuError> { if field != -2147483647 { Ok(Some(field)) } else { Ok(None) } }"
    )]
    pub retainer_id: Option<i32>,

    #[deku(cond = "version == CourseSaveDataVersion::Power2022", default = "None")]
    #[deku(
        map = "|field: u32| -> Result<_, DekuError> { if field != 2147483648 { Ok(Some(PowerSignalMode::try_from(&field.to_le_bytes().to_vec()[..]).unwrap())) } else { Ok(None) } }"
    )] // This is horrible!
    pub power_signal_mode: Option<PowerSignalMode>,
}
