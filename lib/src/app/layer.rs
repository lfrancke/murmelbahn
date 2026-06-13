use deku::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::app::course::CourseSaveDataVersion;
use crate::app::course::HexVector;

#[derive(Clone, Debug, Deserialize, Eq, DekuRead, Hash, JsonSchema, PartialEq, Serialize)]
#[deku(id_type = "u32")]
pub enum LayerKind {
    #[deku(id = "0")]
    BaseLayerPiece,
    #[deku(id = "1")]
    BaseLayer,
    #[deku(id = "2")]
    LargeLayer,
    #[deku(id = "3")]
    LargeGhostLayer,
    #[deku(id = "4")]
    SmallLayer,

    /// A discriminant this parser version does not define, for example one
    /// introduced by a newer app release. Keeping the raw value consumes the
    /// fixed four-byte tag so the rest of the course still parses.
    #[deku(id_pat = "_")]
    Unknown(u32),
}

#[derive(Clone, Debug, Deserialize, Eq, DekuRead, Hash, JsonSchema, PartialEq, Serialize)]
#[deku(id_type = "u32")]
pub enum TileKind {
    #[deku(id = "0")]
    None,
    #[deku(id = "1")]
    Starter,
    #[deku(id = "2")]
    Curve,
    #[deku(id = "3")]
    Catch,
    #[deku(id = "4")]
    GoalBasin,
    #[deku(id = "5")]
    Drop,
    #[deku(id = "6")]
    Hammer,
    #[deku(id = "7")]
    Catapult,
    #[deku(id = "8")]
    Cross,
    #[deku(id = "9")]
    Threeway,
    #[deku(id = "10")]
    TwoWay,
    #[deku(id = "11")]
    Spiral,
    #[deku(id = "12")]
    Splash,
    #[deku(id = "13")]
    Loop,
    #[deku(id = "14")]
    Cannon,
    #[deku(id = "15")]
    Stacker,
    #[deku(id = "16")]
    StackerSmall,
    #[deku(id = "17")]
    SwitchLeft,
    #[deku(id = "18")]
    SwitchRight,
    #[deku(id = "19")]
    GoalRail,
    #[deku(id = "20")]
    StackerBatch,
    #[deku(id = "21")]
    Cascade,
    #[deku(id = "22")]
    StraightTunnel,
    #[deku(id = "23")]
    CurveTunnel,
    #[deku(id = "24")]
    SwitchTunnel,
    #[deku(id = "25")]
    Trampolin0,
    #[deku(id = "26")]
    Trampolin1,
    #[deku(id = "27")]
    Trampolin2,
    #[deku(id = "28")]
    LiftSmall,
    #[deku(id = "29")]
    LiftLarge,
    #[deku(id = "30")]
    Flip,
    #[deku(id = "31")]
    TipTube,
    #[deku(id = "32")]
    Volcano,
    #[deku(id = "33")]
    Jumper,
    #[deku(id = "34")]
    Transfer,
    #[deku(id = "35")]
    ZiplineStart,
    #[deku(id = "36")]
    ZiplineEnd,
    #[deku(id = "37")]
    Bridge,
    #[deku(id = "38")]
    ScrewSmall,
    #[deku(id = "39")]
    ScrewMedium,
    #[deku(id = "40")]
    ScrewLarge,
    #[deku(id = "41")]
    MixerOffsetExits,
    #[deku(id = "42")]
    Splitter,
    #[deku(id = "43")]
    StackerTowerClosed,
    #[deku(id = "44")]
    StackerTowerOpened,
    #[deku(id = "45")]
    DoubleBalcony,
    #[deku(id = "46")]
    MixerSameExits,
    #[deku(id = "47")]
    DipperLeft,
    #[deku(id = "48")]
    DipperRight,
    #[deku(id = "49")]
    Helix,
    #[deku(id = "50")]
    Turntable,
    #[deku(id = "51")]
    Spinner,
    #[deku(id = "52")]
    TwoInOneSmallCurveA,
    #[deku(id = "53")]
    TwoInOneSmallCurveB,
    #[deku(id = "54")]
    FlexibleTwoInOneB,
    #[deku(id = "55")]
    RibbonCurve,
    #[deku(id = "56")]
    ThreeEntranceFunnel,
    #[deku(id = "57")]
    CurveCrossing,
    #[deku(id = "58")]
    DoubleBigCurve,
    #[deku(id = "59")]
    DoubleSmallCurve,
    #[deku(id = "60")]
    MultiJunction,
    #[deku(id = "61")]
    StraightCurveCrossing,
    #[deku(id = "62")]
    TripleSmallCurve,
    #[deku(id = "63")]
    FlexibleTwoInOneA,
    #[deku(id = "64")]
    ColorSwapEmpty,
    #[deku(id = "65")]
    ColorSwapPreloaded,
    #[deku(id = "66")]
    CarouselSameExits,
    #[deku(id = "67")]
    CarouselOffsetExits,
    #[deku(id = "68")]
    DomeStarter,
    #[deku(id = "69")]
    FinishTrigger,
    #[deku(id = "70")]
    FinishArena,
    #[deku(id = "71")]
    Trigger,
    #[deku(id = "72")]
    DropdownSwitchLeft,
    #[deku(id = "73")]
    DropdownSwitchRight,
    #[deku(id = "74")]
    Queue,
    #[deku(id = "75")]
    Lever,
    #[deku(id = "77")]
    Elevator,
    #[deku(id = "78")]
    LightBase,
    #[deku(id = "79")]
    LightStacker,
    #[deku(id = "80")]
    LightStackerSmall,
    #[deku(id = "81")]
    LightStackerBatch,
    #[deku(id = "82")]
    Releaser1,
    #[deku(id = "83")]
    Releaser2,
    #[deku(id = "84")]
    Releaser3,
    #[deku(id = "85")]
    Releaser4,
    #[deku(id = "86")]
    VerticalCannon0,
    #[deku(id = "87")]
    VerticalCannon60,
    #[deku(id = "88")]
    VerticalCannon120,
    #[deku(id = "89")]
    VerticalCannon180,
    #[deku(id = "90")]
    VerticalCannon240,
    #[deku(id = "91")]
    VerticalCannon300,
    #[deku(id = "92")]
    SpaceTubeAligned,
    #[deku(id = "93")]
    SpaceTubeUnaligned,
    #[deku(id = "94")]
    ElectricCannon,
    #[deku(id = "95")]
    K2In1Slope,
    #[deku(id = "96")]
    K3In1Slope,
    #[deku(id = "97")]
    K120DoubleCurveSlope,
    #[deku(id = "98")]
    KBoomerangSlope,
    #[deku(id = "99")]
    KCrossingSlope,
    #[deku(id = "100")]
    KCurveSlope1,
    #[deku(id = "101")]
    KCurveSlope2,
    #[deku(id = "102")]
    KJumpCrossingSlope,
    #[deku(id = "103")]
    Kst2In1L,
    #[deku(id = "104")]
    Kst2In1R,
    #[deku(id = "105")]
    Kst120CatchDrop60L,
    #[deku(id = "106")]
    Kst120CatchDrop60R,
    #[deku(id = "107")]
    Kst180Catch6060,
    #[deku(id = "108")]
    KstCrossingCatchDrop,
    #[deku(id = "109")]
    KstCurveCatch,
    #[deku(id = "110")]
    KstCurveDrop,
    #[deku(id = "111")]
    KstFinish,
    #[deku(id = "112")]
    KstGtDrop,
    #[deku(id = "113")]
    KstHs5,
    #[deku(id = "114")]
    KstHs20,
    #[deku(id = "115")]
    KstMultiCatchDrop,
    #[deku(id = "116")]
    KstMultiCatcher,
    #[deku(id = "117")]
    KstSpiral120CatchDropCatchL,
    #[deku(id = "118")]
    KstSpiral120CatchDropCatchR,
    #[deku(id = "119")]
    KstSpiral180CatchDropL,
    #[deku(id = "120")]
    KstSpiral180CatchDropR,
    #[deku(id = "121")]
    KstSpiral240CatchL,
    #[deku(id = "122")]
    KstSpiral240CatchR,
    #[deku(id = "123")]
    KstSpiral300L,
    #[deku(id = "124")]
    KstSpiral300R,
    #[deku(id = "125")]
    KstStarter,
    #[deku(id = "126")]
    Kst3In1,

