use crate::camera::create_camera;
use crate::constants::*;
use crate::player::Player;
use macroquad::prelude::*;

pub struct Game {
    player: Player,
    camera: Camera2D,
}

impl Game {
    pub async fn new() -> Self {
        let player = Player::new();

        let camera = create_camera(player.position);

        Self { player, camera }
    }

    pub fn update(&mut self) {
        self.player.update();

        self.camera.target = self.player.position;
    }

    pub fn draw(&self) {
        set_camera(&self.camera);
        for x in (0..WORLD_WIDTH as i32).step_by(32) {
            draw_line(x as f32, 0.0, x as f32, WORLD_HEIGHT, 1.0, DARKGRAY);
        }

        for y in (0..WORLD_HEIGHT as i32).step_by(32) {
            draw_line(0.0, y as f32, WORLD_WIDTH, y as f32, 1.0, DARKGRAY);
        }

        self.player.draw();

        set_default_camera();
    }
}
