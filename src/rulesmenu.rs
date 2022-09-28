use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{
    resources::CurrentRule, resources::GameRules, resources::Rule, resources::Rules,
    resources::SelectedRule, GameState,
};

pub struct RulesMenuPlugin;

impl Plugin for RulesMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Rule::default())
            .insert_resource(CurrentRule::default())
            .add_system_set(SystemSet::on_update(GameState::Paused).with_system(rules_menu_system));
    }
}

fn rules_menu_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut rule: ResMut<Rule>,
    mut current_rule: ResMut<CurrentRule>,
) {
    egui::Window::new("Rules Menu").show(egui_ctx.ctx_mut(), |ui| {
        // living cell
        ui.label("Living Cell Rule:");
        ui.checkbox(&mut current_rule.0.default, "Use default");
        if !current_rule.0.default {
            ui.checkbox(&mut current_rule.0.single, "Use Single");
            ui.checkbox(&mut current_rule.0.range, "Use range");
            ui.checkbox(&mut current_rule.0.singles, "Use singles");
            if current_rule.0.single {
                current_rule.0.range = false;
                current_rule.0.singles = false;
                current_rule.0.default = false;
                ui.add(
                    egui::Slider::new(&mut current_rule.0.single_value, 0..=8)
                        .text("Number of neighbours"),
                );
                rule.0.living_rule = Rules::Single(current_rule.0.single_value);
            } else if current_rule.0.range {
                current_rule.0.single = false;
                current_rule.0.singles = false;
                current_rule.0.default = false;
                ui.add(
                    egui::Slider::new(&mut current_rule.0.range_value.0, 0..=7)
                        .text("Lowest number of neighbours"),
                );
                ui.add(
                    egui::Slider::new(&mut current_rule.0.range_value.1, 1..=8)
                        .text("Highest number of neighbours"),
                );
                rule.0.living_rule = Rules::Range {
                    min: current_rule.0.range_value.0,
                    max: current_rule.0.range_value.1,
                };
            } else if current_rule.0.singles {
                current_rule.0.single = false;
                current_rule.0.range = false;
                current_rule.0.default = false;
                ui.label("List of possible neighbour values (separate by comma e.g. 1,4,7,8)");
                ui.text_edit_singleline(&mut current_rule.0.singles_value);
                let string_out: Vec<&str> = current_rule.0.singles_value.split(",").collect();
                let mut singles = Vec::new();
                for s in string_out {
                    match s.parse() {
                        Ok(i) => singles.push(i),
                        Err(e) => {
                            ui.label(format!("{}", e));
                        }
                    }
                }
                rule.0.living_rule = Rules::Singles(singles);
            }
        } else {
            current_rule.0 = SelectedRule::default();
            rule.0.living_rule = Rules::Default;
        }

        // dead cell
        ui.label("Dead Cell Rules:");
        ui.checkbox(&mut current_rule.1.default, "Use default");
        if !current_rule.1.default {
            ui.checkbox(&mut current_rule.1.single, "Use Single");
            ui.checkbox(&mut current_rule.1.range, "Use range");
            ui.checkbox(&mut current_rule.1.singles, "Use singles");
            if current_rule.1.single {
                current_rule.1.range = false;
                current_rule.1.singles = false;
                current_rule.1.default = false;
                ui.add(
                    egui::Slider::new(&mut current_rule.1.single_value, 0..=8)
                        .text("Number of neighbours"),
                );
                rule.0.dead_rule = Rules::Single(current_rule.1.single_value);
            } else if current_rule.1.range {
                current_rule.1.single = false;
                current_rule.1.singles = false;
                current_rule.1.default = false;
                ui.add(
                    egui::Slider::new(&mut current_rule.1.range_value.0, 0..=7)
                        .text("Lowest number of neighbours"),
                );
                ui.add(
                    egui::Slider::new(&mut current_rule.1.range_value.1, 1..=8)
                        .text("Highest number of neighbours"),
                );
                rule.0.dead_rule = Rules::Range {
                    min: current_rule.1.range_value.0,
                    max: current_rule.1.range_value.1,
                };
            } else if current_rule.1.singles {
                current_rule.1.single = false;
                current_rule.1.range = false;
                current_rule.1.default = false;
                ui.label("List of possible neighbour values (separate by comma e.g. 1,4,7,8)");
                ui.text_edit_singleline(&mut current_rule.1.singles_value);
                let string_out: Vec<&str> = current_rule.1.singles_value.split(",").collect();
                let mut singles = Vec::new();
                for s in string_out {
                    match s.parse() {
                        Ok(i) => singles.push(i),
                        Err(e) => {
                            ui.label(format!("{}", e));
                        }
                    }
                }
                rule.0.dead_rule = Rules::Singles(singles);
            }
        } else {
            current_rule.1 = SelectedRule::default();
            rule.0.dead_rule = Rules::Default;
        }

        // infected cell
        ui.label("Virulence:");
        ui.checkbox(&mut current_rule.2, "Use default");
        if !current_rule.2 {
            ui.add(egui::Slider::new(&mut current_rule.3, 0..=8).text("Number of neighbours"));
            rule.0.virulence = current_rule.3;
        } else {
            current_rule.2 = true;
            rule.0.virulence = GameRules::default().virulence;
        }
    });
}
