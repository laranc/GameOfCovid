use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContext};
use chrono::Local;
use std::{fs::File, io::prelude::*};

use crate::{
    components::CellState,
    resources::{CellStates, QuestionnarieResponse, Questions},
    GameState,
};

pub struct ResultMenuPlugin;

impl Plugin for ResultMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Questions::default())
            .insert_resource(QuestionnarieResponse::default())
            .add_system_set(
                SystemSet::on_update(GameState::Complete).with_system(result_menu_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Complete).with_system(questionnaire_system),
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
    let proportion: f32 = infected_count / (alive_count + infected_count);
    // display the percentage of infected to alive cells in a floating window
    egui::Window::new("Result Menu").show(egui_ctx.ctx_mut(), |ui| {
        ui.heading("Conclusion:");
        if proportion > 0.5 {
            ui.label(format!(
                "{:.2}% are infected, therefore we are in a pandemic (alive: {}, infected: {})",
                proportion * 100.,
                alive_count,
                infected_count
            ));
        } else if !proportion.is_nan() {
            ui.label(format!(
                "{:.2}% are infected, therefore we are not in a pandemic (alive: {}, infected: {})",
                proportion * 100.,
                alive_count,
                infected_count
            ));
        } else {
            ui.label(format!(
                "It is unknown if we are in a pandemic (alive: {}, infected: {})",
                alive_count, infected_count,
            ));
        }
    });
}

fn questionnaire_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut response: ResMut<QuestionnarieResponse>,
    mut questions: ResMut<Questions>,
    mut exit: EventWriter<AppExit>,
) {
    // open a new floating window to display the final questionnaire
    egui::Window::new("Questionnaire").show(egui_ctx.ctx_mut(), |ui| {
        // survey items
        ui.checkbox(&mut questions.0, "Did you find the game intuitive?");
        ui.checkbox(&mut questions.1, "Did you find the game informative?");
        ui.label("Briefly list what you enjoyed about the game:");
        ui.text_edit_singleline(&mut questions.2);
        ui.label("In your own words, what would you say you have learnt from this game?");
        ui.text_edit_multiline(&mut questions.3);
        ui.label("In your words, what are some aspects of the game that could be improved?");
        ui.text_edit_multiline(&mut questions.4);

        // submit the results of the survey
        if ui.button("Submit").clicked() {
            if questions.0 {
                response
                    .0
                    .push("Did you find the game intuitive? | Y \n".to_string());
            } else {
                response
                    .0
                    .push("Did you find the game intuitive? | N \n".to_string());
            }
            if questions.1 {
                response
                    .0
                    .push("Did you find the game informative? | Y \n".to_string());
            } else {
                response
                    .0
                    .push("Did you find the game informative? | N \n".to_string());
            }
            response.0.push(format!(
                "\nBriefly list what you enjoyed about the game \n{}",
                questions.2.clone()
            ));
            response.0.push(format!(
                "\nIn your own words, what would you say you have learnt from this game? \n{}",
                questions.3.clone()
            ));
            response.0.push(format!(
                "\nIn your words, what are some aspects of the game that could be improved? \n{}",
                questions.4.clone()
            ));
            // fetch the date in format: DD-MM-YYYY_hour-min-sec
            let date_time = Local::now().format("%d-%m-%Y_%H-%M-%S");
            // create new file
            let mut file = File::create(format!("result-{}.txt", date_time)).unwrap();
            let mut success = false;
            for s in response.0.iter() {
                // handle potential write error
                match writeln!(&mut file, "{}", s) {
                    Ok(_) => success = true,
                    Err(e) => {
                        success = false;
                        ui.label(format!("Failed to save your response: {:?}", e));
                    }
                }
            }
            if success {
                response.1 = true; // flag the end of the game
                response.0.clear(); // empty the vector to free memory
            }
        }
        if response.1 {
            ui.label("Thank you for completing this survey. You may now exit:");
            if ui.button("Finish Game").clicked() {
                exit.send(AppExit); // call exit event
            }
        }
    });
}
