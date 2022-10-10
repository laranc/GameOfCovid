use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;

// define component used to identify the map entity
#[derive(Component)]
pub struct MapComponent;

// define the possible state a cell could be in
#[derive(PartialEq, Eq, Copy, Clone, Inspectable, Default, Debug)]
pub enum CellState {
    Alive,
    #[default]
    Dead,
    Infected,
}

// define component used to identify the cell entities
#[derive(Clone, Component, Inspectable)]
pub struct CellComponent {
    pub coord: (usize, usize),
    pub state: CellState,
}

impl Default for CellComponent {
    fn default() -> Self {
        Self {
            coord: (0, 0),
            state: CellState::default(),
        }
    }
}
