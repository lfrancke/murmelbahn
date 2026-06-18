//! Throwaway diagnostic: dump SkyTrax layer positions, wall endpoints, and rail
//! endpoints with resolved world coordinates and computed spans, to understand
//! why the BOM counts differ from the app for a given course.

use murmelbahn_lib::app::BillOfMaterials;
use murmelbahn_lib::app::course::{Course, HexVector, SavedCourse};
use murmelbahn_lib::app::layer::TileTowerTreeNodeData;
use std::collections::HashMap;
use std::env;

/// Collect (tile kind, height_in_small_stacker, has_light_mode) for every node.
fn walk(node: &TileTowerTreeNodeData, acc: &mut Vec<(String, i32, bool)>) {
    acc.push((
        format!("{:?}", node.construction_data.kind),
        node.construction_data.height_in_small_stacker,
        node.construction_data.light_stone_color_mode.is_some(),
    ));
    for c in &node.children {
        walk(c, acc);
    }
}

fn main() {
    let path = env::args()
        .nth(1)
        .expect("usage: dump_skytrax <course-file>");
    let course = SavedCourse::from_path(&path).expect("parse course");

    // Full BOM our code currently produces, for direct comparison with the app.
    let bom = BillOfMaterials::from(SavedCourse::from_path(&path).unwrap().course);
    println!("== BOM TILES ==");
    let mut tiles: Vec<_> = bom.tiles.iter().collect();
    tiles.sort_by_key(|(k, _)| format!("{k:?}"));
    for (k, v) in tiles {
        println!("  {k:?} = {v}");
    }
    println!("== BOM WALLS ==");
    for (k, v) in &bom.walls {
        println!("  {k:?} = {v}");
    }
    println!(
        "== BOM RAILS: small={} medium={} large={} | other-kinds:",
        bom.rails_small, bom.rails_medium, bom.rails_large
    );
    let mut rk: Vec<_> = bom.rails.iter().collect();
    rk.sort_by_key(|(k, _)| format!("{k:?}"));
    for (k, v) in rk {
        println!("  {k:?} = {v}");
    }
    println!(
        "== BOM connectors={} balconies={} ==\n",
        bom.connectors, bom.balconies
    );

    let Course::SkyTrax(c) = course.course else {
        eprintln!("not a SkyTrax course");
        return;
    };

    // Layer (base plate) positions keyed by layer_id, exactly as the BOM seeds them.
    let mut pos: HashMap<i32, HexVector> = HashMap::new();
    println!("== LAYERS ==");
    for l in &c.layers {
        println!(
            "layer id={:<4} kind={:?} pos=({},{}) height={} cells={}",
            l.layer_id,
            l.layer_kind,
            l.position.x,
            l.position.y,
            l.small_stacker_height,
            l.cells.len()
        );
        pos.insert(l.layer_id, l.position.clone());
        let cells: Vec<(i32, i32)> = l
            .cells
            .iter()
            .map(|c| (c.local_hex_position.x, c.local_hex_position.y))
            .collect();
        println!("        cell locals: {cells:?}");
    }

    let resolve = |ret: i32, local: &HexVector| -> Option<HexVector> {
        pos.get(&ret)
            .map(|p| HexVector::new(local.x + p.x, local.y + p.y))
    };

    println!("\n== WALLS ({}) ==", c.walls.len());
    for (i, w) in c.walls.iter().enumerate() {
        let r1 = w.lower_stacker_tower_1_retainer_id;
        let r2 = w.lower_stacker_tower_2_retainer_id;
        let l1 = &w.lower_stacker_tower_1_local_hex_pos;
        let l2 = &w.lower_stacker_tower_2_local_hex_pos;
        let w1 = resolve(r1, l1);
        let w2 = resolve(r2, l2);
        let span = match (&w1, &w2) {
            (Some(a), Some(b)) => Some(a.distance(b) - 1),
            _ => None,
        };
        let same_plate = r1 == r2;
        println!(
            "wall {i:>2}: r1={r1} local=({},{}) -> {:?} | r2={r2} local=({},{}) -> {:?} | same_plate={same_plate} span={:?} balconies={}",
            l1.x,
            l1.y,
            w1.as_ref().map(|v| (v.x, v.y)),
            l2.x,
            l2.y,
            w2.as_ref().map(|v| (v.x, v.y)),
            span,
            w.balcony_construction_datas.len()
        );
    }

    println!("\n== RAILS ({}) ==", c.rails.len());
    for (i, r) in c.rails.iter().enumerate() {
        let r1 = r.exit_1_identifier.retainer_id;
        let r2 = r.exit_2_identifier.retainer_id;
        let l1 = &r.exit_1_identifier.cell_local_hex_pos;
        let l2 = &r.exit_2_identifier.cell_local_hex_pos;
        let w1 = resolve(r1, l1);
        let w2 = resolve(r2, l2);
        let span = match (&w1, &w2) {
            (Some(a), Some(b)) => Some(a.distance(b) - 1),
            _ => None,
        };
        println!(
            "rail {i:>2}: kind={:?} r1={r1} ({},{}) -> {:?} | r2={r2} ({},{}) -> {:?} | span={:?}",
            r.rail_kind,
            l1.x,
            l1.y,
            w1.as_ref().map(|v| (v.x, v.y)),
            l2.x,
            l2.y,
            w2.as_ref().map(|v| (v.x, v.y)),
            span
        );
    }

    // Stacker contributions from the tile tree: our add_stackers is called once
    // per node with its height_in_small_stacker. Tally by kind and total.
    let mut nodes: Vec<(String, i32, bool)> = Vec::new();
    for l in &c.layers {
        for cell in &l.cells {
            walk(&cell.tree_node_data, &mut nodes);
        }
    }
    let mut by_kind: HashMap<String, (i32, i32)> = HashMap::new(); // kind -> (count_with_height>0, sum_height)
    let mut total_tree_height = 0;
    let mut light_nodes = 0;
    for (k, h, light) in &nodes {
        if *light {
            light_nodes += 1;
        }
        if *h > 0 {
            let e = by_kind.entry(k.clone()).or_insert((0, 0));
            e.0 += 1;
            e.1 += h;
            total_tree_height += h;
        }
    }
    println!("\n== LIGHT: nodes with light_stone_color_mode set = {light_nodes} ==");
    for (k, h, light) in &nodes {
        if *light {
            println!("  light tile: {k} height={h}");
        }
    }

    // Print the full tile tree for any cell whose tree contains a LightBase, so
    // we can see what stacks above each base and at what heights.
    fn contains_light(n: &TileTowerTreeNodeData) -> bool {
        format!("{:?}", n.construction_data.kind) == "LightBase"
            || n.children.iter().any(contains_light)
    }
    fn print_tree(n: &TileTowerTreeNodeData, depth: usize) {
        println!(
            "{}{:?} height={} retainer={:?} light={}",
            "  ".repeat(depth + 1),
            n.construction_data.kind,
            n.construction_data.height_in_small_stacker,
            n.construction_data.retainer_id,
            n.construction_data.light_stone_color_mode.is_some(),
        );
        for c in &n.children {
            print_tree(c, depth + 1);
        }
    }
    println!("\n== TREES CONTAINING A LIGHTBASE ==");
    for l in &c.layers {
        for cell in &l.cells {
            if contains_light(&cell.tree_node_data) {
                println!(
                    "layer {} cell local ({},{}):",
                    l.layer_id, cell.local_hex_position.x, cell.local_hex_position.y
                );
                print_tree(&cell.tree_node_data, 0);
            }
        }
    }
    println!(
        "\n== TILE-TREE STACKER CONTRIB (total nodes={}, nodes w/ height>0, total small-stacker height={}) ==",
        nodes.len(),
        total_tree_height
    );
    let mut bk: Vec<_> = by_kind.iter().collect();
    bk.sort_by_key(|(k, _)| (*k).clone());
    for (k, (n, h)) in bk {
        println!("  {k}: {n} nodes, sum height={h}");
    }
    println!("  pillars: {}", c.pillars.len());
    println!("== PILLARS (lower -> upper layer ids; light-base retainers are 1024/1025/1026) ==");
    for (i, p) in c.pillars.iter().enumerate() {
        println!(
            "  pillar {i:>2}: {} ({},{}) -> {} ({},{})",
            p.lower_layer_id,
            p.lower_cell_local_position.x,
            p.lower_cell_local_position.y,
            p.upper_layer_id,
            p.upper_cell_local_position.x,
            p.upper_cell_local_position.y,
        );
    }

    println!("\n== CONNECTORS ({}) ==", c.connectors.len());
    for (i, con) in c.connectors.iter().enumerate() {
        println!(
            "conn {i:>2}: pos=({},{}) height={}",
            con.pos_x, con.pos_y, con.height
        );
    }
}
