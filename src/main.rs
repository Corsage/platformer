use bevy::prelude::*;
use game::GamePlugin;
use graphics::GraphicsPlugin;
use splash::SplashPlugin;
use utils::UtilsPlugin;

pub mod game;
mod graphics;
mod splash;
pub mod utils;

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<HandleUntyped>);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Splash,
    Menu,
    Game,
    Error,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Add all utils.
        .add_plugin(UtilsPlugin)
        .add_state::<AppState>()
        .init_resource::<AssetList>()
        .add_plugin(GraphicsPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(SplashPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

// Camera test.
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}
