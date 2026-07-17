use crate::animation::Animation;
use crate::constants::*;
use crate::player::Player;
use macroquad::prelude::*;

pub struct Wolf {
    pub position: Vec2,
    speed: f32,
    attack_cooldown: f32,
    animation: Animation,
    facing_left: bool,
}

impl Wolf {
    pub async fn new(x: f32, y: f32) -> Self {
        Self {
            position: vec2(x, y),
            speed: WOLF_SPEED,
            attack_cooldown: 0.0,
            animation: Animation::new("assets/enemy/wolf_walk.png", 32.0, 32.0, 4, 0.15).await,
            facing_left: false,
        }
    }

    pub fn update(&mut self, player: &mut Player) {
        let dt = get_frame_time();

        if self.attack_cooldown > 0.0 {
            self.attack_cooldown -= dt;
        }

        let direction = player.position - self.position;
        let distance = direction.length();

        if direction.x < 0.0 {
            self.facing_left = true;
        } else if direction.x > 0.0 {
            self.facing_left = false;
        }

        if distance > WOLF_ATTACK_RANGE {
            self.position += direction.normalize() * self.speed * dt;
            self.animation.update();
        } else if self.attack_cooldown <= 0.0 {
            player.take_damage(WOLF_ATTACK_DAMAGE);
            self.attack_cooldown = WOLF_ATTACK_COOLDOWN;
        }
    }

    pub fn draw(&self) {
        self.animation
            .draw(self.position, self.facing_left, WOLF_VISUAL_SCALE);

        if self.attack_cooldown > WOLF_ATTACK_COOLDOWN - 0.15 {
            draw_circle_lines(
                self.position.x + 16.0,
                self.position.y + 16.0,
                20.0,
                2.0,
                YELLOW,
            );
        }
    }

    pub fn reset(&mut self, x: f32, y: f32) {
        self.position = vec2(x, y);
        self.attack_cooldown = 0.0;
    }
}
