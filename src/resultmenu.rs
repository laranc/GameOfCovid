use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{components::CellState, resources::CellStates, GameState};

pub struct ResultMenuPlugin;

impl Plugin for ResultMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Complete).with_system(result_menu_system),
        );
    }
}

fn result_menu_system(mut egui_ctx: ResMut<EguiContext>, cell_states: Res<CellStates>) {
    // count the number cells in various states
    let mut alive_count = 0.;
    let mut infected_count = 0.;
    for i in 0..cell_states.0.len() {
        for j in 0..cell_states.0[i].len() {
            match cell_states.0[i][j] {
                CellState::Alive => alive_count += 1.,
                CellState::Infected => infected_count += 1.,
                _ => (),
            }
        }
    }
    let proportion = infected_count / (alive_count + infected_count);
    // display the percentage of infected to alive cells in a floating window
    egui::Window::new("Result Menu").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Conclusion:");
        if proportion > 0.5 {
            ui.label(format!(
                "{:.2}% are infected, therefore we are in a pandemic (alive: {}, infected: {})",
                proportion * 100.,
                alive_count,
                infected_count
            ));
        } else {
            ui.label(format!(
                "{:.2}% are infected, therefore we are not in a pandemic (alive: {}, infected: {})",
                proportion * 100.,
                alive_count,
                infected_count
            ));
        }
    });
}
