use crate::animation::Animation;
use crate::constants::*;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub const PLAYER_SIZE: f32 = 12.0;

const ATTACK_FRAME_COUNT: usize = 6;
const ATTACK_FRAME_TIME: f32 = 0.08;
const HURT_FRAME_COUNT: usize = 3;
const HURT_FRAME_TIME: f32 = 0.12;

#[derive(PartialEq)]
enum PlayerState {
    Idle,
    Run,
    Attack,
    Hurt,
}

pub struct Player {
    pub position: Vec2,
    speed: f32,

    idle_animation: Animation,
    run_animation: Animation,
    attack_animations: Vec<Animation>,
    hurt_animation: Animation,

    state: PlayerState,
    facing_left: bool,
    attack_timer: f32,
    current_attack: usize,
    hurt_timer: f32,

    hp: i32,
    invincible_timer: f32,
}
impl Player {
    pub async fn new() -> Self {
        let attack_animations = vec![
            Animation::new(
                "assets/player/Woodcutter_attack1.png",
                48.0,
                48.0,
                ATTACK_FRAME_COUNT,
                ATTACK_FRAME_TIME,
            )
            .await,
            Animation::new(
                "assets/player/Woodcutter_attack2.png",
                48.0,
                48.0,
                ATTACK_FRAME_COUNT,
                ATTACK_FRAME_TIME,
            )
            .await,
            Animation::new(
                "assets/player/Woodcutter_attack3.png",
                48.0,
                48.0,
                ATTACK_FRAME_COUNT,
                ATTACK_FRAME_TIME,
            )
            .await,
        ];

        Self {
            position: vec2(PLAYER_SPAWN_X, PLAYER_SPAWN_Y),
            facing_left: false,
            state: PlayerState::Idle,
            hp: PLAYER_MAX_HP,
            invincible_timer: 0.0,
            speed: PLAYER_SPEED,

            idle_animation: Animation::new(
                "assets/player/Woodcutter_idle.png",
                48.0,
                48.0,
                4,
                0.15,
            )
            .await,
            run_animation: Animation::new("assets/player/Woodcutter_walk.png", 48.0, 48.0, 6, 0.10)
                .await,
            attack_animations,
            hurt_animation: Animation::new(
                "assets/player/Woodcutter_hurt.png",
                48.0,
                48.0,
                HURT_FRAME_COUNT,
                HURT_FRAME_TIME,
            )
            .await,

            attack_timer: 0.0,
            current_attack: 0,
            hurt_timer: 0.0,
        }
    }

    pub fn update(&mut self, world: &crate::world::World) {
        let dt = get_frame_time();

        if self.invincible_timer > 0.0 {
            self.invincible_timer -= dt;
        }

        // Hurt has priority: freeze movement/attack while playing the hurt animation
        if self.hurt_timer > 0.0 {
            self.hurt_timer -= dt;
            self.hurt_animation.update();
            if self.hurt_timer <= 0.0 {
                self.state = PlayerState::Idle;
            }
            return;
        }

        if self.attack_timer > 0.0 {
            self.attack_timer -= dt;
            self.attack_animations[self.current_attack].update();
            if self.attack_timer <= 0.0 {
                self.state = PlayerState::Idle;
            }
            return;
        }

        let mut moving = false;
        let mut direction = Vec2::ZERO;

        if is_key_down(KeyCode::W) {
            direction.y -= 1.0;
            moving = true;
        }
        if is_key_down(KeyCode::S) {
            direction.y += 1.0;
            moving = true;
        }
        if is_key_down(KeyCode::A) {
            direction.x -= 1.0;
            moving = true;
            self.facing_left = true;
        }
        if is_key_down(KeyCode::D) {
            direction.x += 1.0;
            moving = true;
            self.facing_left = false;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let velocity = direction * self.speed * dt;

        let mut next_x = self.position;
        next_x.x += velocity.x;
        let rect_x = Rect::new(next_x.x, self.position.y, PLAYER_SIZE, PLAYER_SIZE);
        if !world.check_collision(rect_x) {
            self.position.x = next_x.x;
        }

        let mut next_y = self.position;
        next_y.y += velocity.y;
        let rect_y = Rect::new(self.position.x, next_y.y, PLAYER_SIZE, PLAYER_SIZE);
        if !world.check_collision(rect_y) {
            self.position.y = next_y.y;
        }

        self.position.x = self.position.x.clamp(0.0, WORLD_WIDTH - PLAYER_SIZE);
        self.position.y = self.position.y.clamp(0.0, WORLD_HEIGHT - PLAYER_SIZE);

        if moving {
            self.state = PlayerState::Run;
            self.run_animation.update();
        } else {
            self.state = PlayerState::Idle;
            self.idle_animation.update();
        }
    }

    pub fn take_damage(&mut self, amount: i32) {
        if self.invincible_timer > 0.0 {
            return;
        }
        self.hp -= amount;
        if self.hp < 0 {
            self.hp = 0;
        }
        self.invincible_timer = PLAYER_INVINCIBILITY_TIME;

        // Interrupt whatever we were doing (including an attack) to play the hurt animation
        self.state = PlayerState::Hurt;
        self.hurt_timer = HURT_FRAME_COUNT as f32 * HURT_FRAME_TIME;
        self.hurt_animation.reset();
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn trigger_attack(&mut self) {
        // don't let a new attack interrupt getting hurt
        if self.hurt_timer > 0.0 {
            return;
        }
        self.current_attack = gen_range(0, 3) as usize % 3;
        self.attack_animations[self.current_attack].reset();
        self.state = PlayerState::Attack;
        self.attack_timer = ATTACK_FRAME_COUNT as f32 * ATTACK_FRAME_TIME;
    }

    pub fn hp(&self) -> i32 {
        self.hp
    }
    pub fn reset(&mut self) {
        self.position = vec2(PLAYER_SPAWN_X, PLAYER_SPAWN_Y);
        self.hp = PLAYER_MAX_HP;
        self.invincible_timer = 0.0;
        self.state = PlayerState::Idle;
        self.facing_left = false;
        self.attack_timer = 0.0;
        self.hurt_timer = 0.0;
    }

    pub fn is_invincible(&self) -> bool {
        self.invincible_timer > 0.0
    }
    pub fn draw(&self) {
        match self.state {
            PlayerState::Idle => self
                .idle_animation
                .draw(self.position, self.facing_left, 1.0),
            PlayerState::Run => self
                .run_animation
                .draw(self.position, self.facing_left, 1.0),
            PlayerState::Attack => self.attack_animations[self.current_attack].draw(
                self.position,
                self.facing_left,
                1.0,
            ),
            PlayerState::Hurt => self
                .hurt_animation
                .draw(self.position, self.facing_left, 1.0),
        }
    }
    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, PLAYER_SIZE, PLAYER_SIZE)
    }
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
    pub fn next_position(&self) -> Vec2 {
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

        self.position + direction * self.speed * dt
    }
}
