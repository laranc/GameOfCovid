use bevy::prelude::*;

use crate::{
    components::CellComponent, components::CellState, components::MapComponent, grid::MAP_SIZE,
    resources::CellStates, resources::CursorPosition, resources::GameRules,
    resources::PrevCursorPosition, resources::Rule, resources::Rules, GameState,
};

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition(MAP_SIZE.0 / 2, MAP_SIZE.1 / 2))
            .insert_resource(PrevCursorPosition(0, 0))
            .add_system_set(
                SystemSet::on_update(GameState::Paused).with_system(cursor_movement_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Paused).with_system(user_drawing_system),
            )
            .add_system_set(SystemSet::on_update(GameState::Play).with_system(generation_system));
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

fn update_sprite(state: CellState, mut sprite: Mut<TextureAtlasSprite>) -> () {
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

fn user_drawing_system(
    position: Res<CursorPosition>,
    prev_position: Res<PrevCursorPosition>,
    keyboard: Res<Input<KeyCode>>,
    mut cell_states: ResMut<CellStates>,
    mut map_query: Query<(&MapComponent, &mut Children)>,
    mut cell_query: Query<(&mut CellComponent, &mut TextureAtlasSprite)>,
) {
    let (_, children) = map_query.single_mut();
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

fn check_cell(
    current_state: CellState,
    live_neighbors: u8,
    infected_neighbors: u8,
    rules: GameRules,
) -> CellState {
    if rules == GameRules::default() {
        match current_state {
            CellState::Alive => {
                if live_neighbors < 2 {
                    CellState::Dead
                } else if live_neighbors == 2 || live_neighbors == 3 {
                    CellState::Alive
                } else if live_neighbors > 3 || infected_neighbors >= rules.virulence {
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
            CellState::Alive => match rules.living_rule {
                Rules::Single(_) => {
                    if live_neighbors < rules.living_rule.value()[0] {
                        CellState::Dead
                    } else if live_neighbors == rules.living_rule.value()[0]
                        || live_neighbors == rules.living_rule.value()[0] + 1
                    {
                        CellState::Alive
                    } else if live_neighbors > rules.living_rule.value()[0] + 1
                        || infected_neighbors >= rules.virulence
                    {
                        CellState::Infected
                    } else {
                        CellState::Alive
                    }
                }
                Rules::Range { min: _, max: _ } => {
                    if !rules.living_rule.in_range(&live_neighbors) {
                        CellState::Dead
                    } else if live_neighbors > rules.living_rule.value()[1] + 1
                        || infected_neighbors >= rules.virulence
                    {
                        CellState::Infected
                    } else {
                        CellState::Alive
                    }
                }
                Rules::Singles(_) => {
                    if !rules.living_rule.in_range(&live_neighbors) {
                        CellState::Dead
                    } else if live_neighbors > *rules.living_rule.value().last().unwrap()
                        || infected_neighbors >= rules.virulence
                    {
                        CellState::Infected
                    } else {
                        CellState::Alive
                    }
                }
                _ => CellState::default(),
            },
            CellState::Dead => match rules.dead_rule {
                Rules::Single(_) => {
                    if live_neighbors == rules.dead_rule.value()[0] {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    }
                }
                Rules::Range { min: _, max: _ } => {
                    if rules.dead_rule.in_range(&live_neighbors) {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    }
                }
                Rules::Singles(_) => {
                    if rules.dead_rule.in_range(&live_neighbors) {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    }
                }
                _ => CellState::default(),
            },
            CellState::Infected => {
                if live_neighbors < rules.virulence + 1 {
                    CellState::Dead
                } else {
                    CellState::Infected
                }
            }
        }
    }
}

fn generation_system(
    mut cell_states: ResMut<CellStates>,
    map_query: Query<(&MapComponent, &mut Children)>,
    mut cell_query: Query<(&mut CellComponent, &mut TextureAtlasSprite)>,
    rule: Res<Rule>,
) {
    let (_, children) = map_query.single();
    for &child in children.iter() {
        let (mut entity, sprite) = cell_query.get_mut(child).unwrap();
        let mut cell = cell_states.0[entity.coord.0][entity.coord.1];
        let mut live_neighbors = 0;
        let mut infected_neighbors = 0;
        // begin neighbour count
        if entity.coord.0 > 0 {
            match cell_states.0[entity.coord.0 - 1][entity.coord.1] {
                CellState::Alive => live_neighbors += 1,
                CellState::Infected => infected_neighbors += 1,
                _ => (),
            }
        }
        if entity.coord.0 > 0 && entity.coord.1 > 0 {
            match cell_states.0[entity.coord.0 - 1][entity.coord.1 - 1] {
                CellState::Alive => live_neighbors += 1,
                CellState::Infected => infected_neighbors += 1,
                _ => (),
            }
        }
        if entity.coord.1 > 0 {
            match cell_states.0[entity.coord.0][entity.coord.1 - 1] {
                CellState::Alive => live_neighbors += 1,
                CellState::Infected => infected_neighbors += 1,
                _ => (),
            }
        }
        if entity.coord.0 < MAP_SIZE.0 - 1 && entity.coord.1 > 0 {
            match cell_states.0[entity.coord.0 + 1][entity.coord.1 - 1] {
                CellState::Alive => live_neighbors += 1,
                CellState::Infected => infected_neighbors += 1,
                _ => (),
            }
        }
        if entity.coord.0 < MAP_SIZE.0 - 1 {
            match cell_states.0[entity.coord.0 + 1][entity.coord.1] {
                CellState::Alive => live_neighbors += 1,
                CellState::Infected => infected_neighbors += 1,
                _ => (),
            }
        }
        if entity.coord.0 < MAP_SIZE.0 - 1 && entity.coord.1 < MAP_SIZE.1 - 1 {
            match cell_states.0[entity.coord.0 + 1][entity.coord.1 + 1] {
                CellState::Alive => live_neighbors += 1,
                CellState::Infected => infected_neighbors += 1,
                _ => (),
            }
        }
        if entity.coord.1 < MAP_SIZE.1 - 1 {
            match cell_states.0[entity.coord.0][entity.coord.1 + 1] {
                CellState::Alive => live_neighbors += 1,
                CellState::Infected => infected_neighbors += 1,
                _ => (),
            }
        }
        if entity.coord.0 > 0 && entity.coord.1 < MAP_SIZE.1 - 1 {
            match cell_states.0[entity.coord.0 - 1][entity.coord.1 + 1] {
                CellState::Alive => live_neighbors += 1,
                CellState::Infected => infected_neighbors += 1,
                _ => (),
            }
        }
        // end neighbour count
        let cell_state = check_cell(cell, live_neighbors, infected_neighbors, rule.0.clone());
        if cell != cell_state {
            cell = cell_state;
            cell_states.0[entity.coord.0][entity.coord.1] = cell;
            entity.state = cell;
            update_sprite(entity.state, sprite);
        }
    }
}
