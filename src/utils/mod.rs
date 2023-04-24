use bevy::prelude::{App, Plugin};
use bevy_common_assets::json::JsonAssetPlugin;

use crate::game::map::structs::{Scene, SceneInfo};

pub mod vectors;

pub struct UtilsPlugin;
impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        // Load custom map scenes.
        app.add_plugin(JsonAssetPlugin::<Scene>::new(&["scene.json"]))
            .add_plugin(JsonAssetPlugin::<SceneInfo>::new(&["tiles.json"]));
    }
}
