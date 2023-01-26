use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use crate::error::{IoSnafu, MurmelbahnError, MurmelbahnResult, ReadSnafu, SerdeJsonSnafu};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct Name {
    pub language_code: String,
    pub name: String
}

#[derive(Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct Set {
    pub id: String,

    #[serde(default)]
    pub names: Vec<Name>,

    #[serde(default)]
    pub content: HashMap<SetContentElement, i32>
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize)]
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
    Lift, // TODO: Split up further?
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


impl Set {

    pub fn from_path<P: AsRef<Path>>(path: P) -> MurmelbahnResult<Set> {
         let file = File::open(path).context(ReadSnafu {})?;
         let reader = BufReader::new(file);

         let set = serde_json::from_reader(reader).context(SerdeJsonSnafu)?;
         Ok(set)
    }

}

pub struct SetRepo {
    pub sets: Vec<Set>
}


impl SetRepo {
    pub fn new() -> SetRepo {
        SetRepo { sets: Vec::new() }
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
                self.sets.push(set);
            }
        }

        Ok(())
    }
}
