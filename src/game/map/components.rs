use bevy::prelude::Component;

use crate::utils::vectors::Vector3Int;

#[derive(Component)]
pub struct Position {
    pub v: Vector3Int,
}

#[derive(Component)]
pub struct Tile {
    pub i: usize,
}
