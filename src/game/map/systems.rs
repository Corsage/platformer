use bevy::{
    prelude::{info, AssetServer, Assets, Commands, Entity, Res, ResMut, Vec2},
    sprite::TextureAtlas,
};
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::{graphics::TILE_SIZE, utils::vectors::Vector3Int, AssetList};

use super::{
    components::{Position, Tile},
    resources::{Board, SceneHandle, SceneInfoHandle, TileMapAsset, Tilemap},
    structs::{Scene, SceneInfo},
};

pub fn load_scene_data(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut atlas: ResMut<Assets<TextureAtlas>>,
    mut assets: ResMut<AssetList>,
) {
    let scene = server.load("maps\\test.scene.json");
    let scene_info = server.load("tiles\\map\\base\\data.tiles.json");
    let texture = server.load("tiles\\map\\base\\tilemap.png");

    assets.0.push(scene.clone_untyped());
    assets.0.push(scene_info.clone_untyped());
    assets.0.push(texture.clone_untyped());

    // TODO: Make tile size, columns, rows dynamic from data file.
    let map = TextureAtlas::from_grid(texture, Vec2::splat(TILE_SIZE), 20, 9, None, None);
    let handle = atlas.add(map);

    // Add the graphic asset.
    commands.insert_resource(TileMapAsset {
        tiles_texture: handle,
    });

    // Add the data asset.
    commands.insert_resource(SceneHandle(scene));
    commands.insert_resource(SceneInfoHandle(scene_info));
}

pub fn load_scene(
    mut commands: Commands,
    scene: Res<SceneHandle>,
    scene_info: Res<SceneInfoHandle>, // Maybe remove later on?
    mut scene_infos: ResMut<Assets<SceneInfo>>, // Maybe remove later on?
    mut scenes: ResMut<Assets<Scene>>,
    mut current: ResMut<Board>,
    mut tilemap: ResMut<Tilemap>, // Maybe remove later on?
) {
    // First get the tilemap data.
    if let Some(scene_info) = scene_infos.remove(scene_info.0.id()) {
        for tile in scene_info.tiles.iter() {
            tilemap.tiles.insert(tile.id, tile.to_owned());
        }
    }

    // Load the map tiles after getting tilemap data.
    if let Some(scene) = scenes.remove(scene.0.id()) {
        // Load scene layer by layer, increasing the z-index as we do.
        let mut z: i32 = 0;
        for layer in scene.layers.iter() {
            for (pos, i) in layer.iter().enumerate() {
                // Calculate y from width.
                // Note: (0, 0) is actually centered.
                // In order to center the map which is 30 x 20
                // we have to start at (-16, -16).
                let index: i32 = (*i as i32) - 1;
                if index >= 0 {
                    let x = (pos as i32 % 30) - 15;
                    let y = 10 - (pos as i32) / 30;

                    let v = Vector3Int::new(x, y, z);

                    let entity: Entity;

                    // See if this is a special tile.
                    if tilemap.tiles.contains_key(&(index as u16)) {
                        let tile = tilemap.tiles.get(&(index as u16)).unwrap().clone();
                        info!("Found tile with data {:?}", tile);

                        let mut command = commands.spawn_empty();

                        if tile.type_field.clone().unwrap() == String::from("floor") {
                            info!("Found floor tile.");
                            command
                                .commands()
                                .spawn((RigidBody::Fixed, Collider::cuboid(9.0, 9.0)));
                        }

                        command.commands().spawn((Position { v }, tile));
                        entity = command.id();
                    } else {
                        entity = commands
                            .spawn((
                                Position { v },
                                Tile {
                                    id: index as u16,
                                    type_field: None,
                                    properties: None,
                                },
                            ))
                            .id();
                    }
                    current.tiles.insert(v, entity);
                }
            }
        }
    }
}
