use bevy::{asset::LoadState, prelude::*};
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};

use crate::AssetList;

use self::{map::MapPlugin, player::PlayerPlugin};

pub mod map;
pub mod player;

pub struct GamePlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    None,
    Loading,
    Paused,
    Running,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(18.0)) // TODO: Temporary.
            .insert_resource(RapierConfiguration {
                gravity: Vec2::Y * -980.0, // TODO: Temporary.
                ..default()
            })
            .add_plugin(PlayerPlugin)
            .add_plugin(MapPlugin)
            .add_system(check_asset_loading.in_set(OnUpdate(GameState::Loading)));
    }
}

fn check_asset_loading(
    server: Res<AssetServer>,
    assets: Res<AssetList>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match server.get_group_load_state(assets.0.iter().map(|a| a.id())) {
        LoadState::Loaded => {
            info!("Loaded {} assets.", assets.0.len());
            next_state.set(GameState::Running);
        }
        LoadState::Failed => {
            error!("Failed to load assets.");
        }
        _ => {}
    };
}
