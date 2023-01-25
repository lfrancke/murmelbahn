use crate::bom::{BillOfMaterial, LayerKind, RailKind, WallKind};
use crate::course::common::layer::TileKind;
use derive_more::{Deref, Display};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deref, Display)]
pub struct CourseCode(String);

impl CourseCode {
    pub fn new(code: String) -> CourseCode {
        CourseCode(code.to_uppercase())
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Count<T> {
    pub kind: T,
    pub count: u32,
}

#[derive(Debug, Serialize)]
pub struct GraviSheetOutput {
    pub course_code: String,
    pub empty_1: Option<i32>,
    pub empty_2: Option<i32>,
    pub layer_base: i32,
    pub layer_base_mini: i32,
    pub layer_base_mini_half: i32,
    pub layer_large: i32,
    pub layer_small: i32,

    pub marbles: i32,

    pub stacker_small: i32,
    pub stacker_large: i32,
    pub stacker_angled: i32,
    pub stacker_tower_closed: i32,
    pub stacker_tower_opened: i32,

    pub wall_short: i32,
    pub wall_medium: i32,
    pub wall_long: i32,

    pub balcony: i32,
    pub balcony_double: i32,

    pub rail_short: i32,
    pub rail_medium: i32,
    pub rail_long: i32,
    pub rail_narrow: i32,
    pub rail_slow: i32,
    pub rail_goal: i32,
    pub rail_bernoulli_small_straight: i32,
    pub rail_bernoulli_small_left: i32,
    pub rail_bernoulli_small_right: i32,
    pub rail_bernoulli: i32,
    pub rail_drop_hill: i32,
    pub rail_drop_valley: i32,
    pub catcher: i32,

    pub tile_starter: i32,
    pub tile_curve: i32,
    pub tile_multi_junction: i32,
    pub tile_curve_crossing: i32,
    pub tile_curve_crossing_straight: i32,
    pub tile_curve_big_double: i32,
    pub tile_curve_small_triple: i32,
    pub tile_curve_small_double: i32,
    pub tile_curve_ribbon: i32,
    pub tile_flexible_two_in_one_a: i32,
    pub tile_flexible_two_in_one_b: i32,
    pub tile_curve_small_two_in_one_a: i32,
    pub tile_curve_small_two_in_one_b: i32,
    pub tile_basic_closed: i32,
    pub tile_goal_basin: i32,
    pub tile_cross: i32,
    pub tile_three_way: i32,
    pub tile_two_way: i32,
    pub tile_switch_insert: i32,

    /// Called "Spiral" in the app
    pub tile_two_entrance_funnel: i32,
    pub tile_three_entrance_funnel: i32,
    pub tile_basic: i32,
    pub tile_drop: i32,
    pub tile_catch: i32,
    pub tile_splash: i32,
    pub tile_basic_straight: i32,
    pub tile_tunnel_straight: i32,
    pub tile_tunnel_curve: i32,
    pub tile_tunnel_switch: i32,
    pub rail_uturn: i32,

    pub tile_bridge: i32,
    pub tile_lift: i32,
    pub tile_catapult: i32,
    pub tile_color_swap: i32,
    pub tile_dipper: i32,
    pub rail_flextube: i32,
    pub tile_flip: i32,
    pub tile_hammer: i32,
    pub tile_jumper: i32,
    pub tile_loop: i32,
    pub tile_cannon: i32,
    /// Kaskade / Cascade
    pub tile_scoop: i32,
    pub tile_spinner: i32,
    pub tile_spiral_base: i32,
    pub tile_spiral_entrance: i32,
    pub tile_spiral_curve: i32,
    pub tile_tip_tube: i32,
    pub tile_trampoline: i32,
    pub tile_transfer: i32,
    pub tile_volcano: i32,
    pub tile_zipline: i32,

    pub tile_carousel: i32,
    pub tile_helix: i32,
    pub tile_mixer: i32,
    pub tile_splitter: i32,
    pub tile_turntable: i32,

    pub tile_controller: i32,
    pub tile_dome_starter: i32,
    pub tile_elevator: i32,
    pub tile_lever: i32,
    pub tile_dropdown_switch: i32,
    pub tile_finish_trigger: i32,
    pub tile_finish_arena: i32,
    pub tile_trigger: i32,
    pub tile_queue: i32,
}

impl From<BillOfMaterial> for GraviSheetOutput {
    fn from(bom: BillOfMaterial) -> Self {
        let trampolin_0 = bom.tile_kind(TileKind::Trampolin0);
        let trampolin_1 = bom.tile_kind(TileKind::Trampolin1);
        let trampolin_2 = bom.tile_kind(TileKind::Trampolin2);
        let total_trampolin = trampolin_0 + trampolin_1 + trampolin_2;

        let spiral_small = bom.tile_kind(TileKind::ScrewSmall);
        let spiral_medium = bom.tile_kind(TileKind::ScrewMedium);
        let spiral_large = bom.tile_kind(TileKind::ScrewLarge);
        let total_spirals = spiral_small + spiral_medium + spiral_large;

        GraviSheetOutput {
            course_code: "PLACEHOLDER".to_string(),
            empty_1: None,
            empty_2: None,
            layer_base: bom.layer_kind(LayerKind::Base),
            layer_base_mini: 0,
            layer_base_mini_half: 0,
            layer_large: bom.layer_kind(LayerKind::LargeClear),
            layer_small: bom.layer_kind(LayerKind::SmallClear),
            marbles: bom.marbles().1,
            stacker_small: bom.small_stacker,
            stacker_large: bom.large_stacker,
            stacker_angled: trampolin_1 + (2 * trampolin_2),
            stacker_tower_closed: bom.tile_kind(TileKind::StackerTowerClosed),
            stacker_tower_opened: bom.tile_kind(TileKind::StackerTowerOpened),
            wall_short: bom.wall_kind(WallKind::StraightSmall),
            wall_medium: bom.wall_kind(WallKind::StraightMedium),
            wall_long: bom.wall_kind(WallKind::StraightLarge),
            balcony: bom.balconies as i32,
            balcony_double: bom.tile_kind(TileKind::DoubleBalcony),
            rail_short: bom.rail_kind(RailKind::StraightSmall),
            rail_medium: bom.rail_kind(RailKind::StraightMedium),
            rail_long: bom.rail_kind(RailKind::StraightLarge),
            rail_narrow: bom.rail_kind(RailKind::Narrow),
            rail_slow: bom.rail_kind(RailKind::Slow),
            rail_goal: bom.tile_kind(TileKind::GoalRail),
            rail_bernoulli_small_straight: bom.rail_kind(RailKind::BernoulliSmallStraight),
            rail_bernoulli_small_left: bom.rail_kind(RailKind::BernoulliSmallLeft),
            rail_bernoulli_small_right: bom.rail_kind(RailKind::BernoulliSmallRight),
            rail_bernoulli: bom.rail_kind(RailKind::Bernoulli),
            rail_drop_hill: bom.rail_kind(RailKind::DropHill),
            rail_drop_valley: bom.rail_kind(RailKind::DropValley),
            catcher: 0, // Not available in the app

            tile_starter: bom.tile_kind(TileKind::Starter),
            tile_curve: bom.tile_kind(TileKind::Curve),
            tile_multi_junction: bom.tile_kind(TileKind::MultiJunction),
            tile_curve_crossing: bom.tile_kind(TileKind::CurveCrossing),
            tile_curve_crossing_straight: bom.tile_kind(TileKind::StraightCurveCrossing),
            tile_curve_big_double: bom.tile_kind(TileKind::DoubleBigCurve),
            tile_curve_small_triple: bom.tile_kind(TileKind::TripleSmallCurve),
            tile_curve_small_double: bom.tile_kind(TileKind::DoubleSmallCurve),
            tile_curve_ribbon: bom.tile_kind(TileKind::RibbonCurve),
            tile_flexible_two_in_one_a: bom.tile_kind(TileKind::FlexibleTwoInOneA),
            tile_flexible_two_in_one_b: bom.tile_kind(TileKind::FlexibleTwoInOneB),
            tile_curve_small_two_in_one_a: bom.tile_kind(TileKind::TwoInOneSmallCurveA),
            tile_curve_small_two_in_one_b: bom.tile_kind(TileKind::TwoInOneSmallCurveB),
            tile_basic_closed: bom.tile_kind(TileKind::GoalBasin),
            tile_goal_basin: bom.tile_kind(TileKind::GoalBasin),
            tile_cross: bom.tile_kind(TileKind::Cross),
            tile_three_way: bom.tile_kind(TileKind::Threeway),
            tile_two_way: bom.tile_kind(TileKind::TwoWay)
                + bom.tile_kind(TileKind::SwitchLeft)
                + bom.tile_kind(TileKind::SwitchRight),
            tile_switch_insert: bom.tile_kind(TileKind::SwitchLeft)
                + bom.tile_kind(TileKind::SwitchRight),
            tile_two_entrance_funnel: bom.tile_kind(TileKind::Spiral),
            tile_three_entrance_funnel: bom.tile_kind(TileKind::ThreeEntranceFunnel),
            tile_basic: bom.tile_kind(TileKind::Drop)
                + bom.tile_kind(TileKind::Catch)
                + bom.tile_kind(TileKind::Splash)
                + bom.tile_kind(TileKind::CurveTunnel)
                + bom.tile_kind(TileKind::SwitchTunnel),
            tile_drop: bom.tile_kind(TileKind::Drop),
            tile_catch: bom.tile_kind(TileKind::Catch),
            tile_splash: bom.tile_kind(TileKind::Splash),
            tile_basic_straight: bom.tile_kind(TileKind::StraightTunnel),
            tile_tunnel_straight: bom.tile_kind(TileKind::StraightTunnel),
            tile_tunnel_curve: bom.tile_kind(TileKind::CurveTunnel),
            tile_tunnel_switch: bom.tile_kind(TileKind::SwitchTunnel),
            rail_uturn: bom.rail_kind(RailKind::UTurn),
            tile_bridge: bom.tile_kind(TileKind::Bridge),
            tile_lift: bom.tile_kind(TileKind::LiftSmall) + bom.tile_kind(TileKind::LiftLarge),
            tile_catapult: bom.tile_kind(TileKind::Catapult),
            tile_color_swap: bom.tile_kind(TileKind::ColorSwapPreloaded)
                + bom.tile_kind(TileKind::ColorSwapEmpty),
            tile_dipper: bom.tile_kind(TileKind::DipperRight) + bom.tile_kind(TileKind::DipperLeft),
            rail_flextube: bom.rail_kind(RailKind::FlexTube0)
                + bom.rail_kind(RailKind::FlexTube60)
                + bom.rail_kind(RailKind::FlexTube120)
                + bom.rail_kind(RailKind::FlexTube180)
                + bom.rail_kind(RailKind::FlexTube240)
                + bom.rail_kind(RailKind::FlexTube300),
            tile_flip: bom.tile_kind(TileKind::Flip),
            tile_hammer: bom.tile_kind(TileKind::Hammer),
            tile_jumper: bom.tile_kind(TileKind::Jumper),
            tile_loop: bom.tile_kind(TileKind::Loop),
            tile_cannon: bom.tile_kind(TileKind::Cannon),
            tile_scoop: bom.tile_kind(TileKind::Cascade),
            tile_spinner: bom.tile_kind(TileKind::Spinner),
            tile_spiral_base: total_spirals,
            tile_spiral_entrance: total_spirals,
            tile_spiral_curve: spiral_medium * 5 + spiral_large * 12,
            tile_tip_tube: bom.tile_kind(TileKind::TipTube),
            tile_trampoline: total_trampolin,
            tile_transfer: bom.tile_kind(TileKind::Transfer),
            tile_volcano: bom.tile_kind(TileKind::Volcano),
            tile_zipline: bom.tile_kind(TileKind::ZiplineStart), // TODO: Warn if imbalanced
            tile_carousel: bom.tile_kind(TileKind::CarouselSameExits)
                + bom.tile_kind(TileKind::CarouselOffsetExits),
            tile_helix: bom.tile_kind(TileKind::Helix),
            tile_mixer: bom.tile_kind(TileKind::MixerSameExits)
                + bom.tile_kind(TileKind::MixerOffsetExits),
            tile_splitter: bom.tile_kind(TileKind::Splitter),
            tile_turntable: bom.tile_kind(TileKind::Turntable),
            tile_controller: 0, // Leaving out for now, hard to give a good number
            tile_dome_starter: bom.tile_kind(TileKind::DomeStarter),
            tile_elevator: bom.tile_kind(TileKind::Elevator),
            tile_lever: bom.tile_kind(TileKind::Lever),
            tile_dropdown_switch: bom.tile_kind(TileKind::DropdownSwitchRight)
                + bom.tile_kind(TileKind::DropdownSwitchLeft),
            tile_finish_trigger: bom.tile_kind(TileKind::FinishTrigger),
            tile_finish_arena: bom.tile_kind(TileKind::FinishArena),
            tile_trigger: bom.tile_kind(TileKind::Trigger),
            tile_queue: bom.tile_kind(TileKind::Queue),
        }
    }
}
