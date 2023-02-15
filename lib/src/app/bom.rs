use crate::app::course::{Course, Direction, HexVector};
use crate::app::layer::{
    LayerConstructionData, LayerKind, TileKind, TileTowerConstructionData, TileTowerTreeNodeData,
};
use crate::app::pillar::PillarConstructionData;
use crate::app::rail::{RailConstructionData, RailKind};
use crate::app::wall::{WallConstructionData, WallKind, WallSide};
use crate::app::ziplineadded2019::LayerConstructionData as ZiplineLayerConstructionData;
use serde::Serialize;
use std::collections::HashMap;
use tracing::{error, trace};

// 0.36 is a magic number and it represents the height of a small stacker (in the App at least)
const TILE_HEIGHT: f32 = 0.36;

/// This is the Bill of Materials as it appears in the app.
/// That is not very useful if you want to check whether you can build a course with your parts
/// as it includes things like `SwitchLeft` and `SwitchRight` which are the same physical tile,
/// just placed in a different configuration.
#[derive(Debug, Default, Serialize)]
pub struct BillOfMaterials {
    pub layers: HashMap<LayerKind, i32>,
    pub tiles: HashMap<TileKind, i32>,
    pub rails: HashMap<RailKind, i32>,
    pub walls: HashMap<WallKind, i32>,
    pub balconies: i32,
    pub rails_small: i32,
    pub rails_medium: i32,
    pub rails_large: i32,
}

impl BillOfMaterials {
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

    /// This returns a rough estimate on how many marbles are needed.
    /// It won't be perfect because some tiles can use more than one or none etc.
    /// Some courses also have self-loading cannons which don't need any extras and so on.
    pub fn marbles(&self) -> i32 {
        let zipline = self.tile_kind(TileKind::ZiplineStart).unwrap_or(0);
        let cannon = self.tile_kind(TileKind::Cannon).unwrap_or(0);
        let bridge = self.tile_kind(TileKind::Bridge).unwrap_or(0);
        let color_change = self.tile_kind(TileKind::ColorSwapPreloaded).unwrap_or(0);
        let catapult = self.tile_kind(TileKind::Catapult).unwrap_or(0);
        let lift_small = self.tile_kind(TileKind::LiftSmall).unwrap_or(0);
        let lift_large = self.tile_kind(TileKind::LiftLarge).unwrap_or(0);
        // TODO: Tiptube?

        // TODO: To get better number we should check how many rails/adjacent tiles there are
        // for this next group
        let splash = self.tile_kind(TileKind::Splash).unwrap_or(0);
        let volcano = self.tile_kind(TileKind::Volcano).unwrap_or(0);
        let spinner = self.tile_kind(TileKind::Spinner).unwrap_or(0);

        let dome_starter = self.tile_kind(TileKind::DomeStarter).unwrap_or(0);
        let starter = self.tile_kind(TileKind::Starter).unwrap_or(0);

        let marble_guess = cannon * 2
            + zipline
            + color_change
            + bridge * 2
            + catapult * 4
            + lift_small * 5
            + lift_large * 8
            + starter
            + spinner
            + splash
            + volcano
            + dome_starter;

        marble_guess
    }
}

// TODO: Use world positions everywhere?
impl From<Course> for BillOfMaterials {
    fn from(value: Course) -> Self {
        let mut context = CountContext::default();

        match value {
            Course::ZiplineAdded2019(course) => {
                process_layer_construction_data_zipline(
                    &course.layer_construction_data,
                    &mut context,
                );
                process_pillar_construction_data(&course.pillar_construction_data, &mut context);
                process_rail_construction_data(&course.rail_construction_data, &mut context);
            }
            Course::Power2022(course) | Course::Pro2020(course) => {
                process_layer_construction_data(&course.layer_construction_data, &mut context);
                process_pillar_construction_data(&course.pillar_construction_data, &mut context);
                process_wall_construction_data(&course.wall_construction_data, &mut context);
                process_rail_construction_data(&course.rail_construction_data, &mut context);
            }
        }

        BillOfMaterials {
            layers: context.layers,
            tiles: context.tiles,
            rails: context.rails,
            walls: context.walls,
            balconies: context.balconies,
            rails_small: context.rail_small,
            rails_medium: context.rail_medium,
            rails_large: context.rail_large,
        }
    }
}

/// This struct records the height of any retainer (e.g. a layer or a balcony).
/// It keeps the height of the lower end as well as the upper end separately to account for
/// different thicknesses.
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
    pub balconies: i32,
    pub rail_small: i32,
    pub rail_medium: i32,
    pub rail_large: i32,
}

impl CountContext {
    fn local_to_world_hex_vector(&self, local_hex_vector: &HexVector, layer_id: i32) -> HexVector {
        let layer = self.retainer_positions.get(&layer_id).unwrap(); // TODO;
        HexVector::new(local_hex_vector.x + layer.x, local_hex_vector.y + layer.y)
    }

    fn add_layer(&mut self, layer: &LayerConstructionData) -> RetainerHeight {
        // Update the count
        let entry = self.layers.entry(layer.layer_kind.clone()).or_insert(0);
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

        retainer_height
    }

    fn add_zipline_layer(&mut self, layer: &ZiplineLayerConstructionData) {
        // Update the count
        let entry = self.layers.entry(layer.layer_kind.clone()).or_insert(0);
        *entry += 1;

        // Then update the world position of this layer
        // The positions at this level are already absolute ones
        self.retainer_positions
            .insert(layer.layer_id, layer.hex_vector.clone());

        // Layer height in small stackers
        let lower_layer_height = (layer.layer_height / TILE_HEIGHT).round() as i32;
        let retainer_height = RetainerHeight::new(lower_layer_height, lower_layer_height + 1);
        self.retainer_heights
            .insert(layer.layer_id, retainer_height.clone());
    }

