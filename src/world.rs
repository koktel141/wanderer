use crate::constants::*;
use crate::map;
use crate::object::{ObjectType, WorldObject};
use crate::tile::TileType;
use macroquad::prelude::*;

pub struct World {
    tiles: Vec<Vec<TileType>>,
    tileset: Texture2D,
    objects: Vec<WorldObject>,
    object_atlas: Texture2D,
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

        let object_atlas = load_texture("assets/objects/objects.png").await.unwrap();
        object_atlas.set_filter(FilterMode::Nearest);

        let objects = Self::scatter_objects(&tiles);

        Self {
            tiles,
            tileset,
            objects,
            object_atlas,
        }
    }

    fn scatter_objects(tiles: &[Vec<TileType>]) -> Vec<WorldObject> {
        let mut objects: Vec<WorldObject> = Vec::new();

        let spawn_x = (SCREEN_WIDTH / 2.0 / TILE_SIZE) as i32;
        let spawn_y = (SCREEN_HEIGHT / 2.0 / TILE_SIZE) as i32;
        let spawn_clear_radius = 6;
        let min_spacing_tiles = 2.0; // حداقل فاصله بین دو آبجکت، بر حسب تایل

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if tiles[y][x] != TileType::Grass {
                    continue;
                }

                let dx = x as i32 - spawn_x;
                let dy = y as i32 - spawn_y;
                if dx * dx + dy * dy < spawn_clear_radius * spawn_clear_radius {
                    continue;
                }

                let roll = map::hash01(x as i32, y as i32, 999);
                if roll >= 0.04 {
                    continue;
                }

                let candidate_pos = vec2(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);

                // چک کن هیچ آبجکت دیگه‌ای تو شعاع min_spacing نباشه، وگرنه رد کن
                let too_close = objects.iter().any(|o: &WorldObject| {
                    o.position.distance(candidate_pos) < min_spacing_tiles * TILE_SIZE
                });
                if too_close {
                    continue;
                }

                let type_roll = map::hash01(x as i32, y as i32, 1000);
                let object_type = if type_roll < 0.5 {
                    ObjectType::Tree
                } else if type_roll < 0.8 {
                    ObjectType::Rock
                } else {
                    ObjectType::Bush
                };

                objects.push(WorldObject {
                    position: candidate_pos,
                    object_type,
                });
            }
        }

        objects.sort_by(|a, b| a.position.y.partial_cmp(&b.position.y).unwrap());

        objects
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

        for obj in &self.objects {
            let atlas_x = obj.object_type.atlas_index() as f32 * OBJECT_FRAME_WIDTH;
            let draw_y = obj.position.y + TILE_SIZE - OBJECT_FRAME_HEIGHT;

            draw_texture_ex(
                &self.object_atlas,
                obj.position.x,
                draw_y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        atlas_x,
                        0.0,
                        OBJECT_FRAME_WIDTH,
                        OBJECT_FRAME_HEIGHT,
                    )),
                    dest_size: Some(vec2(OBJECT_FRAME_WIDTH, OBJECT_FRAME_HEIGHT)),
                    ..Default::default()
                },
            );
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

        for obj in &self.objects {
            if !obj.object_type.is_solid() {
                continue;
            }
            let obj_rect = Rect::new(obj.position.x, obj.position.y, TILE_SIZE, TILE_SIZE);
            if obj_rect.overlaps(&rect) {
                return true;
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

        for obj in &self.objects {
            if obj.object_type.is_solid() {
                draw_rectangle_lines(
                    obj.position.x,
                    obj.position.y,
                    TILE_SIZE,
                    TILE_SIZE,
                    2.0,
                    ORANGE,
                );

                let label = match obj.object_type {
                    ObjectType::Tree => "TREE",
                    ObjectType::Rock => "ROCK",
                    ObjectType::Bush => "BUSH",
                };
                draw_text(label, obj.position.x, obj.position.y - 4.0, 16.0, MAGENTA);
            }
        }
    }
}
