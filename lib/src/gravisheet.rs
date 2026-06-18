//! Model used to output a course's piece counts in the column order of the
//! GraviSheet "Tracks to Build" tab, so the result can be pasted into the sheet.
//! Field order must match that tab's columns exactly (the value row is what gets
//! pasted). A few columns the app cannot fill are emitted empty as spacers.
use crate::app::BillOfMaterials;
use crate::app::layer::{LayerKind, TileKind};
use crate::app::rail::RailKind;
use crate::app::wall::WallKind;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GraviSheetOutput {
    // Leading columns for the backend paste sheet: the course title, one spacer,
    // the app code, then two spacers before the platforms.
    pub title: String,
    pub empty_video: Option<i32>,
    pub course_code: String,
    pub empty_spacer_1: Option<i32>,
    pub empty_spacer_2: Option<i32>,

    // Platforms
    pub layer_base: Option<i32>,
    pub layer_base_mini: Option<i32>,
    pub layer_base_mini_half: Option<i32>,
    pub layer_base_micro: Option<i32>,
    pub layer_large: Option<i32>,
    pub layer_small: Option<i32>,

    pub marbles: Option<i32>,

    pub stacker_small: Option<i32>,
    pub stacker_large: Option<i32>,
    pub lightstacker_small: Option<i32>,
    pub lightstacker_large: Option<i32>,
    pub stacker_angled: Option<i32>,
    pub stacker_tower_closed: Option<i32>,
    pub stacker_tower_opened: Option<i32>,

    pub wall_short: Option<i32>,
    pub wall_medium: Option<i32>,
    pub wall_long: Option<i32>,

    pub balcony: Option<i32>,
    pub balcony_double: Option<i32>,

    pub rail_small: Option<i32>,
    pub rail_medium: Option<i32>,
    pub rail_large: Option<i32>,
    pub rail_narrow: Option<i32>,
    pub rail_slow: Option<i32>,
    pub rail_goal: Option<i32>,
    pub rail_bernoulli_small_straight: Option<i32>,
    pub rail_bernoulli_small_left: Option<i32>,
    pub rail_bernoulli_small_right: Option<i32>,
    pub rail_bernoulli: Option<i32>,
    pub rail_drop_hill: Option<i32>,
    pub rail_drop_valley: Option<i32>,
    pub catcher: Option<i32>,

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

    // Advent slope tiles (BE..BL in the sheet).
    pub slope_2in1: Option<i32>,
    pub slope_3in1: Option<i32>,
    pub slope_curve_1: Option<i32>,
    pub slope_curve_2: Option<i32>,
    pub slope_double_curve: Option<i32>,
    pub slope_triple_curve: Option<i32>,
    pub slope_crossing: Option<i32>,
    pub slope_jump_crossing: Option<i32>,

    pub tile_basic_closed: Option<i32>,
    pub tile_goal_basin: Option<i32>,
    pub tile_cross: Option<i32>,
    pub tile_three_way: Option<i32>,
    pub tile_two_way: Option<i32>,
    pub tile_switch_insert: Option<i32>,
    /// Called "Spiral" in the app
    pub tile_two_entrance_funnel: Option<i32>,
    pub tile_three_entrance_funnel: Option<i32>,
    pub tile_space_tube: Option<i32>,
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
    pub tile_lift: Option<i32>,
    pub tile_catapult: Option<i32>,
    pub tile_color_swap: Option<i32>,
    pub tile_dipper: Option<i32>,
    pub rail_flextube: Option<i32>,
    pub tile_flip: Option<i32>,
    pub tile_hammer: Option<i32>,
    pub tile_jumper: Option<i32>,
    pub tile_loop: Option<i32>,
    pub tile_cannon: Option<i32>,
    pub tile_vertical_cannon: Option<i32>,
    /// Kaskade / Cascade
    pub tile_scoop: Option<i32>,
    pub tile_spinner: Option<i32>,
    pub tile_spiral_base: Option<i32>,
    pub tile_spiral_entrance: Option<i32>,
    pub tile_spiral_curve: Option<i32>,
    pub tile_tip_tube: Option<i32>,
    pub tile_trampoline: Option<i32>,
    pub tile_transfer: Option<i32>,
    /// Spacer for the Turbo Lift, which the app cannot count yet.
    pub empty_turbo_lift: Option<i32>,
    pub tile_volcano: Option<i32>,
    pub tile_zipline: Option<i32>,

    pub tile_carousel: Option<i32>,
    pub tile_helix: Option<i32>,
    pub tile_mixer: Option<i32>,
    pub releaser: Option<i32>,
    pub tile_splitter: Option<i32>,
    pub tile_turntable: Option<i32>,

    pub empty_connect: Option<i32>,
    pub tile_controller: Option<i32>,
    pub tile_dome_starter: Option<i32>,
    pub tile_dropdown_switch: Option<i32>,
    /// Spacer for the Dynamo, which the app cannot count yet.
    pub empty_dynamo: Option<i32>,
    pub tile_electric_cannon: Option<i32>,
    pub tile_elevator: Option<i32>,
    pub tile_finish_arena: Option<i32>,
    pub tile_finish_trigger: Option<i32>,
    pub tile_lever: Option<i32>,
    pub lightbase: Option<i32>,
    pub tile_queue: Option<i32>,
    pub empty_sound: Option<i32>,
    pub tile_trigger: Option<i32>,

    // SkyTrax tiles, then the 6 SkyTrax slide rails, then the connector, in the
    // sheet's column order. The slides sit between the 3-in-1 and the connector.
    pub skytrax_2in1_left: Option<i32>,
    pub skytrax_2in1_right: Option<i32>,
    pub skytrax_120_catch_drop_60_left: Option<i32>,
    pub skytrax_120_catch_drop_60_right: Option<i32>,
    pub skytrax_180_catch_60_60: Option<i32>,
    pub skytrax_crossing_catch_drop: Option<i32>,
    pub skytrax_curve_catch: Option<i32>,
    pub skytrax_curve_drop: Option<i32>,
    pub skytrax_finish: Option<i32>,
    pub skytrax_gt_drop: Option<i32>,
    pub skytrax_hs5: Option<i32>,
    pub skytrax_hs20: Option<i32>,
    pub skytrax_multi_catch_drop: Option<i32>,
    pub skytrax_multi_catcher: Option<i32>,
    pub skytrax_spiral_120_catch_left: Option<i32>,
    pub skytrax_spiral_120_catch_right: Option<i32>,
    pub skytrax_spiral_180_catch_left: Option<i32>,
    pub skytrax_spiral_180_catch_right: Option<i32>,
    pub skytrax_spiral_240_catch_left: Option<i32>,
    pub skytrax_spiral_240_catch_right: Option<i32>,
    pub skytrax_spiral_300_left: Option<i32>,
    pub skytrax_spiral_300_right: Option<i32>,
    pub skytrax_starter: Option<i32>,
    pub skytrax_3in1: Option<i32>,

    // SkyTrax slide rails (ET..EY in the sheet). The app stores them as rails
    // hung onto placed tiles, so they come from the rail counts, not the tiles.
    pub skytrax_bernoulli_left: Option<i32>,
    pub skytrax_bernoulli_right: Option<i32>,
    pub skytrax_slide_60_left: Option<i32>,
    pub skytrax_slide_60_right: Option<i32>,
    pub skytrax_slide_120_left: Option<i32>,
    pub skytrax_slide_120_right: Option<i32>,

    pub skytrax_connector: Option<i32>,
}

