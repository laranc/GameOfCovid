use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

mod ascii;
mod components;
mod debug;
mod generation;
mod grid;
mod optionsmenu;
mod resources;
mod resultmenu;

use ascii::AsciiPlugin;
use components::{CellComponent, MapComponent};
use debug::DebugPlugin;
use generation::GenerationPlugin;
use grid::{GridPlugin, MAP_SIZE};
use optionsmenu::OptionsMenuPlugin;
use resources::{CameraPosition, CellStates, CursorPosition, PrevCursorPosition};
use resultmenu::ResultMenuPlugin;

pub const RESOLUTION: f32 = 16. / 9.;
pub const SCREEN_HEIGHT: f32 = 900.;
pub const TILE_SIZE: f32 = 20.;
pub const BASE_TICK_SPEED: f32 = 0.5;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

impl Default for WinSize {
    fn default() -> Self {
        Self {
            w: SCREEN_HEIGHT * RESOLUTION,
            h: SCREEN_HEIGHT,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Paused,
    Play,
    Complete,
}

fn main() {
    App::new()
        .add_state(GameState::Paused)
        .insert_resource(WindowDescriptor {
            title: "Game of Covid".to_string(),
            width: SCREEN_HEIGHT * RESOLUTION,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .init_resource::<CellStates>()
        .add_plugins(DefaultPlugins)
        .add_plugin(AsciiPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(GenerationPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(OptionsMenuPlugin)
        .add_plugin(ResultMenuPlugin)
        .add_startup_system(setup_system)
        .add_system(game_state_system)
        .add_system(controls_panel_system)
        .run();
}

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    // add WinSize resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);
}

fn game_state_system(mut state: ResMut<State<GameState>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_released(KeyCode::E) {
        match state.current() {
            GameState::Paused => state
                .set(GameState::Play)
                .expect("Failed to change state to Paused"),
            GameState::Play => state
                .set(GameState::Paused)
                .expect("Failed to change state to Play"),
            _ => (),
        }
    }
    if keyboard.just_released(KeyCode::F) {
        match state.current() {
            GameState::Complete => state
                .set(GameState::Paused)
                .expect("Failed to change state to Paused"),
            _ => state
                .set(GameState::Complete)
                .expect("Failed to change state to Complete"),
        }
    }
}

fn controls_panel_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut camera_query: Query<&mut Transform, (Without<MapComponent>, Without<CellComponent>)>,
    map_query: Query<&Children, With<MapComponent>>,
    cell_query: Query<(&CellComponent, &Transform)>,
    mut camera_position: ResMut<CameraPosition>,
    mut cursor_position: ResMut<CursorPosition>,
    mut prev_position: ResMut<PrevCursorPosition>,
) {
    egui::TopBottomPanel::bottom("Controls").show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Use [W][A][S][D] to move the cursor.");
            ui.label("Press [SPACE] to change the cell.");
            ui.label("Press [E] to to start and stop the game.");
            ui.label("Press [F] to conclude the game.");
            ui.label("Use arrow keys to pan the camera.");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                if ui.button("Centre camera").clicked() {
                    let mut camera_transform = camera_query.single_mut();
                    let children = map_query.single();
                    for &child in children.iter() {
                        let (cell, cell_transform) = cell_query.get(child).unwrap();
                        if cell.coord.0 == MAP_SIZE.0 / 2 && cell.coord.1 == MAP_SIZE.1 / 2 {
                            camera_transform.translation.x = cell_transform.translation.x;
                            camera_transform.translation.y = cell_transform.translation.y;
                            camera_position.0 = cell.coord.0;
                            camera_position.1 = cell.coord.1;
                        }
                    }
                }
                if ui.button("Go to cursor").clicked() {
                    let mut camera_transform = camera_query.single_mut();
                    let children = map_query.single();
                    for &child in children.iter() {
                        let (cell, cell_transform) = cell_query.get(child).unwrap();
                        if cell.coord.0 == cursor_position.0 && cell.coord.1 == cursor_position.1 {
                            camera_transform.translation.x = cell_transform.translation.x;
                            camera_transform.translation.y = cell_transform.translation.y;
                            camera_position.0 = cell.coord.0;
                            camera_position.1 = cell.coord.1;
                        }
                    }
                }
                if ui.button("Centre cursor").clicked() {
                    prev_position.0 = cursor_position.0;
                    prev_position.1 = cursor_position.1;
                    cursor_position.0 = MAP_SIZE.0 / 2;
                    cursor_position.1 = MAP_SIZE.1 / 2;
                }
            });
        });
    });
}
