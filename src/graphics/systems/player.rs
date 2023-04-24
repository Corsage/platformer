use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::game::{
    map::{components::Position, get_world_position},
    player::components::Player,
};

pub fn spawn_player_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Player>>,
    // assets: Res<GraphicsAssets>,
    mut meshes: ResMut<Assets<Mesh>>,             // TODO: Temporary.
    mut materials: ResMut<Assets<ColorMaterial>>, // TODO: Temporary.
) {
    let Ok((entity, position)) = query.get_single() else { return };

    // let mut sprite = TextureAtlasSprite::new(95);
    // sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let v = get_world_position(position);
    // commands.entity(entity).insert(SpriteSheetBundle {
    //     sprite,
    //     texture_atlas: assets.sprite_texture.clone(),
    //     transform: Transform::from_translation(v),
    //     ..Default::default()
    // });

    // TODO: Temporary.
    commands.entity(entity).insert(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Circle::new(9.0))).into(),
        material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
        transform: Transform::from_translation(v),
        ..Default::default()
    });
}
