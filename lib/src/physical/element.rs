use crate::app::layer::{LayerKind, TileKind};
use crate::app::rail::RailKind;
use crate::app::wall::WallKind;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use ts_rs::TS;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to deserialize course"))]
    UnknownElement,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize, TS)]
#[ts(export)]
pub enum Element {
    // Layer
    BaseLayer,
    SmallClearLayer,
    LargeClearLayer,
    MiniBaseLayer,
    HalfMiniBaseLayer,

    // Marbles
    MarbleRed,
    MarbleGreen,
    MarbleBlue,
    MarbleSilver,
    MarbleGold,

    // Stacker
    Stacker,
    StackerSmall,
    StackerAngled,
    StackerTowerClosed,
    StackerTowerOpened,

    // Walls & Baconies
    WallSmall,
    WallMedium,
    WallLarge,
    Balcony,
    DoubleBalcony,

    // Rails
    Bernoulli,
    BernoulliSmallLeft,
    BernoulliSmallRight,
    BernoulliSmallStraight,
    Catcher,
    DropHill,
    DropValley,
    FlexTube,
    Narrow,
    Slow,
    StraightLarge,
    StraightMedium,
    StraightSmall,
    UTurn,

    // Standard Tiles
    Bridge,
    Cannon,
    Cascade, // Scoop in GraviSheet
    Catapult,
    ColorSwap,
    Cross,
    Curve,
    CurveCrossing,
    Dipper,
    DoubleBigCurve,
    DoubleSmallCurve,
    FlexibleTwoInOneA,
    FlexibleTwoInOneB,
    Flip,
    GoalRail,
    Hammer,
    Jumper,
    LiftEntrance,
    LiftHeightTube,
    LiftExit,
    Loop,
    MultiJunction,
    RibbonCurve,
    Spinner,
    // Called Screw internally
    SpiralBase,
    SpiralCurve,
    SpiralEntrance,
    Starter,
    StraightCurveCrossing,
    SwitchInsert,
    ThreeEntranceFunnel,
    ThreeWay,
    TipTube,
    Trampoline,
    Transfer,
    TripleSmallCurve,
    // Called Spiral internally
    TwoEntranceFunnel,
    TwoInOneSmallCurveA,
    TwoInOneSmallCurveB,
    TwoWay,
    Volcano,
    Zipline,

    // Basic tiles
    BasicClosed,
    BasicOpen,
    BasicStraight,

    // Inserts
    GoalBasin,
    Catch,
    Drop,
    Splash,
    StraightTunnel,
    CurveTunnel,
    SwitchTunnel,

    // Pro
    Carousel,
    Helix,
    Mixer,
    Splitter,
    Turntable,

    // Power
    Controller,
    DomeStarter,
    Elevator,
    Lever,
    DropdownSwitch,
    FinishTrigger,
    FinishArena,
    Trigger,
    Queue,
}

