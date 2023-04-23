use bevy::{asset::LoadState, prelude::*};

use crate::AssetList;

use self::map::MapPlugin;

pub mod map;

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
