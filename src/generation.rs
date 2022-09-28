use bevy::prelude::*;

use crate::{
    components::CellComponent, components::CellState, components::MapComponent,
    grid::update_sprite, grid::MAP_SIZE, resources::CellStates, resources::CursorPosition,
    resources::GameOptions, resources::GameTimer, resources::Options,
    resources::PrevCursorPosition, resources::Rules, GameState,
};

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer::default())
            .add_system_set(
                SystemSet::on_update(GameState::Paused).with_system(user_drawing_system),
            )
            .add_system_set(SystemSet::on_update(GameState::Play).with_system(generation_system));
    }
}

fn user_drawing_system(
    position: Res<CursorPosition>,
    prev_position: Res<PrevCursorPosition>,
    keyboard: Res<Input<KeyCode>>,
    mut cell_states: ResMut<CellStates>,
    map_query: Query<&mut Children, With<MapComponent>>,
    mut cell_query: Query<(&mut CellComponent, &mut TextureAtlasSprite)>,
) {
    let children = map_query.single();
    for &child in children.iter() {
        let (mut cell, mut sprite) = cell_query.get_mut(child).unwrap();
        if (prev_position.0, prev_position.1) == cell.coord {
            match cell.state {
                CellState::Dead => {
                    sprite.color = Color::rgb(0., 0., 0.);
                }
                CellState::Alive => {
                    sprite.color = Color::rgb(255., 255., 255.);
                }
                CellState::Infected => {
                    sprite.color = Color::rgb(0., 255., 0.);
                }
            }
        }
        if cell.coord == (position.0, position.1) {
            sprite.color = Color::rgb(255., 255., 0.);
            if keyboard.just_released(KeyCode::Space) {
                match cell.state {
                    CellState::Dead => cell.state = CellState::Alive,
                    CellState::Alive => cell.state = CellState::Infected,
                    CellState::Infected => cell.state = CellState::Dead,
                }
                update_sprite(cell.state, sprite);
                cell_states.0[cell.coord.0][cell.coord.1] = cell.state;
            }
        }
    }
}

fn generation_system(
    mut cell_states: ResMut<CellStates>,
    map_query: Query<&mut Children, With<MapComponent>>,
    mut cell_query: Query<(&mut CellComponent, &mut TextureAtlasSprite)>,
    time: Res<Time>,
    mut game_time: ResMut<GameTimer>,
    options: Res<GameOptions>,
) {
    for i in 0..cell_states.0.len() {
        for j in 0..cell_states.0[i].len() {
            let mut live_neighbors = 0;
            let mut infected_neighbors = 0;
            if i > 0 {
                match cell_states.0[i - 1][j] {
                    CellState::Alive => live_neighbors += 1,
                    CellState::Infected => infected_neighbors += 1,
                    _ => (),
                }
            }
            if i > 0 && j > 0 {
                match cell_states.0[i - 1][j - 1] {
                    CellState::Alive => live_neighbors += 1,
                    CellState::Infected => infected_neighbors += 1,
                    _ => (),
                }
            }
            if j > 0 {
                match cell_states.0[i][j - 1] {
                    CellState::Alive => live_neighbors += 1,
                    CellState::Infected => infected_neighbors += 1,
                    _ => (),
                }
            }
            if i < MAP_SIZE.0 - 1 && j > 0 {
                match cell_states.0[i + 1][j - 1] {
                    CellState::Alive => live_neighbors += 1,
                    CellState::Infected => infected_neighbors += 1,
                    _ => (),
                }
            }
            if i < MAP_SIZE.0 - 1 {
                match cell_states.0[i + 1][j] {
                    CellState::Alive => live_neighbors += 1,
                    CellState::Infected => infected_neighbors += 1,
                    _ => (),
                }
            }
            if i < MAP_SIZE.0 - 1 && j < MAP_SIZE.1 - 1 {
                match cell_states.0[i + 1][j + 1] {
                    CellState::Alive => live_neighbors += 1,
                    CellState::Infected => infected_neighbors += 1,
                    _ => (),
                }
            }
            if j < MAP_SIZE.1 - 1 {
                match cell_states.0[i][j + 1] {
                    CellState::Alive => live_neighbors += 1,
                    CellState::Infected => infected_neighbors += 1,
                    _ => (),
                }
            }
            if i > 0 && j < MAP_SIZE.1 - 1 {
                match cell_states.0[i - 1][j + 1] {
                    CellState::Alive => live_neighbors += 1,
                    CellState::Infected => infected_neighbors += 1,
                    _ => (),
                }
            }
            cell_states.0[i][j] = cell_check(
                cell_states.0[i][j],
                live_neighbors,
                infected_neighbors,
                options.0.clone(),
            );
        }
    }

    let children = map_query.single();
    for &child in children.iter() {
        let (mut cell, sprite) = cell_query.get_mut(child).unwrap();
        if game_time.0.tick(time.delta()).just_finished() {
            if cell.state != cell_states.0[cell.coord.0][cell.coord.1] {
                cell.state = cell_states.0[cell.coord.0][cell.coord.1];
                update_sprite(cell.state, sprite);
            }
        }
    }
}

