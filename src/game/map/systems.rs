use bevy::{
    prelude::{AssetServer, Assets, Commands, Res, ResMut, Vec2},
    sprite::TextureAtlas,
};

use crate::{graphics::TILE_SIZE, utils::vectors::Vector3Int, AssetList};

use super::{
    components::{Position, Tile},
    resources::{Board, SceneHandle, TileMapAsset},
    Scene,
};

pub fn load_scene_data(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut atlas: ResMut<Assets<TextureAtlas>>,
    mut assets: ResMut<AssetList>,
) {
    let scene = server.load("maps\\test.json");
    let texture = server.load("tilemaps\\base.png");

    assets.0.push(scene.clone_untyped());
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
}

pub fn load_scene(
    mut commands: Commands,
    scene: Res<SceneHandle>,
    mut scenes: ResMut<Assets<Scene>>,
    mut current: ResMut<Board>,
) {
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
                    let tile = commands
                        .spawn((Position { v }, Tile { i: index as usize }))
                        .id(); // Offset by 1.
                    current.tiles.insert(v, tile);
                }
            }
        }
    }
}
