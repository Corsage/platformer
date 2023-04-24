use bevy::{asset::LoadState, prelude::*};

use crate::{game::GameState, AppState, AssetList};

pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>()
            .add_system(check_asset_loading.in_set(OnUpdate(AppState::Splash)));
    }
}

fn check_asset_loading(
    server: Res<AssetServer>,
    assets: Res<AssetList>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    match server.get_group_load_state(assets.0.iter().map(|a| a.id())) {
        LoadState::Loaded => {
            info!("Loaded {} assets.", assets.0.len());
            app_state.set(AppState::Game); // TODO: Testing game state, this is usually menu.
            game_state.set(GameState::Loading);
        }
        LoadState::Failed => {
            error!("Failed to load assets.");
        }
        _ => {}
    };
}
