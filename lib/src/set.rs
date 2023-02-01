use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use tracing::info;
use ts_rs::TS;
use crate::bom::{LayerKind, RailKind, TileKind, WallKind};
use crate::error::{IoSnafu, MurmelbahnError, MurmelbahnResult, ReadSnafu, SerdeJsonSnafu};

#[derive(Clone, Serialize, Deserialize, JsonSchema, TS)]
#[ts(export)]
pub struct Name {
    pub language_code: String,
    pub name: String
}

#[derive(Clone, Serialize, Default, Deserialize, JsonSchema, TS)]
#[ts(export)]
pub struct Set {
    pub id: String,

    #[serde(default)]
    pub names: Vec<Name>,

    #[serde(default)]
    pub content: HashMap<SetContentElement, i32>
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize, TS)]
#[ts(export)]
pub enum SetContentElement {
    // Layer
    BaseLayer,
    SmallClearLayer,
    LargeClearLayer,
    MiniBasePlate,
    HalfMiniBasePlate,

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
    Controler,
    DomeStarter,
    Elevator,
    Lever,
    DropdownSwitch,
    FinishTrigger,
    FinishArena,
    Trigger,
    Queue,
}

impl SetContentElement {

    pub fn element_for_layerkind(layer_kind: &LayerKind) -> SetContentElement {
        match layer_kind {
            LayerKind::Base => SetContentElement::BaseLayer,
            LayerKind::SmallClear => SetContentElement::SmallClearLayer,
            LayerKind::LargeClear => SetContentElement::LargeClearLayer
        }
    }

    pub fn element_for_wallkind(wall_kind: &WallKind) -> SetContentElement {
        match wall_kind {
            WallKind::StraightSmall => SetContentElement::WallSmall,
            WallKind::StraightMedium => SetContentElement::WallMedium,
            WallKind::StraightLarge => SetContentElement::WallLarge
        }
    }

    pub fn element_for_railkind(rail_kind: &RailKind) -> SetContentElement {
        match rail_kind {
            RailKind::StraightSmall => SetContentElement::StraightSmall,
            RailKind::StraightMedium => SetContentElement::StraightMedium,
            RailKind::StraightLarge => SetContentElement::StraightLarge,
            RailKind::Bernoulli => SetContentElement::Bernoulli,
            RailKind::DropHill => SetContentElement::DropHill,
            RailKind::DropValley => SetContentElement::DropValley,
            RailKind::UTurn => SetContentElement::UTurn,
            RailKind::Narrow => SetContentElement::Narrow,
            RailKind::Slow => SetContentElement::Slow,
            RailKind::BernoulliSmallStraight => SetContentElement::BernoulliSmallStraight,
            RailKind::BernoulliSmallLeft => SetContentElement::BernoulliSmallLeft,
            RailKind::BernoulliSmallRight => SetContentElement::BernoulliSmallRight,
            RailKind::FlexTube0 => SetContentElement::FlexTube,
            RailKind::FlexTube60 => SetContentElement::FlexTube,
            RailKind::FlexTube120 => SetContentElement::FlexTube,
            RailKind::FlexTube180 => SetContentElement::FlexTube,
            RailKind::FlexTube240 => SetContentElement::FlexTube,
            RailKind::FlexTube300 => SetContentElement::FlexTube
        }
    }

