use bevy::prelude::*;

use crate::{
    ascii::spawn_sprite, components::AsciiSheet, components::CellComponent, components::CellState,
    components::MapComponent, resources::CameraPosition, resources::CellStates,
    resources::CursorPosition, resources::PrevCursorPosition, GameState, TILE_SIZE,
};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraPosition::default())
            .insert_resource(CursorPosition::default())
            .insert_resource(PrevCursorPosition(0, 0))
            .add_startup_system(create_grid_system)
            .add_startup_system_to_stage(StartupStage::PostStartup, add_grid_system)
            .add_system_set(
                SystemSet::on_update(GameState::Paused).with_system(cursor_movement_system),
            )
            .add_system(camera_movement_system)
            .add_system(camera_update_system);
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
    map_query: Query<&Children, With<MapComponent>>,
    cell_query: Query<(&CellComponent, &Transform)>,
    mut cell_states: ResMut<CellStates>,
    mut camera_query: Query<&mut Transform, (Without<MapComponent>, Without<CellComponent>)>,
) {
    // add grid to array resource and centre camera
    let mut camera_transform = camera_query.single_mut();
    let children = map_query.single();
    for &child in children.iter() {
        let (cell, cell_transform) = cell_query.get(child).unwrap();
        cell_states.0[cell.coord.0][cell.coord.1] = cell.state;
        if cell.coord.0 == MAP_SIZE.0 / 2 && cell.coord.1 == MAP_SIZE.1 / 2 {
            camera_transform.translation.x = cell_transform.translation.x;
            camera_transform.translation.y = cell_transform.translation.y;
        }
    }
}

fn cursor_movement_system(
    keyboard: Res<Input<KeyCode>>,
    mut position: ResMut<CursorPosition>,
    mut prev_position: ResMut<PrevCursorPosition>,
) {
    if keyboard.just_released(KeyCode::A) {
        if position.0 > 0 {
            prev_position.0 = position.0;
            prev_position.1 = position.1;
            position.0 -= 1;
        }
    }
    if keyboard.just_released(KeyCode::D) {
        if position.0 < MAP_SIZE.0 - 1 {
            prev_position.0 = position.0;
            prev_position.1 = position.1;
            position.0 += 1;
        }
    }
    if keyboard.just_released(KeyCode::W) {
        if position.1 > 0 {
            prev_position.0 = position.0;
            prev_position.1 = position.1;
            position.1 -= 1;
        }
    }
    if keyboard.just_released(KeyCode::S) {
        if position.1 < MAP_SIZE.1 - 1 {
            prev_position.0 = position.0;
            prev_position.1 = position.1;
            position.1 += 1;
        }
    }
}

fn camera_movement_system(keyboard: Res<Input<KeyCode>>, mut position: ResMut<CameraPosition>) {
    if keyboard.pressed(KeyCode::Left) {
        if position.0 > 0 {
            position.0 -= 1;
        }
    }
    if keyboard.pressed(KeyCode::Right) {
        if position.0 < MAP_SIZE.0 - 1 {
            position.0 += 1;
        }
    }
    if keyboard.pressed(KeyCode::Up) {
        if position.1 > 0 {
            position.1 -= 1;
        }
    }
    if keyboard.pressed(KeyCode::Down) {
        if position.1 < MAP_SIZE.1 - 1 {
            position.1 += 1;
        }
    }
}

fn camera_update_system(
    mut camera_query: Query<&mut Transform, (Without<MapComponent>, Without<CellComponent>)>,
    map_query: Query<&Children, With<MapComponent>>,
    cell_query: Query<(&CellComponent, &Transform)>,
    position: Res<CameraPosition>,
) {
    let mut camera_transform = camera_query.single_mut();
    let children = map_query.single();
    for &child in children.iter() {
        let (cell, cell_transform) = cell_query.get(child).unwrap();
        if cell.coord.0 == position.0 && cell.coord.1 == position.1 {
            camera_transform.translation.x = cell_transform.translation.x;
            camera_transform.translation.y = cell_transform.translation.y;
        }
    }
}

pub fn update_sprite(state: CellState, mut sprite: Mut<TextureAtlasSprite>) -> () {
    match state {
        CellState::Alive => {
            sprite.color = Color::rgb(255., 255., 255.);
            sprite.index = 2;
        }
        CellState::Dead => {
            sprite.color = Color::rgb(0., 0., 0.);
            sprite.index = 0;
        }
        CellState::Infected => {
            sprite.color = Color::rgb(0., 255., 0.);
            sprite.index = 1;
        }
    }
}

pub fn clear_grid(
    map_query: Query<&mut Children, With<MapComponent>>,
    mut cell_query: Query<(&mut CellComponent, &mut TextureAtlasSprite)>,
    mut cell_states: ResMut<CellStates>,
) -> () {
    let children = map_query.single();
    for &child in children.iter() {
        let (mut cell, sprite) = cell_query.get_mut(child).unwrap();
        cell.state = CellState::Dead;
        update_sprite(cell.state, sprite);
        cell_states.0[cell.coord.0][cell.coord.1] = CellState::Dead;
    }
}
