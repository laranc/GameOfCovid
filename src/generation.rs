use bevy::prelude::*;

use crate::{
    components::{CellComponent, CellState, MapComponent},
    grid::{update_sprite, MAP_SIZE},
    resources::{
        CellStates, CursorPosition, GameOptions, GameTimer, History, Options, PrevCursorPosition,
        Rules,
    },
    GameState,
};

// define the plugin to be inserted into the main app
pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        // insert various functions into the app
        app.insert_resource(GameTimer::default())
            .insert_resource(History::default())
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
    // fetch the children of the map entity using a query
    let children = map_query.single();
    // iterate over the children
    for &child in children.iter() {
        // fetch the cell component and sprite of the cell using the child
        let (mut cell, mut sprite) = cell_query.get_mut(child).unwrap();
        // return the previously selected cell's colour back to its original colour
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
        // check if the cell is selected
        if cell.coord == (position.0, position.1) {
            sprite.color = Color::rgb(255., 255., 0.);
            // input system to change the state of the cell
            if keyboard.just_released(KeyCode::Space) {
                match cell.state {
                    CellState::Dead => cell.state = CellState::Alive,
                    CellState::Alive => cell.state = CellState::Infected,
                    CellState::Infected => cell.state = CellState::Dead,
                }
                update_sprite(cell.state, &mut sprite);
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
    position: Res<CursorPosition>,
    mut history: ResMut<History>,
) {
    // iterate over the cell data structure
    for i in 0..cell_states.0.len() {
        for j in 0..cell_states.0[i].len() {
            let mut live_neighbors = 0;
            let mut infected_neighbors = 0;
            // count the neighbours
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

    // fetch children from the map entity
    let children = map_query.single();
    // iterate over children
    for &child in children.iter() {
        // fetch the cell component and sprite from the cell entity
        let (mut cell, mut sprite) = cell_query.get_mut(child).unwrap();
        // set the cell selected by the cursor to its original colour
        if cell.coord == (position.0, position.1) {
            update_sprite(cell.state, &mut sprite);
        }
        // run only after each game tick
        if game_time.0.tick(time.delta()).just_finished() {
            // check if the cell entity is in a different state to its equivalent in the array
            if cell.state != cell_states.0[cell.coord.0][cell.coord.1] {
                // only record history if the program is compiled in debug mode
                if cfg!(debug_assertions) {
                    history.0.push(format!(
                        "({}, {}): {:?} -> {:?}",
                        cell.coord.0,
                        cell.coord.1,
                        cell.state,
                        cell_states.0[cell.coord.0][cell.coord.1]
                    ));
                }
                cell.state = cell_states.0[cell.coord.0][cell.coord.1];
                // update the sprite to reflect its state
                update_sprite(cell.state, &mut sprite);
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
    // check if the default options are in use
    if options == Options::default() {
        // check the cell against the criteria
        match current_state {
            CellState::Alive => {
                if live_neighbors < 2 {
                    CellState::Dead
                } else if live_neighbors == 2 || live_neighbors == 3 {
                    CellState::Alive
                } else if live_neighbors > 3 || infected_neighbors >= 2 {
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
        // check the cell against the user defined criteria
        match current_state {
            CellState::Alive => match options.living_rule {
                Rules::Single(i) => {
                    if live_neighbors < i {
                        CellState::Dead
                    } else if live_neighbors == i {
                        CellState::Alive
                    } else if live_neighbors > i + 1 || infected_neighbors >= options.virulence {
                        CellState::Infected
                    } else {
                        CellState::Alive
                    }
                }
                Rules::Range { min: _, max: u } => {
                    if !options.living_rule.in_range(&live_neighbors) {
                        CellState::Dead
                    } else if live_neighbors > u + 1 || infected_neighbors >= options.virulence {
                        CellState::Infected
                    } else {
                        CellState::Alive
                    }
                }
                Rules::Singles(_) => {
                    if !options.living_rule.in_range(&live_neighbors) {
                        CellState::Dead
                    } else if live_neighbors > options.living_rule.max()
                        || infected_neighbors >= options.virulence
                    {
                        CellState::Infected
                    } else {
                        CellState::Alive
                    }
                }
                _ => {
                    if live_neighbors < 2 {
                        CellState::Dead
                    } else if live_neighbors == 2 || live_neighbors == 3 {
                        CellState::Alive
                    } else if live_neighbors > 3 || infected_neighbors >= 2 {
                        CellState::Infected
                    } else {
                        CellState::Alive
                    }
                }
            },
            CellState::Dead => match options.dead_rule {
                Rules::Single(i) => {
                    if live_neighbors == i {
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
                _ => {
                    if live_neighbors == 3 {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    }
                }
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
