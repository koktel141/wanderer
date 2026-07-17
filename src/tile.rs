use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum TileType {
    Grass,
    Dirt,
    Water,
}

pub struct Tile {
    pub tile_type: TileType,
    pub position: Vec2,
}

impl Tile {
    pub fn new(tile_type: TileType, x: f32, y: f32) -> Self {
        Self {
            tile_type,
            position: vec2(x, y),
        }
    }
}