use bevy::prelude::*;
use std::{fs::File, io::BufRead, io::BufReader};

use crate::{
    ascii::spawn_ascii_sprite, components::AsciiSheet, components::Tile, components::TileCollider,
    level::get_current_level, TILE_SIZE,
};

const MAP_SIZE: (usize, usize) = (50, 50);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_map_system);
    }
}

fn create_map_system(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut map: [[u8; MAP_SIZE.0]; MAP_SIZE.1];

    // let file = File::open(get_current_level()).expect("No map file found");
    // let mut tiles = Vec::new();

    // for (y, line) in BufReader::new(file).lines().enumerate() {
    //     if let Ok(line) = line {
    //         for (x, char) in line.chars().enumerate() {
    //             let tile = spawn_ascii_sprite(
    //                 &mut commands,
    //                 &ascii,
    //                 match char {
    //                     '#' => 0,
    //                     _ => 32,
    //                 },
    //                 Color::rgb(0.9, 0.9, 0.9),
    //                 Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.),
    //             );
    //             match char {
    //                 '#' => commands
    //                     .entity(tile)
    //                     .insert(Tile {
    //                         tile_id: 1,
    //                         sprite: tile,
    //                         // sprite: spawn_ascii_sprite(
    //                         //     &mut commands,
    //                         //     &ascii,
    //                         //     0,
    //                         //     Color::rgb(0.9, 0.9, 0.9),
    //                         //     translation,
    //                         // ),
    //                         is_transparent: false,
    //                         is_kill_plane: false,
    //                     })
    //                     .insert(TileCollider),
    //                 'x' => commands
    //                     .entity(tile)
    //                     .insert(Tile {
    //                         tile_id: 2,
    //                         sprite: tile,
    //                         // sprite: spawn_ascii_sprite(
    //                         //     &mut commands,
    //                         //     &ascii,
    //                         //     33,
    //                         //     Color::rgb(0., 0., 0.),
    //                         //     translation,
    //                         // ),
    //                         is_transparent: true,
    //                         is_kill_plane: false,
    //                     })
    //                     .insert(TileCollider),
    //                 's' => commands
    //                     .entity(tile)
    //                     .insert(Tile {
    //                         tile_id: 3,
    //                         sprite: tile,
    //                         // sprite: spawn_ascii_sprite(
    //                         //     &mut commands,
    //                         //     &ascii,
    //                         //     33,
    //                         //     Color::rgb(0., 0., 0.),
    //                         //     translation,
    //                         // ),
    //                         is_transparent: true,
    //                         is_kill_plane: true,
    //                     })
    //                     .insert(TileCollider),
    //                 _ => commands.entity(tile).insert(Tile {
    //                     tile_id: 0,
    //                     sprite: tile,
    //                     // sprite: spawn_ascii_sprite(
    //                     //     &mut commands,
    //                     //     &ascii,
    //                     //     33,
    //                     //     Color::rgb(0., 0., 0.),
    //                     //     translation,
    //                     // ),
    //                     is_transparent: true,
    //                     is_kill_plane: false,
    //                 }),
    //             };
    //             tiles.push(tile);
    //         }
    //     }
    // }

    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(ComputedVisibility::default())
        .insert(Visibility::visible())
        .push_children(&tiles);
}
