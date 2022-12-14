use bevy::prelude::*;

use crate::{resources::AsciiSheet, TILE_SIZE};

pub struct AsciiPlugin;

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii_system);
    }
}

fn load_ascii_system(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // open a new texture atlas from the file
    let image = assets.load("spritesheet.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.),
        16,
        16,
        Vec2::splat(2.),
        Vec2::splat(0.),
    );
    let atlas_handle = texture_atlases.add(atlas);
    // save the texture atlas as a global resource
    commands.insert_resource(AsciiSheet(atlas_handle));
}

pub fn spawn_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
) -> Entity {
    // fetch sprite from texture atlas
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    // return the id of the texture atlas sprite
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}