impl From<BillOfMaterials> for GraviSheetOutput {
    fn from(bom: BillOfMaterials) -> Self {
        let trampolin_0 = bom.tile_kind(TileKind::Trampolin0);
        let trampolin_1 = bom.tile_kind(TileKind::Trampolin1);
        let trampolin_2 = bom.tile_kind(TileKind::Trampolin2);
        let total_trampolin = option_adder_helper(vec![trampolin_0, trampolin_1, trampolin_2]);
        let stacker_angled = trampolin_1.unwrap_or(0) + (2 * trampolin_2.unwrap_or(0));
        let stacker_angled = i32_to_option(stacker_angled);

        let spiral_small = bom.tile_kind(TileKind::ScrewSmall);
        let spiral_medium = bom.tile_kind(TileKind::ScrewMedium);
        let spiral_large = bom.tile_kind(TileKind::ScrewLarge);
        let total_spirals = option_adder_helper(vec![spiral_small, spiral_medium, spiral_large]);
        let tile_spiral_curve = spiral_medium.unwrap_or(0) * 5 + spiral_large.unwrap_or(0) * 12;
        let tile_spiral_curve = i32_to_option(tile_spiral_curve);

        GraviSheetOutput {
            title: "PLACEHOLDER".to_string(),
            course_code: "PLACEHOLDER".to_string(),
            empty_video: None,
            empty_spacer_1: None,
            empty_spacer_2: None,

            layer_base: bom.layer_kind(LayerKind::BaseLayerPiece),
            layer_base_mini: None,
            layer_base_mini_half: None,
            layer_base_micro: None,
            layer_large: bom.layer_kind(LayerKind::LargeLayer),
            layer_small: bom.layer_kind(LayerKind::SmallLayer),
            marbles: i32_to_option(bom.marbles()),
            stacker_small: bom.tile_kind(TileKind::StackerSmall),
            stacker_large: bom.tile_kind(TileKind::Stacker),
            lightstacker_small: bom.tile_kind(TileKind::LightStackerSmall),
            lightstacker_large: bom.tile_kind(TileKind::LightStacker),
            stacker_angled,
            stacker_tower_closed: bom.tile_kind(TileKind::StackerTowerClosed),
            stacker_tower_opened: bom.tile_kind(TileKind::StackerTowerOpened),
            wall_short: bom.wall_kind(WallKind::StraightSmall),
            wall_medium: bom.wall_kind(WallKind::StraightMedium),
            wall_long: bom.wall_kind(WallKind::StraightLarge),
            balcony: i32_to_option(bom.balconies),
            balcony_double: bom.tile_kind(TileKind::DoubleBalcony),
            rail_small: i32_to_option(bom.rails_small),
            rail_medium: i32_to_option(bom.rails_medium),
            rail_large: i32_to_option(bom.rails_large),
            rail_narrow: bom.rail_kind(RailKind::Narrow),
            rail_slow: bom.rail_kind(RailKind::Slow),
            rail_goal: bom.tile_kind(TileKind::GoalRail),
            rail_bernoulli_small_straight: bom.rail_kind(RailKind::BernoulliSmallStraight),
            rail_bernoulli_small_left: bom.rail_kind(RailKind::BernoulliSmallLeft),
            rail_bernoulli_small_right: bom.rail_kind(RailKind::BernoulliSmallRight),
            rail_bernoulli: bom.rail_kind(RailKind::Bernoulli),
            rail_drop_hill: bom.rail_kind(RailKind::DropHill),
            rail_drop_valley: bom.rail_kind(RailKind::DropValley),
            catcher: None, // Not available in the app

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

            slope_2in1: bom.tile_kind(TileKind::K2In1Slope),
            slope_3in1: bom.tile_kind(TileKind::K3In1Slope),
            slope_curve_1: bom.tile_kind(TileKind::KCurveSlope1),
            slope_curve_2: bom.tile_kind(TileKind::KCurveSlope2),
            slope_double_curve: bom.tile_kind(TileKind::K120DoubleCurveSlope),
            slope_triple_curve: bom.tile_kind(TileKind::KBoomerangSlope),
            slope_crossing: bom.tile_kind(TileKind::KCrossingSlope),
            slope_jump_crossing: bom.tile_kind(TileKind::KJumpCrossingSlope),

            tile_basic_closed: bom.tile_kind(TileKind::GoalBasin),
            tile_goal_basin: bom.tile_kind(TileKind::GoalBasin),
            tile_cross: bom.tile_kind(TileKind::Cross),
            tile_three_way: bom.tile_kind(TileKind::Threeway),
            tile_two_way: option_adder_helper(vec![
                bom.tile_kind(TileKind::TwoWay),
                bom.tile_kind(TileKind::SwitchLeft),
                bom.tile_kind(TileKind::SwitchRight),
            ]),
            tile_switch_insert: option_adder_helper(vec![
                bom.tile_kind(TileKind::SwitchLeft),
                bom.tile_kind(TileKind::SwitchRight),
            ]),
            tile_two_entrance_funnel: bom.tile_kind(TileKind::Spiral),
            tile_three_entrance_funnel: option_adder_helper(vec![
                bom.tile_kind(TileKind::ThreeEntranceFunnel),
                bom.tile_kind(TileKind::MixerSameExits),
                bom.tile_kind(TileKind::MixerOffsetExits),
            ]),
            tile_space_tube: option_adder_helper(vec![
                bom.tile_kind(TileKind::SpaceTubeAligned),
                bom.tile_kind(TileKind::SpaceTubeUnaligned),
            ]),
            tile_basic: option_adder_helper(vec![
                bom.tile_kind(TileKind::Drop),
                bom.tile_kind(TileKind::Catch),
                bom.tile_kind(TileKind::Splash),
                bom.tile_kind(TileKind::CurveTunnel),
                bom.tile_kind(TileKind::SwitchTunnel),
            ]),
            tile_drop: bom.tile_kind(TileKind::Drop),
            tile_catch: bom.tile_kind(TileKind::Catch),
            tile_splash: bom.tile_kind(TileKind::Splash),
            tile_basic_straight: bom.tile_kind(TileKind::StraightTunnel),
            tile_tunnel_straight: bom.tile_kind(TileKind::StraightTunnel),
            tile_tunnel_curve: bom.tile_kind(TileKind::CurveTunnel),
            tile_tunnel_switch: bom.tile_kind(TileKind::SwitchTunnel),
            rail_uturn: bom.rail_kind(RailKind::UTurn),
            tile_bridge: bom.tile_kind(TileKind::Bridge),
            tile_lift: option_adder_helper(vec![
                bom.tile_kind(TileKind::LiftSmall),
                bom.tile_kind(TileKind::LiftLarge),
            ]),
            tile_catapult: bom.tile_kind(TileKind::Catapult),
            tile_color_swap: option_adder_helper(vec![
                bom.tile_kind(TileKind::ColorSwapPreloaded),
                bom.tile_kind(TileKind::ColorSwapEmpty),
            ]),
            tile_dipper: option_adder_helper(vec![
                bom.tile_kind(TileKind::DipperRight),
                bom.tile_kind(TileKind::DipperLeft),
            ]),
            rail_flextube: option_adder_helper(vec![
                bom.rail_kind(RailKind::FlexTube0),
                bom.rail_kind(RailKind::FlexTube60),
                bom.rail_kind(RailKind::FlexTube120),
                bom.rail_kind(RailKind::FlexTube180),
                bom.rail_kind(RailKind::FlexTube240),
                bom.rail_kind(RailKind::FlexTube300),
            ]),
            tile_flip: bom.tile_kind(TileKind::Flip),
            tile_hammer: bom.tile_kind(TileKind::Hammer),
            tile_jumper: bom.tile_kind(TileKind::Jumper),
            tile_loop: bom.tile_kind(TileKind::Loop),
            tile_cannon: bom.tile_kind(TileKind::Cannon),
            tile_vertical_cannon: option_adder_helper(vec![
                bom.tile_kind(TileKind::VerticalCannon0),
                bom.tile_kind(TileKind::VerticalCannon60),
                bom.tile_kind(TileKind::VerticalCannon120),
                bom.tile_kind(TileKind::VerticalCannon180),
                bom.tile_kind(TileKind::VerticalCannon240),
                bom.tile_kind(TileKind::VerticalCannon300),
            ]),
            tile_scoop: bom.tile_kind(TileKind::Cascade),
            tile_spinner: bom.tile_kind(TileKind::Spinner),
            tile_spiral_base: total_spirals,
            tile_spiral_entrance: total_spirals,
            tile_spiral_curve,
            tile_tip_tube: bom.tile_kind(TileKind::TipTube),
            tile_trampoline: total_trampolin,
            tile_transfer: bom.tile_kind(TileKind::Transfer),
            empty_turbo_lift: None,
            tile_volcano: bom.tile_kind(TileKind::Volcano),
            tile_zipline: bom.tile_kind(TileKind::ZiplineStart),
            tile_carousel: option_adder_helper(vec![
                bom.tile_kind(TileKind::CarouselSameExits),
                bom.tile_kind(TileKind::CarouselOffsetExits),
            ]),
            tile_helix: bom.tile_kind(TileKind::Helix),
            tile_mixer: option_adder_helper(vec![
                bom.tile_kind(TileKind::MixerSameExits),
                bom.tile_kind(TileKind::MixerOffsetExits),
            ]),
            releaser: option_adder_helper(vec![
                bom.tile_kind(TileKind::Releaser1),
                bom.tile_kind(TileKind::Releaser2),
                bom.tile_kind(TileKind::Releaser3),
                bom.tile_kind(TileKind::Releaser4),
            ]),
            tile_splitter: bom.tile_kind(TileKind::Splitter),
            tile_turntable: bom.tile_kind(TileKind::Turntable),
            empty_connect: None,
            tile_controller: None, // Leaving out for now, hard to give a good number
            tile_dome_starter: bom.tile_kind(TileKind::DomeStarter),
            tile_dropdown_switch: option_adder_helper(vec![
                bom.tile_kind(TileKind::DropdownSwitchRight),
                bom.tile_kind(TileKind::DropdownSwitchLeft),
            ]),
            empty_dynamo: None,
            tile_electric_cannon: bom.tile_kind(TileKind::ElectricCannon),
            tile_elevator: bom.tile_kind(TileKind::Elevator),
            tile_finish_arena: bom.tile_kind(TileKind::FinishArena),
            tile_finish_trigger: bom.tile_kind(TileKind::FinishTrigger),
            tile_lever: bom.tile_kind(TileKind::Lever),
            lightbase: bom.tile_kind(TileKind::LightBase),
            tile_queue: bom.tile_kind(TileKind::Queue),
            empty_sound: None,
            tile_trigger: bom.tile_kind(TileKind::Trigger),

            skytrax_2in1_left: bom.tile_kind(TileKind::Kst2In1L),
            skytrax_2in1_right: bom.tile_kind(TileKind::Kst2In1R),
            skytrax_120_catch_drop_60_left: bom.tile_kind(TileKind::Kst120CatchDrop60L),
            skytrax_120_catch_drop_60_right: bom.tile_kind(TileKind::Kst120CatchDrop60R),
            skytrax_180_catch_60_60: bom.tile_kind(TileKind::Kst180Catch6060),
            skytrax_crossing_catch_drop: bom.tile_kind(TileKind::KstCrossingCatchDrop),
            skytrax_curve_catch: bom.tile_kind(TileKind::KstCurveCatch),
            skytrax_curve_drop: bom.tile_kind(TileKind::KstCurveDrop),
            skytrax_finish: bom.tile_kind(TileKind::KstFinish),
            skytrax_gt_drop: bom.tile_kind(TileKind::KstGtDrop),
            skytrax_hs5: bom.tile_kind(TileKind::KstHs5),
            skytrax_hs20: bom.tile_kind(TileKind::KstHs20),
            skytrax_multi_catch_drop: bom.tile_kind(TileKind::KstMultiCatchDrop),
            skytrax_multi_catcher: bom.tile_kind(TileKind::KstMultiCatcher),
            skytrax_spiral_120_catch_left: bom.tile_kind(TileKind::KstSpiral120CatchDropCatchL),
            skytrax_spiral_120_catch_right: bom.tile_kind(TileKind::KstSpiral120CatchDropCatchR),
            skytrax_spiral_180_catch_left: bom.tile_kind(TileKind::KstSpiral180CatchDropL),
            skytrax_spiral_180_catch_right: bom.tile_kind(TileKind::KstSpiral180CatchDropR),
            skytrax_spiral_240_catch_left: bom.tile_kind(TileKind::KstSpiral240CatchL),
            skytrax_spiral_240_catch_right: bom.tile_kind(TileKind::KstSpiral240CatchR),
            skytrax_spiral_300_left: bom.tile_kind(TileKind::KstSpiral300L),
            skytrax_spiral_300_right: bom.tile_kind(TileKind::KstSpiral300R),
            skytrax_starter: bom.tile_kind(TileKind::KstStarter),
            skytrax_3in1: bom.tile_kind(TileKind::Kst3In1),
            skytrax_bernoulli_left: bom.rail_kind(RailKind::KstBernoulliL),
            skytrax_bernoulli_right: bom.rail_kind(RailKind::KstBernoulliR),
            skytrax_slide_60_left: bom.rail_kind(RailKind::KstSlide60L),
            skytrax_slide_60_right: bom.rail_kind(RailKind::KstSlide60R),
            skytrax_slide_120_left: bom.rail_kind(RailKind::KstSlide120L),
            skytrax_slide_120_right: bom.rail_kind(RailKind::KstSlide120R),
            skytrax_connector: i32_to_option(bom.connectors),
        }
    }
}

