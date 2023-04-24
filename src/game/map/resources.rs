use bevy::{
    prelude::{Entity, Handle, Resource},
    sprite::TextureAtlas,
    utils::HashMap,
};

use crate::utils::vectors::Vector3Int;

use super::{components::Tile, structs::Scene, structs::SceneInfo};

#[derive(Resource)]
pub struct SceneHandle(pub Handle<Scene>);

#[derive(Resource)]
pub struct SceneInfoHandle(pub Handle<SceneInfo>);

#[derive(Resource)]
pub struct TileMapAsset {
    pub tiles_texture: Handle<TextureAtlas>,
}

#[derive(Default, Resource)]
pub struct Board {
    pub tiles: HashMap<Vector3Int, Entity>,
}

#[derive(Default, Resource)]
pub struct Tilemap {
    pub tiles: HashMap<u16, Tile>,
}
