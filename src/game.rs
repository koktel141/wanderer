use crate::camera::create_camera;
use crate::constants::*;
use crate::enemy::Wolf;
use crate::npc::Npc;
use crate::player::Player;
use crate::quest::Quest;
use crate::state::GameState;
use crate::world::World;
use macroquad::audio::{Sound, load_sound, play_sound_once};
use macroquad::prelude::*;

const WOLF_SPAWNS: [(f32, f32); 3] = [(700.0, 500.0), (900.0, 300.0), (500.0, 700.0)];

pub struct Game {
    player: Player,
    camera: Camera2D,
    world: World,
    wolves: Vec<Wolf>,
    npc: Npc,
    quest: Quest,
    state: GameState,
    hit_sound: Sound,
    complete_sound: Sound,
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

        let npc = Npc::new(player.position.x + 150.0, player.position.y);

        let hit_sound = load_sound("assets/sounds/hit.wav").await.unwrap();
        let complete_sound = load_sound("assets/sounds/complete.wav").await.unwrap();

        Self {
            world,
            player,
            camera,
            wolves,
            npc,
            quest: Quest::new(),
            state: GameState::Playing,
            hit_sound,
            complete_sound,
        }
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::Playing => {
                self.player.update(&self.world);

                for wolf in self.wolves.iter_mut() {
                    wolf.update(&mut self.player, &self.world);
                }

                if is_key_pressed(KeyCode::Space) {
                    let player_rect = self.player.rect();
                    for wolf in self.wolves.iter_mut() {
                        if wolf.rect().overlaps(&Rect::new(
                            player_rect.x - PLAYER_ATTACK_RANGE / 2.0,
                            player_rect.y - PLAYER_ATTACK_RANGE / 2.0,
                            player_rect.w + PLAYER_ATTACK_RANGE,
                            player_rect.h + PLAYER_ATTACK_RANGE,
                        )) {
                            wolf.take_damage(999); 
                            play_sound_once(&self.hit_sound);
                        }
                    }
                }

                let quest_just_completed_now = {
                    let mut completed_this_frame = false;
                    self.wolves.retain(|w| {
                        if w.is_dead() {
                            completed_this_frame =
                                self.quest.register_kill() || completed_this_frame;
                            false
                        } else {
                            true
                        }
                    });
                    completed_this_frame
                };

                if quest_just_completed_now {
                    play_sound_once(&self.complete_sound);
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
        self.npc.draw();

        for wolf in &self.wolves {
            wolf.draw();
        }

        set_default_camera();

        let hp_color = if self.player.is_invincible() {
            YELLOW
        } else {
            WHITE
        };
        draw_text(
            &format!("HP: {}/{}", self.player.hp(), PLAYER_MAX_HP),
            20.0,
            30.0,
            28.0,
            hp_color,
        );

        draw_text(&self.quest.description(), 20.0, 60.0, 22.0, GOLD);

        if self.npc.is_player_nearby(self.player.position) {
            draw_text(
                "Talk to the NPC (quest: defeat wolves)",
                20.0,
                90.0,
                20.0,
                WHITE,
            );
        }

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
