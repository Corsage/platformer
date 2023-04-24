use bevy::prelude::*;

use crate::game::GameState;

use self::systems::{map, player};

mod systems;

pub const TILE_SIZE: f32 = 18.;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(map::spawn_scene_renderer.in_set(OnUpdate(GameState::Running)))
            .add_system(player::spawn_player_renderer.in_set(OnUpdate(GameState::Running)));
    }
}
