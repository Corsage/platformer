use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::utils::vectors::Vector3Int;

use self::components::Player;

use super::{map::components::Position, GameState};

pub mod components;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_player.in_schedule(OnEnter(GameState::Running)));
    }
}

fn load_player(mut commands: Commands) {
    info!("Loading player...");

    // TODO: This has Rapier data, maybe move elsewhere?
    commands.spawn((
        Player,
        Position {
            v: Vector3Int::new(0, 6, 5), // Temp z-index.
        },
        RigidBody::Dynamic,
        Collider::ball(9.0),
    ));
}
