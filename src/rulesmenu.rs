use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{resources::Rule, resources::Rules, GameState};

pub struct RulesMenuPlugin;

impl Plugin for RulesMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Rule::default())
            .insert_resource(CurrentRule::default())
            .add_system_set(SystemSet::on_update(GameState::Paused).with_system(rules_menu_system));
    }
}

struct CurrentRule {
    single: bool,
    range: bool,
    singles: bool,
    default: bool,
}

impl Default for CurrentRule {
    fn default() -> Self {
        CurrentRule {
            single: false,
            range: false,
            singles: false,
            default: true,
        }
    }
}

fn rules_menu_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut rule: ResMut<Rule>,
    mut current_rule: ResMut<CurrentRule>,
) {
    egui::Window::new("Rules Menu").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Rules:");
        ui.checkbox(&mut current_rule.default, "Use default rules");
        if !current_rule.default {
            ui.checkbox(&mut current_rule.single, "Use Single");
            ui.checkbox(&mut current_rule.range, "Use range");
            ui.checkbox(&mut current_rule.singles, "Use singles");
            if current_rule.single {
                current_rule.range = false;
                current_rule.singles = false;
                current_rule.default = false;
                let mut single = 0;
                ui.add(egui::Slider::new(&mut single, 0..=8).text("Number of neighbours"));
                rule.0.living_rule = Rules::Single(single);
            } else if current_rule.range {
                current_rule.single = false;
                current_rule.singles = false;
                current_rule.default = false;
                let mut lower = 0;
                let mut upper = 0;
                ui.add(egui::Slider::new(&mut lower, 0..=7).text("Lowest number of neighbours"));
                ui.add(egui::Slider::new(&mut upper, 1..=8).text("Highest number of neighbours"));
                rule.0.living_rule = Rules::Range {
                    min: lower,
                    max: upper,
                };
            } else if current_rule.singles {
                current_rule.single = false;
                current_rule.range = false;
                current_rule.default = false;
                let mut string_in = String::new();
                ui.label("List of possible neighbour values (separate by comma e.g. 1,4,7,8)");
                ui.text_edit_singleline(&mut string_in);
                let string_out: Vec<&str> = string_in.split(",").collect();
                let mut singles = Vec::new();
                for s in string_out {
                    match s.parse() {
                        Ok(v) => singles.push(v),
                        Err(e) => {
                            ui.label(format!("{}", e));
                        }
                    }
                }
                rule.0.living_rule = Rules::Singles(singles);
            } else {
                current_rule.single = false;
                current_rule.range = false;
                current_rule.singles = false;
                current_rule.default = true;
            }
        } else {
            current_rule.single = false;
            current_rule.range = false;
            current_rule.singles = false;
            rule.0.living_rule = Rules::Default;
        }
    });
}
