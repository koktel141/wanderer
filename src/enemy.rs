use crate::animation::Animation;
use crate::constants::*;
use crate::player::Player;
use crate::world::World;
use macroquad::prelude::*;

pub struct Wolf {
    pub position: Vec2,
    speed: f32,
    attack_cooldown: f32,
    animation: Animation,
    facing_left: bool,
    hp: i32,
}

impl Wolf {
    pub async fn new(x: f32, y: f32) -> Self {
        Self {
            position: vec2(x, y),
            speed: WOLF_SPEED,
            attack_cooldown: 0.0,
            animation: Animation::new("assets/enemy/wolf_walk.png", 32.0, 32.0, 4, 0.15).await,
            facing_left: false,
            hp: WOLF_HP,
        }
    }

    pub fn update(&mut self, player: &mut Player, world: &World, aggressive: bool) {
        let dt = get_frame_time();

        if self.attack_cooldown > 0.0 {
            self.attack_cooldown -= dt;
        }

        if !aggressive {
            return;
        }

        let direction = player.position - self.position;
        let distance = direction.length();

        if direction.x < 0.0 {
            self.facing_left = true;
        } else if direction.x > 0.0 {
            self.facing_left = false;
        }

        if distance > WOLF_ATTACK_RANGE {
            let velocity = direction.normalize() * self.speed * dt;

            let mut next_x = self.position;
            next_x.x += velocity.x;
            let rect_x = Rect::new(
                next_x.x,
                self.position.y,
                WOLF_HITBOX_SIZE,
                WOLF_HITBOX_SIZE,
            );
            if !world.check_collision(rect_x) {
                self.position.x = next_x.x;
            }

            let mut next_y = self.position;
            next_y.y += velocity.y;
            let rect_y = Rect::new(
                self.position.x,
                next_y.y,
                WOLF_HITBOX_SIZE,
                WOLF_HITBOX_SIZE,
            );
            if !world.check_collision(rect_y) {
                self.position.y = next_y.y;
            }

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

    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, PLAYER_SIZE, PLAYER_SIZE)
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    pub fn reset(&mut self, x: f32, y: f32) {
        self.position = vec2(x, y);
        self.attack_cooldown = 0.0;
        self.hp = WOLF_HP;
    }
}
