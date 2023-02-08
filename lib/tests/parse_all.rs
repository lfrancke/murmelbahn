use csv::ReaderBuilder;
use murmelbahn_lib::common::GraviSheetOutput;
use murmelbahn_lib::course::common::Course;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct GraviSheetInput {
    pub app_code: String,
    pub layer_base: Option<i32>,
    pub layer_base_mini: Option<i32>,
    pub layer_base_mini_half: Option<i32>,
    pub layer_large: Option<i32>,
    pub layer_small: Option<i32>,

    pub marbles: Option<i32>,

    pub stacker_small: Option<i32>,
    pub stacker_large: Option<i32>,
    pub stacker_angled: Option<i32>,
    pub stacker_tower_closed: Option<i32>,
    pub stacker_tower_opened: Option<i32>,

    pub wall_short: Option<i32>,
    pub wall_medium: Option<i32>,
    pub wall_long: Option<i32>,

    pub balcony: Option<i32>,
    pub balcony_double: Option<i32>,

    pub rail_short: Option<i32>,
    pub rail_medium: Option<i32>,
    pub rail_long: Option<i32>,
    pub rail_narrow: Option<i32>,
    pub rail_slow: Option<i32>,
    pub rail_goal: Option<i32>,
    pub rail_bernoulli_small_straight: Option<i32>,
    pub rail_bernoulli_small_left: Option<i32>,
    pub rail_bernoulli_small_right: Option<i32>,
    pub rail_bernoulli: Option<i32>,
    pub rail_drop_hill: Option<i32>,
    pub rail_drop_valley: Option<i32>,
    pub catcher: Option<i32>, // TODO

    pub tile_starter: Option<i32>,
    pub tile_curve: Option<i32>,
    pub tile_multi_junction: Option<i32>,
    pub tile_curve_crossing: Option<i32>,
    pub tile_curve_crossing_straight: Option<i32>,
    pub tile_curve_big_double: Option<i32>,
    pub tile_curve_small_triple: Option<i32>,
    pub tile_curve_small_double: Option<i32>,
    pub tile_curve_ribbon: Option<i32>,
    pub tile_flexible_two_in_one_a: Option<i32>,
    pub tile_flexible_two_in_one_b: Option<i32>,
    pub tile_curve_small_two_in_one_a: Option<i32>,
    pub tile_curve_small_two_in_one_b: Option<i32>,
    pub tile_basic_closed: Option<i32>,
    pub tile_goal_basin: Option<i32>,
    pub tile_cross: Option<i32>,
    pub tile_three_way: Option<i32>,
    pub tile_two_way: Option<i32>,
    pub tile_switch_insert: Option<i32>,

    /// Called "Spiral" in the app
    pub tile_two_entrance_funnel: Option<i32>,
    pub tile_three_entrance_funnel: Option<i32>,
    pub tile_basic: Option<i32>,
    pub tile_drop: Option<i32>,
    pub tile_catch: Option<i32>,
    pub tile_splash: Option<i32>,
    pub tile_basic_straight: Option<i32>,
    pub tile_tunnel_straight: Option<i32>,
    pub tile_tunnel_curve: Option<i32>,
    pub tile_tunnel_switch: Option<i32>,
    pub rail_uturn: Option<i32>,

    pub tile_bridge: Option<i32>,
    pub tile_lift: Option<i32>, // TODO: GraviSheet only has a single lift
    pub tile_catapult: Option<i32>,
    pub tile_color_swap: Option<i32>,
    pub tile_dipper: Option<i32>,
    pub rail_flextube: Option<i32>,
    pub tile_flip: Option<i32>,
    pub tile_hammer: Option<i32>,
    pub tile_jumper: Option<i32>,
    pub tile_loop: Option<i32>,
    pub tile_cannon: Option<i32>,
    /// Kaskade / Cascade
    pub tile_scoop: Option<i32>,
    pub tile_spinner: Option<i32>,
    pub tile_spiral_base: Option<i32>,
    pub tile_spiral_entrance: Option<i32>,
    pub tile_spiral_curve: Option<i32>,
    pub tile_tip_tube: Option<i32>,
    pub tile_trampoline: Option<i32>,
    pub tile_transfer: Option<i32>,
    pub tile_volcano: Option<i32>,
    pub tile_zipline: Option<i32>, // App noch mit ziplinestart und ziplineende

    pub tile_carousel: Option<i32>,
    pub tile_helix: Option<i32>,
    pub tile_mixer: Option<i32>,
    pub tile_splitter: Option<i32>,
    pub tile_turntable: Option<i32>,

    pub tile_controller: Option<i32>,
    pub tile_dome_starter: Option<i32>,
    pub tile_elevator: Option<i32>,
    pub tile_lever: Option<i32>,
    pub tile_dropdown_switch: Option<i32>,
    pub tile_finish_trigger: Option<i32>,
    pub tile_finish_arena: Option<i32>,
    pub tile_trigger: Option<i32>,
    pub tile_queue: Option<i32>,
}

