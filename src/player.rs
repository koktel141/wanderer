use crate::animation::Animation;
use crate::constants::*;
use macroquad::prelude::*;

#[derive(PartialEq)]
enum PlayerState {
    Idle,
    Run,
}
pub struct Player {
    pub position: Vec2,
    speed: f32,

    idle_animation: Animation,
    run_animation: Animation,

    state: PlayerState,
    facing_left: bool,
}
impl Player {
    pub async fn new() -> Self {
        Self {
            position: vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            facing_left: false,

            speed: PLAYER_SPEED,

            idle_animation: Animation::new("assets/player/Idle (32x32).png", 32.0, 32.0, 11, 0.10)
                .await,

            run_animation: Animation::new("assets/player/Run (32x32).png", 32.0, 32.0, 12, 0.08)
                .await,

            state: PlayerState::Idle,
        }
    }

pub fn update(&mut self, world: &crate::world::World) {
    let dt = get_frame_time();
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

    // حرکت روی محور X، فقط اگه برخورد نداشت اعمالش کن
    let mut next_x = self.position;
    next_x.x += velocity.x;
    let rect_x = Rect::new(next_x.x, self.position.y, PLAYER_SIZE, PLAYER_SIZE);
    if !world.check_collision(rect_x) {
        self.position.x = next_x.x;
    }

    // حرکت روی محور Y، جدا از X
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
    pub fn draw(&self) {
        match self.state {
            PlayerState::Idle => {
                self.idle_animation.draw(self.position, self.facing_left);
            }

            PlayerState::Run => {
                self.run_animation.draw(self.position, self.facing_left);
            }
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
