use crate::bom;
use crate::bom::{BillOfMaterial, WallKind};
use deku::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use tracing::{error, trace};

use crate::course::common::layer::{LayerConstructionData, TileKind, TileTowerConstructionData, TileTowerTreeNodeData};
use crate::course::common::pillar::PillarConstructionData;
use crate::course::common::rail::{RailConstructionData, RailKind};
use crate::course::common::wall::{WallConstructionData, WallSide};
use crate::course::common::{
    CourseElementGeneration, CourseMetaData, CourseSaveDataVersion, Direction, HexVector,
};
use crate::error::MurmelbahnResult;

// 0.36 is a magic number and it represents the height of a small stacker (in the App at least)
pub const TILE_HEIGHT: f32 = 0.36;

#[deku_derive(DekuRead)]
#[derive(Debug, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion")]
pub struct Course {
    pub meta_data: CourseMetaData,

    #[deku(temp)]
    layer_construction_data_size: u32,
    #[deku(ctx = "version")]
    #[deku(count = "layer_construction_data_size")]
    pub layer_construction_data: Vec<LayerConstructionData>,

    #[deku(temp)]
    rail_construction_data_size: u32,
    #[deku(count = "rail_construction_data_size")]
    pub rail_construction_data: Vec<RailConstructionData>,

    #[deku(temp)]
    pillar_construction_data_size: u32,
    #[deku(count = "pillar_construction_data_size")]
    pub pillar_construction_data: Vec<PillarConstructionData>,

    pub generation: CourseElementGeneration,

    #[deku(temp)]
    wall_construction_data_size: i32,
    #[deku(ctx = "version")]
    #[deku(count = "wall_construction_data_size")]
    pub wall_construction_data: Vec<WallConstructionData>,
}

#[derive(Clone, Debug)]
struct RetainerHeight {
    lower: i32,
    upper: i32
}

impl RetainerHeight {
    fn new(lower: i32, upper: i32) -> RetainerHeight {
        RetainerHeight {
            lower,
            upper
        }
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
    layers: HashMap<bom::LayerKind, i32>,
    tile_counts: HashMap<TileKind, i32>,
    small_stacker_count: i32,
    large_stacker_count: i32,
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
            .entry(bom::LayerKind::try_from(layer.layer_kind.clone())?)
            .or_insert(0);
        *entry += 1;

        // Then update the world position of this layer
        // The positions at this level are already absolute ones
        self.retainer_positions
            .insert(layer.layer_id, layer.world_hex_position.clone());

        // Layer height in small stackers
        let lower_layer_height = (layer.layer_height / TILE_HEIGHT).round() as i32;
        let retainer_height = RetainerHeight::new(lower_layer_height, lower_layer_height + 1);
        self.retainer_heights.insert(layer.layer_id, retainer_height.clone());

        Ok(retainer_height)
    }

    fn add_tile(&mut self, tile: &TileTowerConstructionData) {
        let kind = tile.kind.clone();
        let entry = self.tile_counts.entry(kind).or_insert(0);
        *entry += 1;
    }

    fn add_small_stackers(&mut self, mut small_stacker: i32) {
        // We need to calculate the small/large stacker per stack/cell/pillar and not overall as each stack with
        // an uneven number of small stackers actually needs at least one physical small stacker
        if small_stacker % 2 != 0 {
            self.small_stacker_count += 1;
            small_stacker -= 1;
        }
        self.large_stacker_count += small_stacker / 2;

    }
}