    /// A discriminant this parser version does not define, for example one
    /// introduced by a newer app release. Keeping the raw value consumes the
    /// fixed four-byte tag so the rest of the course still parses.
    #[deku(id_pat = "_")]
    Unknown(u32),
}

#[derive(Debug, DekuRead, Serialize)]
#[deku(id_type = "u32")]
pub enum PowerSignalMode {
    Off = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
    Automatic = 4,
}

#[derive(Debug, DekuRead, Serialize)]
#[deku(id_type = "u32")]
pub enum LightStoneColorMode {
    Off = 0,
    Alternating = 1,
    Red = 2,
    Green = 3,
    Blue = 4,
    White = 5,
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
    /// For the (hexagonal) clear layers it is the cell in the middle
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
    /// To make it absolute this needs to be added to the `world_hex_position` of the layer
    pub local_hex_position: HexVector,

    #[deku(ctx = "version")]
    pub tree_node_data: TileTowerTreeNodeData,
}

impl CellConstructionData {
    pub fn world_hex_position(&self, layer: &LayerConstructionData) -> HexVector {
        self.local_hex_position.add(&layer.world_hex_position)
    }
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
    #[deku(map = "TileTowerConstructionData::map_retainer_id")]
    pub retainer_id: Option<i32>,

    #[deku(
        cond = "version == CourseSaveDataVersion::Power2022 || version == CourseSaveDataVersion::LightStones2023 || version == CourseSaveDataVersion::SkyTrax",
        default = "None"
    )]
    #[deku(map = "TileTowerConstructionData::map_power_signal_mode")]
    pub power_signal_mode: Option<PowerSignalMode>,

    #[deku(
        cond = "version == CourseSaveDataVersion::LightStones2023 || version == CourseSaveDataVersion::SkyTrax",
        default = "None"
    )]
    #[deku(map = "TileTowerConstructionData::map_light_stone_color_mode")]
    pub light_stone_color_mode: Option<LightStoneColorMode>,
}

