use crate::{components::CellState, grid::MAP_SIZE};

#[derive(Default)]
pub struct CursorPosition(pub usize, pub usize);

#[derive(Default)]
pub struct PrevCursorPosition(pub usize, pub usize);

pub struct CellStates(pub [[CellState; MAP_SIZE.1]; MAP_SIZE.0]);

impl Default for CellStates {
    fn default() -> Self {
        Self([[CellState::default(); MAP_SIZE.1]; MAP_SIZE.0])
    }
}

#[derive(Default, Eq, PartialEq, Clone)]
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
    pub fn value(self) -> Vec<u8> {
        match self {
            Self::Single(i) => vec![i],
            Self::Range { min: l, max: u } => vec![l, u],
            Self::Singles(v) => v,
            _ => panic!("Error, cannot return value from default rule"),
        }
    }
    pub fn in_range(self, num: u8) -> bool {
        match self {
            Self::Single(i) => {
                if num == i {
                    true
                } else {
                    false
                }
            }
            Self::Range { min: l, max: u } => {
                for i in l..=u {
                    if num == i {
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

#[derive(Eq, PartialEq, Clone)]
pub struct GameRules {
    pub living_rule: Rules,
    pub dead_rule: Rules,
    pub virulence: u8,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            living_rule: Rules::default(),
            dead_rule: Rules::default(),
            virulence: 2,
        }
    }
}

#[derive(Default)]
pub struct Rule(pub GameRules);
