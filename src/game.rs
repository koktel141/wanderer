use crate::camera::create_camera;
use crate::constants::*;
use crate::enemy::Wolf;
use crate::player::Player;
use crate::world::World;
use macroquad::prelude::*;

pub struct Game {
    player: Player,
    camera: Camera2D,
    world: World,
    wolf: Wolf,
}

impl Game {
    pub async fn new() -> Self {
        let world = World::new().await;
        let player = Player::new().await;
        let camera = create_camera(player.position);
        let wolf = Wolf::new(700.0, 500.0);
        Self {
            world,
            player,
            camera,
            wolf,
        }
    }

pub fn update(&mut self) {
    self.player.update(&self.world);
    self.wolf.update(self.player.position);
    self.camera.target = self.player.position;
}

    pub fn draw(&self) {
        set_camera(&self.camera);
        self.world.draw();
        self.world.draw_colliders();
        self.player.draw();
        self.wolf.draw();

        set_default_camera();
    }
}
