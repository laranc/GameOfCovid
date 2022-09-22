use bevy::{asset::Handle, prelude::Component, sprite::TextureAtlas};
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct MapComponent;

#[derive(PartialEq, Eq, Copy, Clone, Inspectable, Default, Debug)]
pub enum CellState {
    Alive,
    #[default]
    Dead,
    Infected,
}

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