fn i32_to_option(value: i32) -> Option<i32> {
    if value == 0 { None } else { Some(value) }
}

fn option_adder_helper(vec: Vec<Option<i32>>) -> Option<i32> {
    let mut sum = 0;
    for item in vec {
        sum += item.unwrap_or(0);
    }

    i32_to_option(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::rail::RailKind;

    /// The SkyTrax slide rails come from the rail counts and land in their own
    /// columns between the 3-in-1 and the connector. Mirrors the bill of
    /// materials reported for course 3SHTTIDZQ5.
    #[test]
    fn skytrax_slide_rails_populate_their_columns() {
        let mut bom = BillOfMaterials::default();
        bom.rails.insert(RailKind::KstBernoulliL, 1);
        bom.rails.insert(RailKind::KstSlide60L, 1);
        bom.rails.insert(RailKind::KstSlide120L, 3);
        bom.connectors = 2;

        let out = GraviSheetOutput::from(bom);

        assert_eq!(out.skytrax_bernoulli_left, Some(1));
        assert_eq!(out.skytrax_bernoulli_right, None);
        assert_eq!(out.skytrax_slide_60_left, Some(1));
        assert_eq!(out.skytrax_slide_60_right, None);
        assert_eq!(out.skytrax_slide_120_left, Some(3));
        assert_eq!(out.skytrax_slide_120_right, None);
        assert_eq!(out.skytrax_connector, Some(2));
    }
}