impl TileTowerConstructionData {
    fn map_retainer_id(field: i32) -> Result<Option<i32>, DekuError> {
        if field == -2147483647 {
            Ok(None)
        } else {
            Ok(Some(field))
        }
    }

    fn map_power_signal_mode(field: u32) -> Result<Option<PowerSignalMode>, DekuError> {
        if field == 2147483648 {
            Ok(None)
        } else {
            let input = field.to_le_bytes();
            let input2 = input.as_slice();
            Ok(Some(PowerSignalMode::try_from(input2)?))
        }
    }

    fn map_light_stone_color_mode(field: u32) -> Result<Option<LightStoneColorMode>, DekuError> {
        if field == 2147483648 {
            Ok(None)
        } else {
            let input = field.to_le_bytes();
            let input2 = input.as_slice();
            Ok(Some(LightStoneColorMode::try_from(input2)?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An unrecognised `TileKind` discriminant (a value from a newer app
    /// release) parses and consumes its fixed four bytes, so the rest of the
    /// course still parses.
    #[test]
    fn unknown_tile_kind_value_parses_and_consumes_four_bytes() {
        // 1337 little-endian; not a known TileKind discriminant.
        let bytes = [0x39u8, 0x05, 0x00, 0x00];
        let ((rest, _bit), kind) =
            TileKind::from_bytes((&bytes, 0)).expect("unknown TileKind should parse");
        assert_eq!(kind, TileKind::Unknown(1337), "raw discriminant preserved");
        assert!(
            rest.is_empty(),
            "should consume exactly 4 bytes, {} byte(s) left",
            rest.len()
        );

        // A known discriminant must still parse to its named variant.
        let known = [0x01u8, 0x00, 0x00, 0x00];
        let (_, kind) = TileKind::from_bytes((&known, 0)).unwrap();
        assert_eq!(kind, TileKind::Starter);
    }

    /// Discriminants 94 to 126 are the SkyTrax tile kinds and parse to their
    /// named variants.
    #[test]
    fn skytrax_tile_kinds_parse_to_their_variant() {
        let bytes = 94u32.to_le_bytes();
        let (_, kind) = TileKind::from_bytes((&bytes, 0)).unwrap();
        assert_eq!(kind, TileKind::ElectricCannon);

        let bytes = 126u32.to_le_bytes();
        let (_, kind) = TileKind::from_bytes((&bytes, 0)).unwrap();
        assert_eq!(kind, TileKind::Kst3In1);
    }
}