// TODO: Use world positions everywhere?
impl Course {
    // TODO: The individual count parts can probably be moved into their own methods (possibly impls of the specific subtypes)
    pub fn bill_of_material(&self) -> MurmelbahnResult<BillOfMaterial> {
        let mut context = CountContext::default();

        process_layer_construction_data(&self.layer_construction_data, &mut context)?;

        for pillar_construction_datum in self.pillar_construction_data.iter() {
            let lower_layer_height = context
                .retainer_heights
                .get(&pillar_construction_datum.lower_layer_id)
                .unwrap();
            let upper_layer_height = context
                .retainer_heights
                .get(&pillar_construction_datum.upper_layer_id)
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
            let mut small_stacker = upper_layer_height.lower - lower_layer_height.upper;
            trace!(
                "Pillar data: {} ({:?}) -> {} ({:?}): {}/{}",
                pillar_construction_datum.lower_layer_id,
                lower_layer_height,
                pillar_construction_datum.upper_layer_id,
                upper_layer_height,
                if small_stacker % 2 != 0 { 1 } else { 0 },
                if small_stacker % 2 != 0 {
                    (small_stacker - 1) / 2
                } else {
                    small_stacker / 2
                }
            );

            if small_stacker % 2 != 0 {
                context.small_stacker_count += 1;
                small_stacker -= 1;
            }
            context.large_stacker_count += small_stacker / 2;
        }

        let mut walls = HashMap::new();
        let mut balcony_count: usize = 0;
        for wall_construction_datum in self.wall_construction_data.iter() {
            balcony_count += wall_construction_datum.balcony_construction_datas.len();

            let tower_1_world_pos = &context.local_to_world_hex_vector(
                &wall_construction_datum.lower_stacker_tower_1_local_hex_pos,
                wall_construction_datum.lower_stacker_tower_1_retainer_id,
            );
            let tower_2_world_pos = &context.local_to_world_hex_vector(
                &wall_construction_datum.lower_stacker_tower_2_local_hex_pos,
                wall_construction_datum.lower_stacker_tower_2_retainer_id,
            );

            // Distance in fields -1 because we usually want to know how long a thing needs to be between
            // both cells (e.g. rails and walls)
            let distance = tower_1_world_pos.distance(&tower_2_world_pos) - 1;

            let wall_direction = hex_direction(&tower_1_world_pos, &tower_2_world_pos);

            trace!("Wall:\n{:#?}", wall_construction_datum);
            trace!(
                "Distance between walls: {}, wall direction: {:?}",
                distance,
                wall_direction
            );

            let wall = WallKind::straight_of_length(distance);
            let entry = walls.entry(wall).or_insert(0);
            *entry += 1;

            for balcony_construction_data in
                wall_construction_datum.balcony_construction_datas.iter()
            {
                let hex_vector = wall_construction_datum
                    .lower_stacker_tower_1_local_hex_pos
                    .hex_vector_in_distance(
                        &wall_direction,
                        balcony_construction_data.wall_coordinate.column,
                    );

                let target_direction =
                    wall_side_direction(&wall_direction, &balcony_construction_data.wall_side);
                let balcony_hex_vector = hex_vector.neighbor(&target_direction);

                let balcony_world_hex_vector = context.local_to_world_hex_vector(
                    &balcony_hex_vector,
                    wall_construction_datum.lower_stacker_tower_1_retainer_id,
                );

                context.retainer_positions.insert(
                    balcony_construction_data.retainer_id,
                    balcony_world_hex_vector.clone(),
                );

                if let Some(cell_construction_data) =
                    &balcony_construction_data.cell_construction_datas
                {
                    process_tree_node_data(
                        &cell_construction_data.tree_node_data,
                        &balcony_world_hex_vector,
                        0,
                        &mut context,
                    );
                }
            }
        }

        let mut rails = HashMap::new();
        for rail_construction_datum in self.rail_construction_data.iter() {
            // As far as I know `Straight` rails are the only ones that come in different length but are only
            // encoded as a single enum variant.
            let rail_kind = if &rail_construction_datum.rail_kind == &RailKind::Straight {
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
                    1 => bom::RailKind::StraightSmall,
                    2 => bom::RailKind::StraightMedium,
                    3 => bom::RailKind::StraightLarge,
                    _ => {
                        error!("Unrecognized length for small rail: {}", distance); // TODO: Abort?
                        panic!("No no no");
                    }
                }
            } else {
                bom::RailKind::try_from(rail_construction_datum.rail_kind.clone()).unwrap()
            };

            let entry = rails.entry(rail_kind).or_insert(0);
            *entry += 1;
        }

        // TODO: Impl into for CountContext
        Ok(BillOfMaterial {
            layers: context.layers,
            tiles: context.tile_counts,
            small_stacker: context.small_stacker_count,
            large_stacker: context.large_stacker_count,
            rails: rails,
            walls: walls,
            balconies: balcony_count,
        })
    }

    pub fn layer_by_id(&self, id: i32) -> Option<&LayerConstructionData> {
        self.layer_construction_data
            .iter()
            .find(|layer| layer.layer_id == id)
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
            process_tree_node_data(&cell.tree_node_data, &world_cell_position, height.upper, &mut context);
        }
    }

    trace!("Layer heights:\n{:#?}", context.retainer_heights);
    trace!(
        "Stackers not in Pillars: {} small, {} large",
        context.small_stacker_count,
        context.large_stacker_count
    );

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
    context.add_tile(&data.construction_data);

    context.add_small_stackers(data.construction_data.height_in_small_stacker);

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
                RetainerHeight::new(current_height, current_height + 14 + data.construction_data.height_in_small_stacker) // TODO: Maybe this 14 needs to be a 13 for reasons I don't understand
            );
            current_height += 14 + data.construction_data.height_in_small_stacker;
        } else {
            // All other things are 1 high I believe (i.e. DoubleBalcony)
            context.retainer_heights.insert(
                retainer_id,
                RetainerHeight::new(current_height, data.construction_data.height_in_small_stacker + current_height + 1)
            );
            current_height += data.construction_data.height_in_small_stacker;
        }
    }

    for child in data.children.iter() {
        process_tree_node_data(child, world_cell_position, current_height, context);
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
}

// TODO: Count double balconies