macro_rules! generate_compare {
    ($field_name:ident, $self:ident, $output:ident) => {
        compare(
            $self.$field_name,
            $output.$field_name,
            stringify!($field_name),
        )
    };
}

impl GraviSheetInput {
    pub fn compare_with_output(&self, output: &GraviSheetOutput) {
        compare(self.layer_base, output.layer_base, "layer_base");
        // compare(self.layer_base_mini, output.layer_base_mini); IGNORE
        // compare(self.layer_base_mini_half, output.layer_base_mini_half); IGNORE
        compare(self.layer_large, output.layer_large, "layer_large");
        compare(self.layer_small, output.layer_small, "layer_small");

        // compare(self.marbles, output.marbles, "marbles"); TODO

        compare(self.stacker_small, output.stacker_small, "stacker_small");
        compare(self.stacker_large, output.stacker_large, "stacker_large");
        compare(self.stacker_angled, output.stacker_angled, "stacker_angled");
        compare(
            self.stacker_tower_closed,
            output.stacker_tower_closed,
            "stacker_tower_closed",
        );
        compare(
            self.stacker_tower_opened,
            output.stacker_tower_opened,
            "stacker_tower_opened",
        );

        generate_compare!(wall_short, self, output);
        generate_compare!(wall_medium, self, output);
        generate_compare!(wall_long, self, output);

        generate_compare!(balcony, self, output);
        generate_compare!(balcony_double, self, output);

        generate_compare!(rail_short, self, output);
        generate_compare!(rail_medium, self, output);
        generate_compare!(rail_long, self, output);
        generate_compare!(rail_narrow, self, output);
        generate_compare!(rail_slow, self, output);
        generate_compare!(rail_goal, self, output);
        generate_compare!(rail_bernoulli_small_straight, self, output);
        generate_compare!(rail_bernoulli_small_left, self, output);
        generate_compare!(rail_bernoulli_small_right, self, output);
        generate_compare!(rail_bernoulli, self, output);
        generate_compare!(rail_bernoulli, self, output);
        generate_compare!(rail_drop_hill, self, output);
        generate_compare!(rail_drop_valley, self, output);
        //generate_compare!(catcher, self, output); TODO

        generate_compare!(tile_starter, self, output);
        generate_compare!(tile_curve, self, output);
        generate_compare!(tile_multi_junction, self, output);
        generate_compare!(tile_curve_crossing, self, output);
        generate_compare!(tile_curve_crossing_straight, self, output);
        generate_compare!(tile_curve_big_double, self, output);
        generate_compare!(tile_curve_small_triple, self, output);
        generate_compare!(tile_curve_small_double, self, output);
        generate_compare!(tile_curve_ribbon, self, output);
        generate_compare!(tile_flexible_two_in_one_a, self, output);
        generate_compare!(tile_flexible_two_in_one_b, self, output);
        generate_compare!(tile_curve_small_two_in_one_a, self, output);
        generate_compare!(tile_curve_small_two_in_one_b, self, output);
        // generate_compare!(tile_basic_closed, self, output); TODO
        generate_compare!(tile_goal_basin, self, output);
        generate_compare!(tile_cross, self, output);
        generate_compare!(tile_three_way, self, output);
        generate_compare!(tile_two_way, self, output);
        generate_compare!(tile_switch_insert, self, output);
        generate_compare!(tile_two_entrance_funnel, self, output);
        generate_compare!(tile_three_entrance_funnel, self, output);
        // generate_compare!(tile_basic, self, output); TODO
        generate_compare!(tile_drop, self, output);
        generate_compare!(tile_catch, self, output);
        generate_compare!(tile_splash, self, output);
        generate_compare!(tile_basic_straight, self, output);
        generate_compare!(tile_tunnel_straight, self, output);
        generate_compare!(tile_tunnel_curve, self, output);
        generate_compare!(tile_tunnel_switch, self, output);
        generate_compare!(rail_uturn, self, output);
        generate_compare!(tile_bridge, self, output);
        generate_compare!(tile_lift, self, output);
        generate_compare!(tile_catapult, self, output);
        generate_compare!(tile_color_swap, self, output);
        generate_compare!(tile_dipper, self, output);
        generate_compare!(rail_flextube, self, output);
        generate_compare!(tile_flip, self, output);
        generate_compare!(tile_hammer, self, output);
        generate_compare!(tile_jumper, self, output);
        generate_compare!(tile_loop, self, output);
        generate_compare!(tile_cannon, self, output);
        generate_compare!(tile_scoop, self, output);
        generate_compare!(tile_spinner, self, output);
        generate_compare!(tile_spiral_base, self, output);
        generate_compare!(tile_spiral_entrance, self, output);
        generate_compare!(tile_spiral_curve, self, output);
        generate_compare!(tile_tip_tube, self, output);
        generate_compare!(tile_trampoline, self, output);
        generate_compare!(tile_transfer, self, output);
        generate_compare!(tile_volcano, self, output);
        generate_compare!(tile_zipline, self, output);
        generate_compare!(tile_carousel, self, output);
        generate_compare!(tile_helix, self, output);
        generate_compare!(tile_mixer, self, output);
        generate_compare!(tile_splitter, self, output);
        generate_compare!(tile_turntable, self, output);
        generate_compare!(tile_controller, self, output);
        generate_compare!(tile_dome_starter, self, output);
        generate_compare!(tile_elevator, self, output);
        generate_compare!(tile_lever, self, output);
        generate_compare!(tile_dropdown_switch, self, output);
        generate_compare!(tile_finish_trigger, self, output);
        generate_compare!(tile_finish_arena, self, output);
        generate_compare!(tile_trigger, self, output);
        generate_compare!(tile_queue, self, output);
    }
}

