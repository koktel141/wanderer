use crate::constants::*;
use macroquad::prelude::*;

pub struct Player {
    pub position: Vec2,
    speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            speed: PLAYER_SPEED,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        let mut direction = Vec2::ZERO;

        if is_key_down(KeyCode::W) {
            direction.y -= 1.0;
        }

        if is_key_down(KeyCode::S) {
            direction.y += 1.0;
        }

        if is_key_down(KeyCode::A) {
            direction.x -= 1.0;
        }

        if is_key_down(KeyCode::D) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        self.position += direction * self.speed * dt;
        self.position.x = self.position.x.clamp(0.0, WORLD_WIDTH - PLAYER_SIZE);

        self.position.y = self.position.y.clamp(0.0, WORLD_HEIGHT - PLAYER_SIZE);
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            PLAYER_SIZE,
            PLAYER_SIZE,
            BLUE,
        );
    }
}
