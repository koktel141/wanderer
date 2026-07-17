use crate::constants::*;
use macroquad::prelude::*;

pub struct Wolf {
    pub position: Vec2,
    speed: f32,
}

impl Wolf {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: vec2(x, y),
            speed: WOLF_SPEED,
        }
    }

    pub fn update(&mut self, player_pos: Vec2) {
        let dt = get_frame_time();

        let direction = player_pos - self.position;

        if direction.length() > 5.0 {
            self.position += direction.normalize() * self.speed * dt;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x + 16.0, self.position.y + 16.0, 16.0, RED);
    }
}
