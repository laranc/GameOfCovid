use bevy::prelude::Timer;
use bevy_inspector_egui::Inspectable;
use std::time::Duration;

use crate::{components::CellState, grid::MAP_SIZE, BASE_TICK_SPEED};

pub struct CursorPosition(pub usize, pub usize);

impl Default for CursorPosition {
    fn default() -> Self {
        Self(MAP_SIZE.0 / 2, MAP_SIZE.1 / 2)
    }
}

#[derive(Default)]
pub struct PrevCursorPosition(pub usize, pub usize);

pub struct CameraPosition(pub usize, pub usize);

impl Default for CameraPosition {
    fn default() -> Self {
        Self(MAP_SIZE.0 / 2, MAP_SIZE.1 / 2)
    }
}

pub struct CellStates(pub [[CellState; MAP_SIZE.1]; MAP_SIZE.0]);

impl Default for CellStates {
    fn default() -> Self {
        Self([[CellState::default(); MAP_SIZE.1]; MAP_SIZE.0])
    }
}

pub struct GameTimer(pub Timer);

impl Default for GameTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs_f32(BASE_TICK_SPEED), true))
    }
}

#[derive(Default, Eq, PartialEq, Clone, Inspectable)]
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
    pub fn value(&self) -> Vec<u8> {
        match self {
            Self::Single(i) => vec![*i],
            Self::Range { min: l, max: u } => vec![*l, *u],
            Self::Singles(v) => v.clone(),
            _ => panic!("Error, cannot return value from default rule"),
        }
    }
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
                for i in v {
                    if num == i {
                        return true;
                    }
                }
                false
            }
            _ => panic!("Error, cannot check range in default rule"),
        }
    }
}

#[derive(PartialEq, Clone, Inspectable)]
pub struct Options {
    pub living_rule: Rules,
    pub dead_rule: Rules,
    pub virulence: u8,
    pub tick_speed: f32,
}

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

#[derive(Default, Inspectable)]
pub struct GameOptions(pub Options);

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

pub struct CurrentOptions(pub SelectedRules, pub SelectedRules, pub SelectedOptions);

impl Default for CurrentOptions {
    fn default() -> Self {
        Self(
            SelectedRules::default(),
            SelectedRules::default(),
            SelectedOptions::default(),
        )
    }
}
