use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Grass,
    Water,
    Dirt,
}

impl TileType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => TileType::Water,
            2 => TileType::Dirt,
            _ => TileType::Grass,
        }
    }

    pub fn atlas_index(&self) -> u32 {
        match self {
            TileType::Grass => 0,
            TileType::Water => 1,
            TileType::Dirt => 2,
        }
    }

    pub fn is_solid(&self) -> bool {
        matches!(self, TileType::Water)
    }
}