fn cell_check(
    current_state: CellState,
    live_neighbors: u8,
    infected_neighbors: u8,
    options: Options,
) -> CellState {
    if options == Options::default() {
        match current_state {
            CellState::Alive => {
                if live_neighbors < 2 {
                    CellState::Dead
                } else if live_neighbors == 2 || live_neighbors == 3 {
                    CellState::Alive
                } else if live_neighbors > 3 || infected_neighbors >= options.virulence {
                    CellState::Infected
                } else {
                    CellState::Alive
                }
            }
            CellState::Dead => {
                if live_neighbors == 3 {
                    CellState::Alive
                } else {
                    CellState::Dead
                }
            }
            CellState::Infected => {
                if live_neighbors < 3 {
                    CellState::Dead
                } else {
                    CellState::Infected
                }
            }
        }
    } else {
        match current_state {
            CellState::Alive => match options.living_rule {
                Rules::Single(_) => {
                    if live_neighbors < options.living_rule.value()[0] {
                        CellState::Dead
                    } else if live_neighbors == options.living_rule.value()[0]
                        || live_neighbors == options.living_rule.value()[0] + 1
                    {
                        CellState::Alive
                    } else if live_neighbors > options.living_rule.value()[0] + 1
                        || infected_neighbors >= options.virulence
                    {
                        CellState::Infected
                    } else {
                        CellState::Alive
                    }
                }
                Rules::Range { min: _, max: _ } => {
                    if !options.living_rule.in_range(&live_neighbors) {
                        CellState::Dead
                    } else if live_neighbors > options.living_rule.value()[1] + 1
                        || infected_neighbors >= options.virulence
                    {
                        CellState::Infected
                    } else {
                        CellState::Alive
                    }
                }
                Rules::Singles(_) => {
                    if !options.living_rule.in_range(&live_neighbors) {
                        CellState::Dead
                    } else if live_neighbors > *options.living_rule.value().last().unwrap()
                        || infected_neighbors >= options.virulence
                    {
                        CellState::Infected
                    } else {
                        CellState::Alive
                    }
                }
                _ => CellState::default(),
            },
            CellState::Dead => match options.dead_rule {
                Rules::Single(_) => {
                    if live_neighbors == options.dead_rule.value()[0] {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    }
                }
                Rules::Range { min: _, max: _ } => {
                    if options.dead_rule.in_range(&live_neighbors) {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    }
                }
                Rules::Singles(_) => {
                    if options.dead_rule.in_range(&live_neighbors) {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    }
                }
                _ => CellState::default(),
            },
            CellState::Infected => {
                if live_neighbors < options.virulence + 1 {
                    CellState::Dead
                } else {
                    CellState::Infected
                }
            }
        }
    }
}
