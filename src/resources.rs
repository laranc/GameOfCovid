use bevy::{
    asset::Handle,
    prelude::{Component, Timer},
    sprite::TextureAtlas,
};
use bevy_inspector_egui::Inspectable;
use std::time::Duration;

use crate::{components::CellState, grid::MAP_SIZE, BASE_TICK_SPEED};

#[derive(Component, Inspectable)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

// keep track of the cursors position on the grid
pub struct CursorPosition(pub usize, pub usize);

impl Default for CursorPosition {
    fn default() -> Self {
        Self(MAP_SIZE.0 / 2, MAP_SIZE.1 / 2)
    }
}

// keep track of the previous cursor location
#[derive(Default)]
pub struct PrevCursorPosition(pub usize, pub usize);

// keep track of the camera position in relation to the grid
pub struct CameraPosition(pub usize, pub usize);

// implement a starting position of the camera
impl Default for CameraPosition {
    fn default() -> Self {
        Self(MAP_SIZE.0 / 2, MAP_SIZE.1 / 2)
    }
}

// keep track of the state of cells in a data structure outside the game engine
pub struct CellStates(pub [[CellState; MAP_SIZE.1]; MAP_SIZE.0]);

// define the starting conditions of the game
impl Default for CellStates {
    fn default() -> Self {
        Self([[CellState::default(); MAP_SIZE.1]; MAP_SIZE.0])
    }
}

// keep track of the generation speed
pub struct GameTimer(pub Timer);

impl Default for GameTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs_f32(BASE_TICK_SPEED), true))
    }
}

// define the alterable rules of the game
#[derive(Default, Eq, PartialEq, Inspectable)]
pub enum Rules {
    Single(u8),
    Range {
        min: u8,
        max: u8,
    },
    Singles(Vec<u8>),
    #[default]
    Default,
}

impl Rules {
    // verify the criteria
    pub fn in_range(&self, num: &u8) -> bool {
        match self {
            Self::Single(i) => {
                if num == i {
                    true
                } else {
                    false
                }
            }
            Self::Range { min: l, max: u } => {
                for i in *l..=*u {
                    if num == &i {
                        return true;
                    }
                }
                false
            }
            Self::Singles(v) => {
                for i in v.iter() {
                    if num == i {
                        return true;
                    }
                }
                false
            }
            _ => panic!("Error, cannot check value in default rule"),
        }
    }
    pub fn max(&self) -> u8 {
        match self {
            Self::Single(i) => *i,
            Self::Range { min: _, max: u } => *u,
            Self::Singles(v) => {
                let mut max = 0;
                for i in v.iter() {
                    if &max < i {
                        max = *i;
                    }
                }
                max
            }
            _ => panic!("Error, cannot find maximum in default rule"),
        }
    }
}

// define the game options
#[derive(PartialEq, Inspectable)]
pub struct Options {
    pub living_rule: Rules,
    pub dead_rule: Rules,
    pub virulence: u8,
    pub tick_speed: f32,
}

// define the default options
impl Default for Options {
    fn default() -> Self {
        Self {
            living_rule: Rules::default(),
            dead_rule: Rules::default(),
            virulence: 2,
            tick_speed: BASE_TICK_SPEED,
        }
    }
}

// define a wrapper for the options
#[derive(Default, Inspectable)]
pub struct GameOptions(pub Options);

// keep track of changing rules
pub struct SelectedRules {
    pub single: bool,
    pub range: bool,
    pub singles: bool,
    pub default: bool,
    pub single_value: u8,
    pub range_value: (u8, u8),
    pub singles_value: String,
}

impl Default for SelectedRules {
    fn default() -> Self {
        Self {
            single: false,
            range: false,
            singles: false,
            default: true,
            single_value: 2,
            range_value: (0, 2),
            singles_value: "2".to_string(),
        }
    }
}

// keep track of changing settings
pub struct SelectedOptions {
    pub virulence: bool,
    pub tick_speed: bool,
    pub virulence_value: u8,
    pub tick_speed_value: f32,
}

impl Default for SelectedOptions {
    fn default() -> Self {
        Self {
            virulence: true,
            tick_speed: true,
            virulence_value: 2,
            tick_speed_value: BASE_TICK_SPEED,
        }
    }
}

// define a wrapper for the changing rules and settings
#[derive(Default)]
pub struct CurrentOptions(pub SelectedRules, pub SelectedRules, pub SelectedOptions);

// define wrapper for debugging cells that undergo change
#[derive(Default)]
pub struct History(pub Vec<String>);

// define wrapper for the selections of the questions for the final survey
#[derive(Default)]
pub struct Questions(pub bool, pub bool, pub String, pub String, pub String);

// define wrapper for the final results of the survey
#[derive(Default)]
pub struct QuestionnarieResponse(pub Vec<String>, pub bool);
