use crate::camera::create_camera;
use crate::constants::*;
use crate::enemy::Wolf;
use crate::player::Player;
use crate::state::GameState;
use crate::world::World;
use macroquad::prelude::*;

const WOLF_SPAWNS: [(f32, f32); 3] = [(700.0, 500.0), (900.0, 300.0), (500.0, 700.0)];

pub struct Game {
    player: Player,
    camera: Camera2D,
    world: World,
    wolves: Vec<Wolf>,
    state: GameState,
}

impl Game {
pub async fn new() -> Self {
    let world = World::new().await;
    let player = Player::new().await;
    let camera = create_camera(player.position);

    let mut wolves = Vec::new();
    for &(x, y) in WOLF_SPAWNS.iter() {
        wolves.push(Wolf::new(x, y).await);
    }

    Self {
        world,
        player,
        camera,
        wolves,
        state: GameState::Playing,
    }
}

    pub fn update(&mut self) {
        match self.state {
            GameState::Playing => {
                self.player.update(&self.world);

                for wolf in self.wolves.iter_mut() {
                    wolf.update(&mut self.player);
                }

                self.camera.target = self.player.position;

                if !self.player.is_alive() {
                    self.state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Enter) {
                    self.restart();
                }
            }
        }
    }

    fn restart(&mut self) {
        self.player.reset();

        for (wolf, &(x, y)) in self.wolves.iter_mut().zip(WOLF_SPAWNS.iter()) {
            wolf.reset(x, y);
        }

        self.camera.target = self.player.position;
        self.state = GameState::Playing;
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

        let hp_color = if self.player.is_invincible() { YELLOW } else { WHITE };
        draw_text(
            &format!("HP: {}/{}", self.player.hp(), PLAYER_MAX_HP),
            20.0,
            30.0,
            28.0,
            hp_color,
        );

        if self.state == GameState::GameOver {
            draw_text(
                "YOU DIED",
                SCREEN_WIDTH / 2.0 - 80.0,
                SCREEN_HEIGHT / 2.0 - 20.0,
                40.0,
                RED,
            );
            draw_text(
                "Press ENTER to restart",
                SCREEN_WIDTH / 2.0 - 140.0,
                SCREEN_HEIGHT / 2.0 + 20.0,
                24.0,
                WHITE,
            );
        }
    }
}