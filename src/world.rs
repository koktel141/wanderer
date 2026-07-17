use crate::constants::*;
use crate::map;
use crate::tile::TileType;
use macroquad::prelude::*;

pub struct World {
    tiles: Vec<Vec<TileType>>,
    tileset: Texture2D,
}

impl World {
    pub async fn new() -> Self {
        let raw_map = map::create_map();

        let tiles: Vec<Vec<TileType>> = raw_map
            .iter()
            .map(|row| row.iter().map(|&v| TileType::from_u8(v)).collect())
            .collect();

        let tileset = load_texture("assets/maps/tileset.png").await.unwrap();
        tileset.set_filter(FilterMode::Nearest);

        Self { tiles, tileset }
    }

    pub fn draw(&self) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let px = x as f32 * TILE_SIZE;
                let py = y as f32 * TILE_SIZE;
                let atlas_x = tile.atlas_index() as f32 * TILE_SIZE;

                draw_texture_ex(
                    &self.tileset,
                    px,
                    py,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(atlas_x, 0.0, TILE_SIZE, TILE_SIZE)),
                        dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    pub fn check_collision(&self, rect: Rect) -> bool {
        let left = (rect.x / TILE_SIZE).floor() as i32;
        let right = ((rect.x + rect.w) / TILE_SIZE).floor() as i32;
        let top = (rect.y / TILE_SIZE).floor() as i32;
        let bottom = ((rect.y + rect.h) / TILE_SIZE).floor() as i32;

        for ty in top..=bottom {
            for tx in left..=right {
                if ty < 0 || tx < 0 {
                    return true;
                }
                let (ty, tx) = (ty as usize, tx as usize);
                if ty >= self.tiles.len() || tx >= self.tiles[0].len() {
                    return true;
                }
                if self.tiles[ty][tx].is_solid() {
                    return true;
                }
            }
        }
        false
    }

    pub fn draw_colliders(&self) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.is_solid() {
                    let px = x as f32 * TILE_SIZE;
                    let py = y as f32 * TILE_SIZE;
                    draw_rectangle_lines(px, py, TILE_SIZE, TILE_SIZE, 2.0, RED);
                }
            }
        }
    }
}