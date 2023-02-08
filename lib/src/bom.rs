use crate::course::common::layer::{
    LayerConstructionData, LayerKind as PersistenceLayerKind, TileKind as PersistenceTileKind,
    TileTowerConstructionData, TileTowerTreeNodeData,
};
use crate::course::common::pillar::PillarConstructionData;
use crate::course::common::rail::RailKind as PersistenceRailKind;
use crate::course::common::wall::{WallConstructionData, WallSide};
use crate::course::common::{Direction, HexVector};
use crate::course::power2022::Course;
use crate::error::MurmelbahnError::UnsupportedPiece;
use crate::error::{MurmelbahnError, MurmelbahnResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, trace};

// 0.36 is a magic number and it represents the height of a small stacker (in the App at least)
pub const TILE_HEIGHT: f32 = 0.36;

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

impl TryFrom<PersistenceLayerKind> for LayerKind {
    type Error = MurmelbahnError;

    fn try_from(value: PersistenceLayerKind) -> Result<Self, Self::Error> {
        Ok(match value {
            PersistenceLayerKind::BaselayerPiece => LayerKind::Base,
            PersistenceLayerKind::LargeLayer => LayerKind::LargeClear,
            PersistenceLayerKind::SmallLayer => LayerKind::SmallClear,
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

impl TryFrom<PersistenceRailKind> for RailKind {
    type Error = MurmelbahnError;

    fn try_from(value: PersistenceRailKind) -> Result<Self, Self::Error> {
        let value_new = match value {
            PersistenceRailKind::Bernoulli => RailKind::Bernoulli,
            PersistenceRailKind::DropHill => RailKind::DropHill,
            PersistenceRailKind::DropValley => RailKind::DropValley,
            PersistenceRailKind::UTurn => RailKind::UTurn,
            PersistenceRailKind::Narrow => RailKind::Narrow,
            PersistenceRailKind::Slow => RailKind::Slow,
            PersistenceRailKind::BernoulliSmallStraight => RailKind::BernoulliSmallStraight,
            PersistenceRailKind::BernoulliSmallLeft => RailKind::BernoulliSmallLeft,
            PersistenceRailKind::BernoulliSmallRight => RailKind::BernoulliSmallRight,
            PersistenceRailKind::FlexTube0 => RailKind::FlexTube0,
            PersistenceRailKind::FlexTube60 => RailKind::FlexTube60,
            PersistenceRailKind::FlexTube120 => RailKind::FlexTube120,
            PersistenceRailKind::FlexTube180 => RailKind::FlexTube180,
            PersistenceRailKind::FlexTube240 => RailKind::FlexTube240,
            PersistenceRailKind::FlexTube300 => RailKind::FlexTube300,
            PersistenceRailKind::Straight => return Err(UnsupportedPiece),
        };

        Ok(value_new)
    }
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum TileKind {
    Starter,
    Curve,
    Catch,
    GoalBasin,
    Drop,
    Hammer,
    Catapult,
    Cross,
    Threeway,
    TwoWay,
    Spiral,
    Splash,
    Loop,
    Cannon,
    Stacker,
    StackerSmall,
    SwitchLeft,
    SwitchRight,
    GoalRail,
    StackerBatch,
    Cascade,
    StraightTunnel,
    CurveTunnel,
    SwitchTunnel,
    Trampolin0,
    Trampolin1,
    Trampolin2,
    LiftSmall,
    LiftLarge,
    Flip,
    TipTube,
    Volcano,
    Jumper,
    Transfer,
    ZiplineStart,
    ZiplineEnd,
    Bridge,
    ScrewSmall,
    ScrewMedium,
    ScrewLarge,
    MixerOffsetExits,
    Splitter,
    StackerTowerClosed,
    StackerTowerOpened,
    Balcony,
    DoubleBalcony,
    MixerSameExits,
    DipperLeft,
    DipperRight,
    Helix,
    Turntable,
    Spinner,
    TwoInOneSmallCurveA,
    TwoInOneSmallCurveB,
    FlexibleTwoInOneB,
    RibbonCurve,
    ThreeEntranceFunnel,
    CurveCrossing,
    DoubleBigCurve,
    DoubleSmallCurve,
    MultiJunction,
    StraightCurveCrossing,
    TripleSmallCurve,
    FlexibleTwoInOneA,
    ColorSwapEmpty,
    ColorSwapPreloaded,
    CarouselSameExits,
    CarouselOffsetExits,
    DomeStarter,
    FinishTrigger,
    FinishArena,
    Trigger,
    DropdownSwitchLeft,
    DropdownSwitchRight,
    Queue,
    Lever,
    Elevator,
}

impl TryFrom<PersistenceTileKind> for TileKind {
    type Error = MurmelbahnError;

    fn try_from(value: PersistenceTileKind) -> Result<Self, Self::Error> {
        Ok(match value {
            PersistenceTileKind::Starter => TileKind::Starter,
            PersistenceTileKind::Curve => TileKind::Curve,
            PersistenceTileKind::Catch => TileKind::Catch,
            PersistenceTileKind::GoalBasin => TileKind::GoalBasin,
            PersistenceTileKind::Drop => TileKind::Drop,
            PersistenceTileKind::Hammer => TileKind::Hammer,
            PersistenceTileKind::Catapult => TileKind::Catapult,
            PersistenceTileKind::Cross => TileKind::Cross,
            PersistenceTileKind::Threeway => TileKind::Threeway,
            PersistenceTileKind::TwoWay => TileKind::TwoWay,
            PersistenceTileKind::Spiral => TileKind::Spiral,
            PersistenceTileKind::Splash => TileKind::Splash,
            PersistenceTileKind::Loop => TileKind::Loop,
            PersistenceTileKind::Cannon => TileKind::Cannon,
            PersistenceTileKind::Stacker => TileKind::Stacker,
            PersistenceTileKind::StackerSmall => TileKind::StackerSmall,
            PersistenceTileKind::SwitchLeft => TileKind::SwitchLeft,
            PersistenceTileKind::SwitchRight => TileKind::SwitchRight,
            PersistenceTileKind::GoalRail => TileKind::GoalRail,
            PersistenceTileKind::StackerBatch => TileKind::StackerBatch,
            PersistenceTileKind::Cascade => TileKind::Cascade,
            PersistenceTileKind::StraightTunnel => TileKind::StraightTunnel,
            PersistenceTileKind::CurveTunnel => TileKind::CurveTunnel,
            PersistenceTileKind::SwitchTunnel => TileKind::SwitchTunnel,
            PersistenceTileKind::Trampolin0 => TileKind::Trampolin0,
            PersistenceTileKind::Trampolin1 => TileKind::Trampolin1,
            PersistenceTileKind::Trampolin2 => TileKind::Trampolin2,
            PersistenceTileKind::LiftSmall => TileKind::LiftSmall,
            PersistenceTileKind::LiftLarge => TileKind::LiftLarge,
            PersistenceTileKind::Flip => TileKind::Flip,
            PersistenceTileKind::TipTube => TileKind::TipTube,
            PersistenceTileKind::Volcano => TileKind::Volcano,
            PersistenceTileKind::Jumper => TileKind::Jumper,
            PersistenceTileKind::Transfer => TileKind::Transfer,
            PersistenceTileKind::ZiplineStart => TileKind::ZiplineStart,
            PersistenceTileKind::ZiplineEnd => TileKind::ZiplineEnd,
            PersistenceTileKind::Bridge => TileKind::Bridge,
            PersistenceTileKind::ScrewSmall => TileKind::ScrewSmall,
            PersistenceTileKind::ScrewMedium => TileKind::ScrewMedium,
            PersistenceTileKind::ScrewLarge => TileKind::ScrewLarge,
            PersistenceTileKind::MixerOffsetExits => TileKind::MixerOffsetExits,
            PersistenceTileKind::Splitter => TileKind::Splitter,
            PersistenceTileKind::StackerTowerClosed => TileKind::StackerTowerClosed,
            PersistenceTileKind::StackerTowerOpened => TileKind::StackerTowerOpened,
            PersistenceTileKind::DoubleBalcony => TileKind::DoubleBalcony,
            PersistenceTileKind::MixerSameExits => TileKind::MixerSameExits,
            PersistenceTileKind::DipperLeft => TileKind::DipperLeft,
            PersistenceTileKind::DipperRight => TileKind::DipperRight,
            PersistenceTileKind::Helix => TileKind::Helix,
            PersistenceTileKind::Turntable => TileKind::Turntable,
            PersistenceTileKind::Spinner => TileKind::Spinner,
            PersistenceTileKind::TwoInOneSmallCurveA => TileKind::TwoInOneSmallCurveA,
            PersistenceTileKind::TwoInOneSmallCurveB => TileKind::TwoInOneSmallCurveB,
            PersistenceTileKind::FlexibleTwoInOneB => TileKind::FlexibleTwoInOneB,
            PersistenceTileKind::RibbonCurve => TileKind::RibbonCurve,
            PersistenceTileKind::ThreeEntranceFunnel => TileKind::ThreeEntranceFunnel,
            PersistenceTileKind::CurveCrossing => TileKind::CurveCrossing,
            PersistenceTileKind::DoubleBigCurve => TileKind::DoubleBigCurve,
            PersistenceTileKind::DoubleSmallCurve => TileKind::DoubleSmallCurve,
            PersistenceTileKind::MultiJunction => TileKind::MultiJunction,
            PersistenceTileKind::StraightCurveCrossing => TileKind::StraightCurveCrossing,
            PersistenceTileKind::TripleSmallCurve => TileKind::TripleSmallCurve,
            PersistenceTileKind::FlexibleTwoInOneA => TileKind::FlexibleTwoInOneA,
            PersistenceTileKind::ColorSwapEmpty => TileKind::ColorSwapEmpty,
            PersistenceTileKind::ColorSwapPreloaded => TileKind::ColorSwapPreloaded,
            PersistenceTileKind::CarouselSameExits => TileKind::CarouselSameExits,
            PersistenceTileKind::CarouselOffsetExits => TileKind::CarouselOffsetExits,
            PersistenceTileKind::DomeStarter => TileKind::DomeStarter,
            PersistenceTileKind::FinishTrigger => TileKind::FinishTrigger,
            PersistenceTileKind::FinishArena => TileKind::FinishArena,
            PersistenceTileKind::Trigger => TileKind::Trigger,
            PersistenceTileKind::DropdownSwitchLeft => TileKind::DropdownSwitchLeft,
            PersistenceTileKind::DropdownSwitchRight => TileKind::DropdownSwitchRight,
            PersistenceTileKind::Queue => TileKind::Queue,
            PersistenceTileKind::Lever => TileKind::Lever,
            PersistenceTileKind::Elevator => TileKind::Elevator,
            _ => {
                return Err(MurmelbahnError::ConversionFailed {
                    msg: "Could not convert tile".to_string(),
                })
            }
        })
    }
}

/// This is the Bill of Materials as it appears in the app.
/// That is not very useful if you want to check whether you can build a course with your parts
/// as it includes things like `SwitchLeft` and `SwitchRight` which are the same physical tile,
/// just placed in a different configuration.
#[derive(Debug, Default, Deserialize, JsonSchema, Serialize)]
pub struct AppBillOfMaterials {
    pub layers: HashMap<LayerKind, i32>,
    pub tiles: HashMap<TileKind, i32>,
    pub rails: HashMap<RailKind, i32>,
    pub walls: HashMap<WallKind, i32>,
}

impl AppBillOfMaterials {
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
        let zipline = self.tile_kind(TileKind::ZiplineStart).unwrap_or(0);
        let cannon = self.tile_kind(TileKind::Cannon).unwrap_or(0);
        let bridge = self.tile_kind(TileKind::Bridge).unwrap_or(0);
        let color_change = self.tile_kind(TileKind::ColorSwapPreloaded).unwrap_or(0);
        let catapult = self.tile_kind(TileKind::Catapult).unwrap_or(0);
        let lift_small = self.tile_kind(TileKind::LiftSmall).unwrap_or(0);
        let lift_large = self.tile_kind(TileKind::LiftLarge).unwrap_or(0);
        // TODO: Tiptube?

        // TODO:
        // TODO: To get better number we should check how many rails/adjacent tiles there are
        // for this next group
        /*
        let splash = self.tile_kind(TileKind::Splash);
        let volcano = self.tile_kind(TileKind::Volcano);
        let spinner = self.tile_kind(TileKind::Spinner);

        let dome_starter = self.tile_kind(TileKind::DomeStarter);
        let starter = self.tile_kind(TileKind::Starter);
         */

        let min_marbles = cannon * 2
            + zipline
            + color_change
            + bridge * 2
            + catapult * 4
            + lift_small * 5
            + lift_large * 8;

        let max_marbles = cannon * 2
            + zipline
            + color_change
            + bridge * 2
            + catapult * 4
            + lift_small * 5
            + lift_large * 8;

        (min_marbles, max_marbles)
    }
}

#[derive(Clone, Debug)]
struct RetainerHeight {
    lower: i32,
    upper: i32,
}

impl RetainerHeight {
    fn new(lower: i32, upper: i32) -> RetainerHeight {
        RetainerHeight { lower, upper }
    }
}

#[derive(Default)]
struct CountContext {
    retainer_positions: HashMap<i32, HexVector>,

    /// A 'retainer' is anything that can "hold" or "retain" other tiles or items.
    /// Base layers for example but also balconies and other things
    /// Each of those has a height which is measured in small stackers.
    /// We record the lower and the upper height here
    retainer_heights: HashMap<i32, RetainerHeight>,

    layers: HashMap<LayerKind, i32>,
    tiles: HashMap<TileKind, i32>,
    walls: HashMap<WallKind, i32>,
    rails: HashMap<RailKind, i32>,
}

impl CountContext {
    fn local_to_world_hex_vector(&self, local_hex_vector: &HexVector, layer_id: i32) -> HexVector {
        let layer = self.retainer_positions.get(&layer_id).unwrap(); // TODO;
        HexVector::new(local_hex_vector.x + layer.x, local_hex_vector.y + layer.y)
    }

    fn add_layer(&mut self, layer: &LayerConstructionData) -> MurmelbahnResult<RetainerHeight> {
        // Update the count
        let entry = self
            .layers
            .entry(LayerKind::try_from(layer.layer_kind.clone())?)
            .or_insert(0);
        *entry += 1;

        // Then update the world position of this layer
        // The positions at this level are already absolute ones
        self.retainer_positions
            .insert(layer.layer_id, layer.world_hex_position.clone());

        // Layer height in small stackers
        let lower_layer_height = (layer.layer_height / TILE_HEIGHT).round() as i32;
        let retainer_height = RetainerHeight::new(lower_layer_height, lower_layer_height + 1);
        self.retainer_heights
            .insert(layer.layer_id, retainer_height.clone());

        Ok(retainer_height)
    }

    fn add_tiletowerconstructiondata(&mut self, tile: &TileTowerConstructionData) {
        let kind = TileKind::try_from(tile.kind.clone()).unwrap();
        let entry = self.tiles.entry(kind).or_insert(0);
        *entry += 1;
    }

    fn add_stackers(&mut self, mut small_stacker: i32) {
        // We need to calculate the small/large stacker per stack/cell/pillar and not overall as each stack with
        // an uneven number of small stackers actually needs at least one physical small stacker
        if small_stacker % 2 != 0 {
            self.add_tile(TileKind::StackerSmall, 1);
            small_stacker -= 1;
        }
        self.add_tile(TileKind::Stacker, small_stacker / 2);
    }

    fn add_wall(&mut self, wall: WallKind) {
        let entry = self.walls.entry(wall).or_insert(0);
        *entry += 1;
    }

    fn add_tile(&mut self, tile: TileKind, amount: i32) {
        let entry = self.tiles.entry(tile).or_insert(0);
        *entry += amount;
    }

    fn add_rail(&mut self, rail: RailKind) {
        let entry = self.rails.entry(rail).or_insert(0);
        *entry += 1;
    }
}

// TODO: Use world positions everywhere?
impl TryFrom<Course> for AppBillOfMaterials {
    type Error = MurmelbahnError;

    fn try_from(value: Course) -> Result<Self, Self::Error> {
        let mut context = CountContext::default();

        process_layer_construction_data(&value.layer_construction_data, &mut context)?;
        process_pillar_construction_data(&value.pillar_construction_data, &mut context);
        process_wall_construction_data(&value.wall_construction_data, &mut context);

        for rail_construction_datum in value.rail_construction_data.iter() {
            // As far as I know `Straight` rails are the only ones that come in different length but are only
            // encoded as a single enum variant.
            let rail_kind = if &rail_construction_datum.rail_kind == &PersistenceRailKind::Straight
            {
                // A rail has two ends/exits, both are located on a layer,
                // the layer in question is found in the `retainer_id` field
                let exit_1_world_pos = context.local_to_world_hex_vector(
                    &rail_construction_datum.exit_1_identifier.cell_local_hex_pos,
                    rail_construction_datum.exit_1_identifier.retainer_id,
                );
                let exit_2_world_pos = context.local_to_world_hex_vector(
                    &rail_construction_datum.exit_2_identifier.cell_local_hex_pos,
                    rail_construction_datum.exit_2_identifier.retainer_id,
                );

                let distance = exit_1_world_pos.distance(&exit_2_world_pos) - 1;

                match distance {
                    1 => RailKind::StraightSmall,
                    2 => RailKind::StraightMedium,
                    3 => RailKind::StraightLarge,
                    _ => {
                        error!("Unrecognized length for small rail: {}", distance); // TODO: Abort?
                        panic!("No no no");
                    }
                }
            } else {
                RailKind::try_from(rail_construction_datum.rail_kind.clone()).unwrap()
            };

            context.add_rail(rail_kind);
        }

        Ok(AppBillOfMaterials {
            layers: context.layers,
            tiles: context.tiles,
            rails: context.rails,
            walls: context.walls,
        })
    }
}

/// This processes [`LayerConstructionData`] objects and updates the [`CountContext`] with
/// information on tiles in use (but this is not complete as walls / balconies can also have tiles on them),
/// layers (base, clear etc.), and the heights of layers.
///
/// This is not an associated function to make it easier to test in isolation.
fn process_layer_construction_data(
    layers: &[LayerConstructionData],
    mut context: &mut CountContext,
) -> MurmelbahnResult<()> {
    for layer in layers.iter() {
        trace!(
            "Processing layer id [{}] of kind [{:?}] and height [{}] at position [{:?}]]",
            layer.layer_id,
            layer.layer_kind,
            layer.layer_height,
            layer.world_hex_position
        );

        // Process the layer itself
        let height = context.add_layer(layer)?;

        // And now process all its (potential) children
        for cell in layer.cell_construction_datas.iter() {
            // Convert from local to world position as early as possible
            let world_cell_position =
                context.local_to_world_hex_vector(&cell.local_hex_position, layer.layer_id);
            process_tree_node_data(
                &cell.tree_node_data,
                &world_cell_position,
                height.upper,
                &mut context,
            );
        }
    }

    trace!("Layer heights:\n{:#?}", context.retainer_heights);

    Ok(())
}

fn process_tree_node_data(
    data: &TileTowerTreeNodeData,
    // Each [`TileTowerTreeNodeData`] object belongs to one and only one cell on the board
    world_cell_position: &HexVector,
    // TODO:
    mut current_height: i32,
    context: &mut CountContext,
) {
    // TODO: Maybe use our own TileKind object here as well
    context.add_tiletowerconstructiondata(&data.construction_data);

    context.add_stackers(data.construction_data.height_in_small_stacker);

    // Check if this current tile at this cell is a retainer
    // and add it to the retainer_positions and retainer_heights if it is
    // A retainer here can be a double balcony, a stacker tower and ...not sure  what else
    if let Some(retainer_id) = data.construction_data.retainer_id {
        context
            .retainer_positions
            .insert(retainer_id, world_cell_position.clone());

        if matches!(
            data.construction_data.kind,
            PersistenceTileKind::StackerTowerOpened | PersistenceTileKind::StackerTowerClosed
        ) {
            context.retainer_heights.insert(
                retainer_id,
                RetainerHeight::new(
                    current_height,
                    current_height + 14 + data.construction_data.height_in_small_stacker,
                ), // TODO: Maybe this 14 needs to be a 13 for reasons I don't understand
            );
            current_height += 14 + data.construction_data.height_in_small_stacker;
        } else {
            // All other things are 1 high I believe (i.e. DoubleBalcony)
            context.retainer_heights.insert(
                retainer_id,
                RetainerHeight::new(
                    current_height,
                    data.construction_data.height_in_small_stacker + current_height + 1,
                ),
            );
            current_height += data.construction_data.height_in_small_stacker;
        }
    }

    for child in data.children.iter() {
        process_tree_node_data(child, world_cell_position, current_height, context);
    }
}

fn process_pillar_construction_data(
    pillars: &[PillarConstructionData],
    context: &mut CountContext,
) {
    for pillar in pillars.iter() {
        let lower_layer_height = context
            .retainer_heights
            .get(&pillar.lower_layer_id)
            .unwrap();
        let upper_layer_height = context
            .retainer_heights
            .get(&pillar.upper_layer_id)
            .unwrap();

        // The base plates have a layer_height of -1, which probably means that the layer_height
        // is the height at the lower end of a layer, which means you have to add 1 (for thickness
        // of all layer kinds we know so far) if you want to build on top of them
        // e.g. upper_height = 10, lower_height = -1: 10 - (-1 + 1) = 10 - 0 = 10
        // Means we need 10 small or 5 large stackers
        // Stacker towers have a size of 14 (7 large stackers)
        // e.g. a layer has a height of 19 but it is built on top of this tower:
        // base plate (-1), 3 small stackers: 19 - 14 - 3 = 2, means one large stacker is needed
        // TODO: For some reason this means I need to add 13 (not 14) for every stacker tower....
        // I should really understand this
        let small_stacker = upper_layer_height.lower - lower_layer_height.upper;
        trace!(
            "Pillar data: {} ({:?}) -> {} ({:?}): {}/{}",
            pillar.lower_layer_id,
            lower_layer_height,
            pillar.upper_layer_id,
            upper_layer_height,
            if small_stacker % 2 != 0 { 1 } else { 0 },
            if small_stacker % 2 != 0 {
                (small_stacker - 1) / 2
            } else {
                small_stacker / 2
            }
        );

        context.add_stackers(small_stacker);
    }
}

fn process_wall_construction_data(walls: &[WallConstructionData], mut context: &mut CountContext) {
    for wall in walls.iter() {
        context.add_tile(
            TileKind::Balcony,
            wall.balcony_construction_datas.len() as i32,
        );

        let tower_1_world_pos = &context.local_to_world_hex_vector(
            &wall.lower_stacker_tower_1_local_hex_pos,
            wall.lower_stacker_tower_1_retainer_id,
        );
        let tower_2_world_pos = &context.local_to_world_hex_vector(
            &wall.lower_stacker_tower_2_local_hex_pos,
            wall.lower_stacker_tower_2_retainer_id,
        );

        // Distance in fields -1 because we usually want to know how long a thing needs to be between
        // both cells (e.g. rails and walls)
        let distance = tower_1_world_pos.distance(&tower_2_world_pos) - 1;

        let wall_direction = hex_direction(&tower_1_world_pos, &tower_2_world_pos);

        trace!("Wall:\n{:#?}", wall);
        trace!(
            "Distance between walls: {}, wall direction: {:?}",
            distance,
            wall_direction
        );

        context.add_wall(WallKind::straight_of_length(distance));

        for balcony in wall.balcony_construction_datas.iter() {
            let hex_vector = wall
                .lower_stacker_tower_1_local_hex_pos
                .hex_vector_in_distance(&wall_direction, balcony.wall_coordinate.column);

            let target_direction = wall_side_direction(&wall_direction, &balcony.wall_side);
            let balcony_hex_vector = hex_vector.neighbor(&target_direction);

            let balcony_world_hex_vector = context.local_to_world_hex_vector(
                &balcony_hex_vector,
                wall.lower_stacker_tower_1_retainer_id,
            );

            context
                .retainer_positions
                .insert(balcony.retainer_id, balcony_world_hex_vector.clone());

            if let Some(cell_construction_data) = &balcony.cell_construction_datas {
                process_tree_node_data(
                    &cell_construction_data.tree_node_data,
                    &balcony_world_hex_vector,
                    0,
                    &mut context,
                );
            }
        }
    }
}

fn hex_direction(from: &HexVector, to: &HexVector) -> Direction {
    let x_diff = to.x - from.x;
    let y_diff = to.y - from.y;

    if x_diff > 0 && y_diff < 0 {
        Direction::NorthEast
    } else if x_diff == 0 && y_diff < 0 {
        Direction::East
    } else if x_diff < 0 && y_diff == 0 {
        Direction::SouthEast
    } else if x_diff < 0 && y_diff > 0 {
        Direction::SouthWest
    } else if x_diff == 0 && y_diff > 0 {
        Direction::West
    } else {
        Direction::NorthWest
    }
}

fn wall_side_direction(direction: &Direction, wall_side: &WallSide) -> Direction {
    match (direction, wall_side) {
        (Direction::NorthEast, WallSide::East) => Direction::East,
        (Direction::NorthEast, WallSide::West) => Direction::NorthWest,
        (Direction::East, WallSide::East) => Direction::SouthEast,
        (Direction::East, WallSide::West) => Direction::NorthEast,
        (Direction::SouthEast, WallSide::East) => Direction::SouthWest,
        (Direction::SouthEast, WallSide::West) => Direction::East,
        (Direction::SouthWest, WallSide::East) => Direction::West,
        (Direction::SouthWest, WallSide::West) => Direction::SouthEast,
        (Direction::West, WallSide::East) => Direction::NorthWest,
        (Direction::West, WallSide::West) => Direction::SouthWest,
        (Direction::NorthWest, WallSide::East) => Direction::NorthEast,
        (Direction::NorthWest, WallSide::West) => Direction::West,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_hex_direction() {
        let hex_vector_from = HexVector::new(0, 0);

        let hex_vector_northeast = HexVector::new(1, -1);
        assert_eq!(
            Direction::NorthEast,
            hex_direction(&hex_vector_from, &hex_vector_northeast)
        );

        let hex_vector_east = HexVector::new(0, -1);
        assert_eq!(
            Direction::East,
            hex_direction(&hex_vector_from, &hex_vector_east)
        );

        let hex_vector_southeast = HexVector::new(-1, 0);
        assert_eq!(
            Direction::SouthEast,
            hex_direction(&hex_vector_from, &hex_vector_southeast)
        );

        let hex_vector_southwest = HexVector::new(-1, -1);
        assert_eq!(
            Direction::SouthWest,
            hex_direction(&hex_vector_from, &hex_vector_southwest)
        );

        let hex_vector_west = HexVector::new(0, 1);
        assert_eq!(
            Direction::West,
            hex_direction(&hex_vector_from, &hex_vector_west)
        );

        let hex_vector_northwest = HexVector::new(1, 0);
        assert_eq!(
            Direction::NorthWest,
            hex_direction(&hex_vector_from, &hex_vector_northwest)
        );

        let hex_vector_from = HexVector::new(-1, 4);
        let hex_vector_to = HexVector::new(3, 0);
        assert_eq!(
            Direction::NorthEast,
            hex_direction(&hex_vector_from, &hex_vector_to)
        )
    }

     */
}