impl Element {
    pub fn elements_for_tilekind(tile_kind: &TileKind) -> Vec<Element> {
        match tile_kind {
            TileKind::Starter => vec![Element::Starter],
            TileKind::Curve => vec![Element::Curve],
            TileKind::Hammer => vec![Element::Hammer],
            TileKind::Catapult => vec![Element::Catapult],
            TileKind::Cross => vec![Element::Cross],
            TileKind::Threeway => vec![Element::ThreeWay],
            TileKind::Spiral => vec![Element::TwoEntranceFunnel],
            TileKind::Loop => vec![Element::Loop],
            TileKind::Cannon => vec![Element::Cannon],
            TileKind::GoalRail => vec![Element::GoalRail],
            TileKind::Cascade => vec![Element::Cascade],
            TileKind::Flip => vec![Element::Flip],
            TileKind::TipTube => vec![Element::TipTube],
            TileKind::Volcano => vec![Element::Volcano],
            TileKind::Jumper => vec![Element::Jumper],
            TileKind::Transfer => vec![Element::Transfer],
            TileKind::Bridge => vec![Element::Bridge],
            TileKind::Splitter => vec![Element::Splitter],
            TileKind::DoubleBalcony => vec![Element::DoubleBalcony],
            TileKind::Helix => vec![Element::Helix],
            TileKind::Turntable => vec![Element::Turntable],
            TileKind::Spinner => vec![Element::Spinner],
            TileKind::TwoInOneSmallCurveA => vec![Element::TwoInOneSmallCurveA],
            TileKind::TwoInOneSmallCurveB => vec![Element::TwoInOneSmallCurveB],
            TileKind::FlexibleTwoInOneB => vec![Element::FlexibleTwoInOneB],
            TileKind::RibbonCurve => vec![Element::RibbonCurve],
            TileKind::ThreeEntranceFunnel => vec![Element::ThreeEntranceFunnel],
            TileKind::CurveCrossing => vec![Element::CurveCrossing],
            TileKind::DoubleBigCurve => vec![Element::DoubleBigCurve],
            TileKind::DoubleSmallCurve => vec![Element::DoubleSmallCurve],
            TileKind::MultiJunction => vec![Element::MultiJunction],
            TileKind::StraightCurveCrossing => vec![Element::StraightCurveCrossing],
            TileKind::TripleSmallCurve => vec![Element::TripleSmallCurve],
            TileKind::FlexibleTwoInOneA => vec![Element::FlexibleTwoInOneA],
            TileKind::DomeStarter => vec![Element::DomeStarter],
            TileKind::FinishTrigger => vec![Element::FinishTrigger],
            TileKind::FinishArena => vec![Element::FinishArena],
            TileKind::Trigger => vec![Element::Trigger],
            TileKind::Queue => vec![Element::Queue],
            TileKind::Lever => vec![Element::Lever],
            TileKind::Elevator => vec![Element::Elevator],
            TileKind::Catch => vec![Element::Catch, Element::BasicClosed],
            TileKind::GoalBasin => {
                vec![Element::GoalBasin, Element::BasicClosed]
            }
            TileKind::Drop => vec![Element::Drop, Element::BasicOpen],
            TileKind::TwoWay => vec![Element::TwoWay],
            TileKind::Splash => vec![Element::Splash],
            TileKind::Stacker => vec![Element::Stacker],
            TileKind::StackerSmall => vec![Element::StackerSmall],
            TileKind::SwitchLeft => {
                vec![Element::TwoWay, Element::SwitchInsert]
            }
            TileKind::SwitchRight => {
                vec![Element::TwoWay, Element::SwitchInsert]
            }
            TileKind::StackerBatch => vec![],
            TileKind::StraightTunnel => vec![Element::StraightTunnel, Element::BasicStraight],
            TileKind::CurveTunnel => vec![Element::CurveTunnel, Element::BasicClosed],
            TileKind::SwitchTunnel => vec![Element::SwitchTunnel, Element::BasicClosed],
            TileKind::Trampolin0 => vec![Element::Trampoline],
            TileKind::Trampolin1 => vec![Element::Trampoline, Element::StackerAngled],
            TileKind::Trampolin2 => vec![
                Element::Trampoline,
                Element::StackerAngled,
                Element::StackerAngled,
            ],
            TileKind::LiftSmall => vec![
                Element::LiftEntrance,
                Element::LiftExit,
                Element::LiftHeightTube,
            ],
            TileKind::LiftLarge => vec![
                Element::LiftEntrance,
                Element::LiftExit,
                Element::LiftHeightTube,
                Element::LiftHeightTube,
            ],
            TileKind::ZiplineStart => vec![Element::Zipline],
            TileKind::ZiplineEnd => vec![],
            TileKind::ScrewSmall => vec![Element::SpiralBase, Element::SpiralEntrance],
            TileKind::ScrewMedium => {
                let mut vec = vec![Element::SpiralBase, Element::SpiralEntrance];
                for _ in 0..5 {
                    vec.push(Element::SpiralCurve);
                }
                vec
            }
            TileKind::ScrewLarge => {
                let mut vec = vec![Element::SpiralBase, Element::SpiralEntrance];
                for _ in 0..12 {
                    vec.push(Element::SpiralCurve);
                }
                vec
            }
            TileKind::MixerOffsetExits => vec![Element::Mixer],
            TileKind::StackerTowerClosed => vec![Element::StackerTowerClosed],
            TileKind::StackerTowerOpened => vec![Element::StackerTowerOpened],
            TileKind::MixerSameExits => vec![Element::Mixer],
            TileKind::DipperLeft => vec![Element::Dipper],
            TileKind::DipperRight => vec![Element::Dipper],
            TileKind::ColorSwapEmpty => vec![Element::ColorSwap],
            TileKind::ColorSwapPreloaded => vec![Element::ColorSwap],
            TileKind::CarouselSameExits => vec![Element::Carousel],
            TileKind::CarouselOffsetExits => vec![Element::Carousel],
            TileKind::DropdownSwitchLeft => vec![Element::DropdownSwitch],
            TileKind::DropdownSwitchRight => vec![Element::DropdownSwitch],
            TileKind::None => Vec::new(),
        }
    }
}

impl TryFrom<&LayerKind> for Element {
    type Error = Error;

    fn try_from(value: &LayerKind) -> Result<Self, Self::Error> {
        Ok(match value {
            LayerKind::BaseLayerPiece => Element::BaseLayer,
            LayerKind::LargeLayer => Element::LargeClearLayer,
            LayerKind::SmallLayer => Element::SmallClearLayer,
            _ => panic!(""), // TODO error
        })
    }
}

impl From<&WallKind> for Element {
    fn from(value: &WallKind) -> Self {
        match value {
            WallKind::StraightSmall => Element::WallSmall,
            WallKind::StraightMedium => Element::WallMedium,
            WallKind::StraightLarge => Element::WallLarge,
        }
    }
}

impl TryFrom<&RailKind> for Element {
    type Error = Error;

    fn try_from(value: &RailKind) -> Result<Self, Self::Error> {
        Ok(match value {
            RailKind::Straight => panic!("trnt"), // TODO
            RailKind::Bernoulli => Element::Bernoulli,
            RailKind::DropHill => Element::DropHill,
            RailKind::DropValley => Element::DropValley,
            RailKind::UTurn => Element::UTurn,
            RailKind::Narrow => Element::Narrow,
            RailKind::Slow => Element::Slow,
            RailKind::BernoulliSmallStraight => Element::BernoulliSmallStraight,
            RailKind::BernoulliSmallLeft => Element::BernoulliSmallLeft,
            RailKind::BernoulliSmallRight => Element::BernoulliSmallRight,
            RailKind::FlexTube0 => Element::FlexTube,
            RailKind::FlexTube60 => Element::FlexTube,
            RailKind::FlexTube120 => Element::FlexTube,
            RailKind::FlexTube180 => Element::FlexTube,
            RailKind::FlexTube240 => Element::FlexTube,
            RailKind::FlexTube300 => Element::FlexTube,
        })
    }
}
