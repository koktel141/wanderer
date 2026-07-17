use crate::constants::*;
use crate::player::Player;
use macroquad::prelude::*;

pub struct Wolf {
    pub position: Vec2,
    speed: f32,
    attack_cooldown: f32,
}

impl Wolf {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: vec2(x, y),
            speed: WOLF_SPEED,
            attack_cooldown: 0.0,
        }
    }

    pub fn update(&mut self, player: &mut Player) {
        let dt = get_frame_time();

        if self.attack_cooldown > 0.0 {
            self.attack_cooldown -= dt;
        }

        let direction = player.position - self.position;
        let distance = direction.length();

        if distance > WOLF_ATTACK_RANGE {
            self.position += direction.normalize() * self.speed * dt;
        } else if self.attack_cooldown <= 0.0 {
            player.take_damage(WOLF_ATTACK_DAMAGE);
            self.attack_cooldown = WOLF_ATTACK_COOLDOWN;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x + 16.0, self.position.y + 16.0, 16.0, RED);

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
}
