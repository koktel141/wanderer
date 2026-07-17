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
    wolves: Vec<Wolf>,
}

impl Game {
    pub async fn new() -> Self {
        let world = World::new().await;
        let player = Player::new().await;
        let camera = create_camera(player.position);

        let wolves = vec![
            Wolf::new(700.0, 500.0),
            Wolf::new(900.0, 300.0),
            Wolf::new(500.0, 700.0),
        ];

        Self {
            world,
            player,
            camera,
            wolves,
        }
    }

    pub fn update(&mut self) {
        self.player.update(&self.world);

        for wolf in self.wolves.iter_mut() {
            wolf.update(&mut self.player);
        }

        self.camera.target = self.player.position;
    }

    pub fn draw(&self) {
        set_camera(&self.camera);
        self.world.draw();
        self.world.draw_colliders();
        self.player.draw();

        for wolf in &self.wolves {
            wolf.draw();
        }

        set_default_camera();

        // HP رو ثابت رو صفحه بکش، نه وابسته به دوربین دنیا
        let hp_color = if self.player.is_invincible() { YELLOW } else { WHITE };
        draw_text(
            &format!("HP: {}/{}", self.player.hp(), PLAYER_MAX_HP),
            20.0,
            30.0,
            28.0,
            hp_color,
        );

        if !self.player.is_alive() {
            draw_text("YOU DIED", SCREEN_WIDTH / 2.0 - 80.0, SCREEN_HEIGHT / 2.0, 40.0, RED);
        }
    }
}