    pub fn elements_for_tilekind(tile_kind: &TileKind) -> Vec<SetContentElement> {
        match tile_kind {
            TileKind::Starter => vec![SetContentElement::Starter],
            TileKind::Curve => vec![SetContentElement::Curve],
            TileKind::Hammer => vec![SetContentElement::Hammer],
            TileKind::Catapult => vec![SetContentElement::Catapult],
            TileKind::Cross => vec![SetContentElement::Cross],
            TileKind::Threeway => vec![SetContentElement::ThreeWay],
            TileKind::Spiral => vec![SetContentElement::TwoEntranceFunnel],
            TileKind::Loop => vec![SetContentElement::Loop],
            TileKind::Cannon => vec![SetContentElement::Cannon],
            TileKind::GoalRail => vec![SetContentElement::GoalRail],
            TileKind::Cascade => vec![SetContentElement::Cascade],
            TileKind::Flip => vec![SetContentElement::Flip],
            TileKind::TipTube => vec![SetContentElement::TipTube],
            TileKind::Volcano => vec![SetContentElement::Volcano],
            TileKind::Jumper => vec![SetContentElement::Jumper],
            TileKind::Transfer => vec![SetContentElement::Transfer],
            TileKind::Bridge => vec![SetContentElement::Bridge],
            TileKind::Splitter => vec![SetContentElement::Splitter],
            TileKind::Balcony => vec![SetContentElement::Balcony],
            TileKind::DoubleBalcony => vec![SetContentElement::DoubleBalcony],
            TileKind::Helix => vec![SetContentElement::Helix],
            TileKind::Turntable => vec![SetContentElement::Turntable],
            TileKind::Spinner => vec![SetContentElement::Spinner],
            TileKind::TwoInOneSmallCurveA => vec![SetContentElement::TwoInOneSmallCurveA],
            TileKind::TwoInOneSmallCurveB => vec![SetContentElement::TwoInOneSmallCurveB],
            TileKind::FlexibleTwoInOneB => vec![SetContentElement::FlexibleTwoInOneB],
            TileKind::RibbonCurve => vec![SetContentElement::RibbonCurve],
            TileKind::ThreeEntranceFunnel => vec![SetContentElement::ThreeEntranceFunnel],
            TileKind::CurveCrossing => vec![SetContentElement::CurveCrossing],
            TileKind::DoubleBigCurve => vec![SetContentElement::DoubleBigCurve],
            TileKind::DoubleSmallCurve => vec![SetContentElement::DoubleSmallCurve],
            TileKind::MultiJunction => vec![SetContentElement::MultiJunction],
            TileKind::StraightCurveCrossing => vec![SetContentElement::StraightCurveCrossing],
            TileKind::TripleSmallCurve => vec![SetContentElement::TripleSmallCurve],
            TileKind::FlexibleTwoInOneA => vec![SetContentElement::FlexibleTwoInOneA],
            TileKind::DomeStarter => vec![SetContentElement::DomeStarter],
            TileKind::FinishTrigger => vec![SetContentElement::FinishTrigger],
            TileKind::FinishArena => vec![SetContentElement::FinishArena],
            TileKind::Trigger => vec![SetContentElement::Trigger],
            TileKind::Queue => vec![SetContentElement::Queue],
            TileKind::Lever => vec![SetContentElement::Lever],
            TileKind::Elevator => vec![SetContentElement::Elevator],
            TileKind::Catch => vec![SetContentElement::Catch, SetContentElement::BasicClosed],
            TileKind::GoalBasin => vec![SetContentElement::GoalBasin, SetContentElement::BasicClosed],
            TileKind::Drop => vec![SetContentElement::Drop, SetContentElement::BasicOpen],
            TileKind::TwoWay => vec![SetContentElement::TwoWay],
            TileKind::Splash => vec![SetContentElement::Splash],
            TileKind::Stacker => vec![SetContentElement::Stacker],
            TileKind::StackerSmall => vec![SetContentElement::StackerSmall],
            TileKind::SwitchLeft => vec![SetContentElement::TwoWay, SetContentElement::SwitchInsert],
            TileKind::SwitchRight => vec![SetContentElement::TwoWay, SetContentElement::SwitchInsert],
            TileKind::StackerBatch => vec![],
            TileKind::StraightTunnel => vec![SetContentElement::StraightTunnel, SetContentElement::BasicStraight],
            TileKind::CurveTunnel => vec![SetContentElement::CurveTunnel, SetContentElement::BasicClosed],
            TileKind::SwitchTunnel => vec![SetContentElement::SwitchTunnel, SetContentElement::BasicClosed],
            TileKind::Trampolin0 => vec![SetContentElement::Trampoline],
            TileKind::Trampolin1 => vec![SetContentElement::Trampoline, SetContentElement::StackerAngled],
            TileKind::Trampolin2 => vec![SetContentElement::Trampoline, SetContentElement::StackerAngled, SetContentElement::StackerAngled],
            TileKind::LiftSmall => vec![SetContentElement::LiftEntrance, SetContentElement::LiftExit, SetContentElement::LiftHeightTube],
            TileKind::LiftLarge => vec![SetContentElement::LiftEntrance, SetContentElement::LiftExit, SetContentElement::LiftHeightTube, SetContentElement::LiftHeightTube],
            TileKind::ZiplineStart => vec![SetContentElement::Zipline],
            TileKind::ZiplineEnd => vec![],
            TileKind::ScrewSmall => vec![SetContentElement::SpiralBase, SetContentElement::SpiralEntrance],
            TileKind::ScrewMedium => {
                let mut vec = vec![SetContentElement::SpiralBase, SetContentElement::SpiralEntrance];
                for _ in 0..5 {
                    vec.push(SetContentElement::SpiralCurve);
                }
                vec
            },
            TileKind::ScrewLarge => {
                let mut vec = vec![SetContentElement::SpiralBase, SetContentElement::SpiralEntrance];
                for _ in 0..12 {
                    vec.push(SetContentElement::SpiralCurve);
                }
                vec
            },
            TileKind::MixerOffsetExits => vec![SetContentElement::Mixer],
            TileKind::StackerTowerClosed => vec![SetContentElement::StackerTowerClosed],
            TileKind::StackerTowerOpened => vec![SetContentElement::StackerTowerOpened],
            TileKind::MixerSameExits => vec![SetContentElement::Mixer],
            TileKind::DipperLeft => vec![SetContentElement::Dipper],
            TileKind::DipperRight => vec![SetContentElement::Dipper],
            TileKind::ColorSwapEmpty => vec![SetContentElement::ColorSwap],
            TileKind::ColorSwapPreloaded => vec![SetContentElement::ColorSwap],
            TileKind::CarouselSameExits => vec![SetContentElement::Carousel],
            TileKind::CarouselOffsetExits => vec![SetContentElement::Carousel],
            TileKind::DropdownSwitchLeft => vec![SetContentElement::DropdownSwitch],
            TileKind::DropdownSwitchRight => vec![SetContentElement::DropdownSwitch],
        }
    }

}


