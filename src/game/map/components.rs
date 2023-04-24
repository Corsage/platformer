use bevy::prelude::Component;

use crate::utils::vectors::Vector3Int;

use super::structs::Property;

#[derive(Component)]
pub struct Position {
    pub v: Vector3Int,
}

#[derive(Component, serde::Deserialize, bevy::reflect::TypeUuid, Debug, Default, Clone)]
#[uuid = "6d0c2ae7-4334-4ef7-8153-a5872b7d5125"]
pub struct Tile {
    pub id: u16,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(default)]
    pub properties: Option<Vec<Property>>,
}
