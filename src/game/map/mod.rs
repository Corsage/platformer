use bevy::prelude::*;

use crate::graphics::TILE_SIZE;

use self::{
    components::Position,
    resources::{Board, Tilemap},
    systems::{load_scene, load_scene_data},
};

use super::GameState;

pub mod components;
pub mod resources;
pub mod structs;
mod systems;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tilemap>()
            .init_resource::<Board>()
            .add_system(load_scene_data.in_schedule(OnEnter(GameState::Loading)))
            .add_system(load_scene.in_schedule(OnEnter(GameState::Running)));
    }
}

pub fn get_world_position(position: &Position) -> Vec3 {
    Vec3::new(
        TILE_SIZE * position.v.x as f32,
        TILE_SIZE * position.v.y as f32,
        position.v.z as f32,
    )
}
