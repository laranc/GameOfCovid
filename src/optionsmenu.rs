use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use std::time::Duration;

use crate::{
    components::{CellComponent, MapComponent},
    grid::clear_grid,
    resources::{
        CellStates, CurrentOptions, GameOptions, GameTimer, Options, Rules, SelectedRules,
    },
    GameState,
};

pub struct OptionsMenuPlugin;

impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameOptions::default())
            .insert_resource(CurrentOptions::default())
            .add_system_set(
                SystemSet::on_update(GameState::Paused).with_system(options_menu_system),
            );
    }
}

fn options_menu_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut rule: ResMut<GameOptions>,
    mut current_rule: ResMut<CurrentOptions>,
    mut game_time: ResMut<GameTimer>,
    map_query: Query<&mut Children, With<MapComponent>>,
    cell_query: Query<(&mut CellComponent, &mut TextureAtlasSprite)>,
    cell_states: ResMut<CellStates>,
) {
    // open new floating window
    egui::Window::new("Options").show(egui_ctx.ctx_mut(), |ui| {
        // living cell
        ui.label("Living Cell Rule:");
        ui.checkbox(&mut current_rule.0.default, "Use default");
        if !current_rule.0.default {
            ui.checkbox(&mut current_rule.0.single, "Use single");
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
                ui.label("List of possible neighbour values (separate by space e.g. 1 4 7 8)");
                ui.text_edit_singleline(&mut current_rule.0.singles_value);
                let string_out: Vec<&str> = current_rule.0.singles_value.split(" ").collect();
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
            current_rule.0 = SelectedRules::default();
            rule.0.living_rule = Rules::Default;
        }

        // dead cell
        ui.label("Dead Cell Rules:");
        ui.checkbox(&mut current_rule.1.default, "Use default");
        if !current_rule.1.default {
            ui.checkbox(&mut current_rule.1.single, "Use single");
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
                ui.label("List of possible neighbour values (separate by space e.g. 1 4 7 8)");
                ui.text_edit_singleline(&mut current_rule.1.singles_value);
                let string_out: Vec<&str> = current_rule.1.singles_value.split(" ").collect();
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
            current_rule.1 = SelectedRules::default();
            rule.0.dead_rule = Rules::Default;
        }

        // infected cell
        ui.label("Virulence:");
        ui.checkbox(&mut current_rule.2.virulence, "Use default");
        if !current_rule.2.virulence {
            ui.add(
                egui::Slider::new(&mut current_rule.2.virulence_value, 0..=8)
                    .text("Number of neighbours"),
            );
            rule.0.virulence = current_rule.2.virulence_value;
        } else {
            current_rule.2.virulence = true;
            rule.0.virulence = Options::default().virulence;
        }

        // tick speed
        ui.label("Tick Speed:");
        ui.checkbox(&mut current_rule.2.tick_speed, "Use default");
        if !current_rule.2.tick_speed {
            ui.add(
                egui::Slider::new(&mut current_rule.2.tick_speed_value, 0.1..=1.0)
                    .text("Tick speed"),
            );
            rule.0.tick_speed = current_rule.2.tick_speed_value;
            game_time.0 = Timer::new(Duration::from_secs_f32(rule.0.tick_speed), true);
        } else {
            current_rule.2.tick_speed = true;
            rule.0.tick_speed = Options::default().tick_speed;
            game_time.0 = GameTimer::default().0;
        }

        // clear grid
        if ui.button("Clear grid").clicked() {
            clear_grid(map_query, cell_query, cell_states);
        }
    });
}
