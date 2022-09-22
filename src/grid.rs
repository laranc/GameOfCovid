use bevy::prelude::*;

use crate::{
    ascii::spawn_sprite, components::AsciiSheet, components::CellComponent, components::CellState,
    components::MapComponent, resources::CellStates, TILE_SIZE,
};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_grid_system)
            .add_startup_system_to_stage(StartupStage::PostStartup, add_grid_system);
    }
}

pub const MAP_SIZE: (usize, usize) = (100, 100);

fn create_grid_system(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut cells = Vec::new();

    for x in 0..MAP_SIZE.0 {
        for y in 0..MAP_SIZE.1 {
            let cell = spawn_sprite(
                &mut commands,
                &ascii,
                0,
                Color::rgb(0., 0., 0.),
                Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.),
            );
            let cell_name = "Cell (".to_owned() + &x.to_string() + ", " + &y.to_string() + ")";
            commands
                .entity(cell)
                .insert(Name::new(cell_name))
                .insert(CellComponent {
                    coord: (x, y),
                    state: CellState::default(),
                });
            cells.push(cell);
        }
    }

    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(MapComponent)
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(ComputedVisibility::default())
        .insert(Visibility::visible())
        .push_children(&cells);
}

fn add_grid_system(
    map_query: Query<(&MapComponent, &Children)>,
    cell_query: Query<(&CellComponent, &Transform)>,
    mut cell_states: ResMut<CellStates>,
    mut camera_query: Query<&mut Transform, (Without<MapComponent>, Without<CellComponent>)>,
) {
    // add grid to array resource and centre camera
    let mut camera_transform = camera_query.single_mut();
    let (_map, children) = map_query.single();
    for &child in children.iter() {
        let (cell, cell_transform) = cell_query.get(child).unwrap();
        cell_states.0[cell.coord.0][cell.coord.1] = cell.state;
        if cell.coord.0 == MAP_SIZE.0 / 2 && cell.coord.1 == MAP_SIZE.1 / 2 {
            camera_transform.translation.x = cell_transform.translation.x;
            camera_transform.translation.y = cell_transform.translation.y;
        }
    }
}
