use std::collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::set::SetContentElement;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Inventory {
    #[serde(default)]
    pub sets: Vec<String>,

    #[serde(default)]
    pub extra_elements: HashMap<SetContentElement, i32>
}


/*
impl BillOfMaterial {
    pub fn add(&mut self, other: &BillOfMaterial) {
        merge_counter_maps(&mut self.layers, &other.layers);
        merge_counter_maps(&mut self.tiles, &other.tiles);
        merge_counter_maps(&mut self.rails, &other.rails);
    }

    pub fn can_be_built_with(&self, other: &BillOfMaterial) -> bool {
        is_superset_of_and_has_higher_counts(&self.layers, &other.layers)
            && is_superset_of_and_has_higher_counts(&self.tiles, &other.tiles)
            && is_superset_of_and_has_higher_counts(&self.rails, &other.rails)
    }
}

fn is_superset_of_and_has_higher_counts<T: Hash + Eq>(
    map_1: &HashMap<T, i32>,
    map_2: &HashMap<T, i32>,
) -> bool {
    //iterate over the items of map1
    for (key, value) in map_1.iter() {
        // check if the key exists in map2 and its value is greater or equal to map1
        if !map_2.contains_key(key) || *map_2.get(key).unwrap() < *value {
            return false;
        }
    }
    true
}

fn merge_counter_maps<T: Clone + Hash + Eq>(first: &mut HashMap<T, i32>, other: &HashMap<T, i32>) {
    for (key, value) in other {
        let entry = first.entry(key.clone()).or_insert(0);
        *entry += value
    }
}


 */
