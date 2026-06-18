//! Throwaway diagnostic: dump SkyTrax layer positions, wall endpoints, and rail
//! endpoints with resolved world coordinates and computed spans, to understand
//! why the BOM counts differ from the app for a given course.

use murmelbahn_lib::app::BillOfMaterials;
use murmelbahn_lib::app::course::{Course, HexVector, SavedCourse};
use std::collections::HashMap;
use std::env;

fn main() {
    let path = env::args().nth(1).expect("usage: dump_skytrax <course-file>");
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
    println!("== BOM connectors={} balconies={} ==\n", bom.connectors, bom.balconies);

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
            l1.x, l1.y, w1.as_ref().map(|v| (v.x, v.y)),
            l2.x, l2.y, w2.as_ref().map(|v| (v.x, v.y)),
            span, w.balcony_construction_datas.len()
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
            l1.x, l1.y, w1.as_ref().map(|v| (v.x, v.y)),
            l2.x, l2.y, w2.as_ref().map(|v| (v.x, v.y)),
            span
        );
    }

    println!("\n== CONNECTORS ({}) ==", c.connectors.len());
    for (i, con) in c.connectors.iter().enumerate() {
        println!("conn {i:>2}: pos=({},{}) height={}", con.pos_x, con.pos_y, con.height);
    }
}
