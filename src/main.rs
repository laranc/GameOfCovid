use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod ascii;
mod components;
mod debug;
mod generation;
mod grid;
mod resources;
mod resultmenu;
mod rulesmenu;

use ascii::AsciiPlugin;
use debug::DebugPlugin;
use generation::GenerationPlugin;
use grid::GridPlugin;
use resources::CellStates;
use resultmenu::ResultMenuPlugin;
use rulesmenu::RulesMenuPlugin;

pub const RESOLUTION: f32 = 16. / 9.;
pub const SCREEN_HEIGHT: f32 = 900.;
pub const TIME_STEP: f32 = 1. / 60.;
pub const BASE_SPEED: f32 = 500.;
pub const TILE_SIZE: f32 = 20.;

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
        .add_plugin(EguiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(GenerationPlugin)
        .add_plugin(RulesMenuPlugin)
        .add_plugin(ResultMenuPlugin)
        .add_startup_system(setup_system)
        .add_system(game_state_system)
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