fn compare(a: Option<i32>, b: i32, msg: &str) {
    a.map(|val| assert_eq!(val, b, "{}", msg));
}

#[test]
fn test_parse_all() {
    /*
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("tests/gravisheet82.csv");

    let mut reader = ReaderBuilder::new()
           .has_headers(false)
        .from_path(test_data).unwrap();

    let mut input_map = HashMap::new();
    for result in reader.deserialize().skip(1) {
        let record: GraviSheetInput = result.unwrap();
        input_map.insert(record.app_code.clone(), record);
    }

    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("tests/test-data");
    let paths = fs::read_dir(test_data).unwrap();
    for dir_entry in paths.flatten() {
        let string = fs::read(dir_entry.path()).unwrap();
        print!("Parsing {:?} ", dir_entry.path());
        let course = murmelbahn_lib::course::common::SavedCourse::from_bytes(&string).unwrap();
        println!("was succesful: {:?}", course.header.version);
        match course.course {
            Course::ZiplineAdded2019(_) => {}
            Course::Power2022(course) | Course::Pro2020(course) => {
                let bom = course.bill_of_material().unwrap();
                let app_code = dir_entry.path().file_stem().unwrap().to_ascii_uppercase();
                match input_map.get(&app_code.into_string().unwrap()) {
                    None => {}
                    Some(record) => {
                        record.compare_with_output(&bom.into());
                    }
                }



            }
        }

    }

     */
}
