use crate::camera::create_camera;
use crate::constants::*;
use crate::enemy::Wolf;
use crate::npc::Npc;
use crate::player::Player;
use crate::quest::Quest;
use crate::state::GameState;
use crate::world::World;
use macroquad::audio::{
    PlaySoundParams, Sound, load_sound, play_sound, play_sound_once, stop_sound,
};
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
    menu_theme: Sound,
    forest_ambience: Sound,
    music_started: bool,
    talking: bool,
    dialogue_index: usize,
    quest_banner_timer: f32,
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

        let npc = Npc::new(player.position.x + 150.0, player.position.y).await;

        let hit_sound = load_sound("assets/sounds/hit.wav").await.unwrap();
        let complete_sound = load_sound("assets/sounds/complete.wav").await.unwrap();
        let menu_theme = load_sound("assets/sounds/menu_theme.wav").await.unwrap();
        let forest_ambience = load_sound("assets/sounds/forest_ambience.wav")
            .await
            .unwrap();

        Self {
            world,
            player,
            camera,
            wolves,
            npc,
            quest: Quest::new(),
            state: GameState::MainMenu,
            hit_sound,
            complete_sound,
            menu_theme,
            forest_ambience,
            music_started: false,
            talking: false,
            dialogue_index: 0,
            quest_banner_timer: 0.0,
        }
    }

    pub fn update(&mut self) {
        if !self.music_started {
            play_sound(
                &self.menu_theme,
                PlaySoundParams {
                    looped: true,
                    volume: 0.5,
                },
            );
            self.music_started = true;
        }

        if self.quest_banner_timer > 0.0 {
            self.quest_banner_timer -= get_frame_time();
        }

        match self.state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Enter) {
                    stop_sound(&self.menu_theme);
                    play_sound(
                        &self.forest_ambience,
                        PlaySoundParams {
                            looped: true,
                            volume: 0.4,
                        },
                    );
                    self.state = GameState::Playing;
                }
            }

            GameState::Playing => {
                if self.talking {
                    if is_key_pressed(KeyCode::E) {
                        self.dialogue_index += 1;
                        if self.dialogue_index >= self.npc.line_count() {
                            self.talking = false;
                            self.dialogue_index = 0;
                            self.quest.start();
                        }
                    }
                    return;
                }

                if is_key_pressed(KeyCode::E) && self.npc.is_player_nearby(self.player.position) {
                    self.talking = true;
                    self.dialogue_index = 0;
                    return;
                }

                self.player.update(&self.world);

                for wolf in self.wolves.iter_mut() {
                    wolf.update(&mut self.player, &self.world, self.quest.is_active());
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
                    self.quest_banner_timer = 3.0;
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

        self.talking = false;
        self.dialogue_index = 0;
        self.quest_banner_timer = 0.0;
        self.camera.target = self.player.position;
        self.state = GameState::Playing;
    }

    pub fn draw(&self) {
        if self.state == GameState::MainMenu {
            clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

            draw_text(
                "WANDERER",
                SCREEN_WIDTH / 2.0 - 150.0,
                SCREEN_HEIGHT / 2.0 - 100.0,
                60.0,
                GOLD,
            );
            draw_text(
                "A tiny adventure in the woods",
                SCREEN_WIDTH / 2.0 - 160.0,
                SCREEN_HEIGHT / 2.0 - 55.0,
                24.0,
                WHITE,
            );

            draw_text(
                "WASD - Move",
                SCREEN_WIDTH / 2.0 - 90.0,
                SCREEN_HEIGHT / 2.0 + 10.0,
                20.0,
                GRAY,
            );
            draw_text(
                "SPACE - Attack",
                SCREEN_WIDTH / 2.0 - 90.0,
                SCREEN_HEIGHT / 2.0 + 35.0,
                20.0,
                GRAY,
            );
            draw_text(
                "E - Talk to NPC",
                SCREEN_WIDTH / 2.0 - 90.0,
                SCREEN_HEIGHT / 2.0 + 60.0,
                20.0,
                GRAY,
            );

            draw_text(
                "Press ENTER to start",
                SCREEN_WIDTH / 2.0 - 130.0,
                SCREEN_HEIGHT / 2.0 + 110.0,
                28.0,
                YELLOW,
            );
            return;
        }

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

        if !self.talking && self.npc.is_player_nearby(self.player.position) {
            draw_text("Press E to talk", 20.0, 90.0, 20.0, WHITE);
        }

        if self.talking {
            draw_rectangle(
                40.0,
                SCREEN_HEIGHT - 140.0,
                SCREEN_WIDTH - 80.0,
                100.0,
                Color::new(0.0, 0.0, 0.0, 0.85),
            );
            draw_rectangle_lines(
                40.0,
                SCREEN_HEIGHT - 140.0,
                SCREEN_WIDTH - 80.0,
                100.0,
                2.0,
                WHITE,
            );

            if let Some(line) = self.npc.line(self.dialogue_index) {
                draw_text(line, 60.0, SCREEN_HEIGHT - 95.0, 24.0, WHITE);
            }
            draw_text(
                "Press E to continue",
                60.0,
                SCREEN_HEIGHT - 55.0,
                18.0,
                GRAY,
            );
        }

        if self.quest_banner_timer > 0.0 {
            let alpha = (self.quest_banner_timer / 3.0).min(1.0);
            draw_rectangle(
                SCREEN_WIDTH / 2.0 - 220.0,
                SCREEN_HEIGHT / 2.0 - 60.0,
                440.0,
                80.0,
                Color::new(0.0, 0.0, 0.0, 0.7 * alpha),
            );
            draw_text(
                "QUEST COMPLETE!",
                SCREEN_WIDTH / 2.0 - 170.0,
                SCREEN_HEIGHT / 2.0 - 15.0,
                40.0,
                Color::new(1.0, 0.85, 0.2, alpha),
            );
            draw_text(
                "Wolf Hunter",
                SCREEN_WIDTH / 2.0 - 70.0,
                SCREEN_HEIGHT / 2.0 + 15.0,
                22.0,
                Color::new(1.0, 1.0, 1.0, alpha),
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
