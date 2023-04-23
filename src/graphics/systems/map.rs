use bevy::prelude::*;

use crate::{
    game::map::{
        components::{Position, Tile},
        get_world_position,
        resources::TileMapAsset,
    },
    graphics::TILE_SIZE,
};

pub fn spawn_scene_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Tile, &Position), Added<Tile>>,
    assets: Res<TileMapAsset>,
) {
    for (entity, tile, position) in query.iter() {
        let mut sprite = TextureAtlasSprite::new(tile.i);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

        let v = get_world_position(&position);

        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.tiles_texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}
