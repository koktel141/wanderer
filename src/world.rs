use macroquad::prelude::*;

pub struct World {
    pub texture: Texture2D,
    colliders: Vec<Rect>,
}

impl World {
    pub async fn new() -> Self {
        let texture = load_texture("assets/maps/terrain.png").await.unwrap();

        texture.set_filter(FilterMode::Nearest);

        let mut colliders = Vec::new();

        colliders.push(Rect::new(400.0, 400.0, 200.0, 200.0));

        Self { texture, colliders }
    }
    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(256.0 * 4.0, 448.0 * 4.0)),
                ..Default::default()
            },
        );
    }
    pub fn check_collision(&self, rect: Rect) -> bool {
        self.colliders.iter().any(|c| c.overlaps(&rect))
    }

    pub fn draw_colliders(&self) {
        for c in &self.colliders {
            draw_rectangle_lines(c.x, c.y, c.w, c.h, 2.0, RED);
        }
    }
}
