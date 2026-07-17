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
        let player = Player::new().await;

        let camera = create_camera(player.position);

        Self { player, camera }
    }

    pub fn update(&mut self) {
        self.player.update();

        self.camera.target = self.player.position;
    }

    pub fn draw(&self) {
        set_camera(&self.camera);

        self.player.draw();

        set_default_camera();
    }
}
