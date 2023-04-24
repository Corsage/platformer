use super::components::Tile;

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "44df1799-4d08-40fb-8f28-095719311b5d"]
pub struct Scene {
    pub layers: Vec<Vec<usize>>,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "7eea7f44-3e9f-41dc-a8cc-7a29008ed390"]
pub struct SceneInfo {
    pub name: String,
    pub columns: i64,
    pub rows: i64,
    pub margin: i64,
    pub spacing: i64,
    pub tiles: Vec<Tile>,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone)]
#[uuid = "7190e65d-0023-44b1-bb17-a7077bf1b85b"]
pub struct Property {
    pub name: String,
    #[serde(rename = "type")]
    pub effect: String,
    pub value: f64,
}