impl Set {

    pub fn from_path<P: AsRef<Path>>(path: P) -> MurmelbahnResult<Set> {
         let file = File::open(path).context(ReadSnafu {})?;
         let reader = BufReader::new(file);

         let set = serde_json::from_reader(reader).context(SerdeJsonSnafu)?;
         Ok(set)
    }

}

#[derive(TS)]
#[ts(export)]
pub struct SetRepo {
    pub sets: HashMap<String, Set>
}


impl SetRepo {
    pub fn new() -> SetRepo {
        SetRepo { sets: HashMap::new() }
    }

    pub fn read_directory<P: AsRef<Path>>(&mut self, path: P) -> MurmelbahnResult<()> {
        let path = path.as_ref();

        if !path.exists() || !path.is_dir() {
            return Err(MurmelbahnError::MiscError { msg: "Path is not a directory".to_string() });
        }

        // Read all files in the directory
        for entry in fs::read_dir(path).context(IoSnafu)? {
            let entry = entry.context(IoSnafu)?;
            let file_path = entry.path();

            // Only process files
            if file_path.is_file() {
                let set = Set::from_path(file_path)?;
                let set_id = set.id.clone();
                if let Some(_) = self.sets.insert(set_id.clone(), set) {
                    info!("Set with ID [{}] occurs twice, will use a random one", set_id);
                }
            }
        }

        Ok(())
    }
}