    fn add_tiletowerconstructiondata(&mut self, tile: &TileTowerConstructionData) {
        let entry = self.tiles.entry(tile.kind.clone()).or_insert(0);
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

fn process_layer_construction_data_zipline(
    layers: &[ZiplineLayerConstructionData],
    context: &mut CountContext,
) {
    for layer in layers {
        trace!(
            "Processing layer id [{}] of kind [{:?}] and height [{}] at position [{:?}]]",
            layer.layer_id,
            layer.layer_kind,
            layer.layer_height,
            layer.hex_vector
        );

        // Process the layer itself
        context.add_zipline_layer(layer);

        // And now process all tiles which is much easier here because we don't have any retainers
        for cell in layer.cell_construction_data.iter() {
            for tile_kind in cell.tile_kinds.iter() {
                context.add_tile(tile_kind.clone(), 1);
            }
        }
    }
}

/// This processes [`LayerConstructionData`] objects and updates the [`CountContext`] with
/// information on tiles in use (but this is not complete as walls / balconies can also have tiles on them),
/// layers (base, clear etc.), and the heights of layers.
///
/// This is not an associated function to make it easier to test in isolation.
fn process_layer_construction_data(layers: &[LayerConstructionData], context: &mut CountContext) {
    for layer in layers.iter() {
        trace!(
            "Processing layer id [{}] of kind [{:?}] and height [{}] at position [{:?}]]",
            layer.layer_id,
            layer.layer_kind,
            layer.layer_height,
            layer.world_hex_position
        );

        // Process the layer itself
        let height = context.add_layer(layer);

        // And now process all its (potential) children
        for cell in layer.cell_construction_datas.iter() {
            // Convert from local to world position as early as possible
            let world_cell_position =
                context.local_to_world_hex_vector(&cell.local_hex_position, layer.layer_id);
            process_tree_node_data(
                &cell.tree_node_data,
                &world_cell_position,
                height.upper,
                context,
            );
        }
    }

    trace!("Layer heights:\n{:#?}", context.retainer_heights);
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
            TileKind::StackerTowerOpened | TileKind::StackerTowerClosed
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

/// Processes [`WallConstructionData`] including their balconies.
fn process_wall_construction_data(walls: &[WallConstructionData], context: &mut CountContext) {
    for wall in walls.iter() {
        context.balconies += wall.balcony_construction_datas.len() as i32;

        // Convert all local coordinates to world coordinates as soon as possible
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
        let distance = tower_1_world_pos.distance(tower_2_world_pos) - 1;
        context.add_wall(WallKind::straight_of_length(distance));

        let wall_direction = hex_direction(tower_1_world_pos, tower_2_world_pos);

        trace!("Wall:\n{:#?}", wall);
        trace!(
            "Distance between walls: {}, wall direction: {:?}",
            distance,
            wall_direction
        );

        // Process balconies as they can all be retainers and we need to know the exact positions
        // of each of those balconies for proper length calculations of rails
        for balcony in wall.balcony_construction_datas.iter() {
            // This walks a number of cells in the direction of the wall based on the column
            // in which this particular balcony resides.
            // Column 0 means that we "stay" at the starting field.
            let hex_vector = tower_1_world_pos
                .hex_vector_in_distance(&wall_direction, balcony.wall_coordinate.column);

            // From the wall side and the wall direction we need to get the global direction of where
            // the balcony will end up in.
            // When we have that we will get the target cell by finding the proper neighbor to the
            // wall cell we found above.
            let target_direction = wall_side_direction(&wall_direction, &balcony.wall_side);
            let balcony_hex_vector = hex_vector.neighbor(&target_direction);

            // Remember the positions of all balconies
            context
                .retainer_positions
                .insert(balcony.retainer_id, balcony_hex_vector.clone());

            // Now process everything that is built on top of a balcony (which will require the proper
            // retainer location we calculated above)
            if let Some(cell_construction_data) = &balcony.cell_construction_datas {
                process_tree_node_data(
                    &cell_construction_data.tree_node_data,
                    &balcony_hex_vector,
                    0,
                    context,
                );
            }
        }
    }
}

fn process_rail_construction_data(rails: &[RailConstructionData], context: &mut CountContext) {
    for rail_construction_datum in rails.iter() {
        // As far as I know `Straight` rails are the only ones that come in different length but are only
        // encoded as a single enum variant.
        if rail_construction_datum.rail_kind == RailKind::Straight {
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
                1 => context.rail_small += 1,
                2 => context.rail_medium += 1,
                3 => context.rail_large += 1,
                _ => {
                    error!("Unrecognized length for small rail: {}", distance); // TODO: Abort?
                    panic!("No no no");
                }
            }
        } else {
            context.add_rail(rail_construction_datum.rail_kind.clone());
        };
    }
}

/// This calculates the direction between two hexes if going from one to the other.
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

/// Returns the absolute direction a wall side is facing for a specific wall direction.
///
/// Walls in the App have a start and an end point.
/// By connecting the two you get a direction in which the wall is going.
/// Balconies can be attached to either side of the wall and the sides are referred to as east and west
/// which is relative to the direction of the wall and not absolute in relation to the whole board.
///
/// To calculate distances between elements on the board we need to know the absolute direction a balcony is facing.
/// This methods takes care of calculating that direction.
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
