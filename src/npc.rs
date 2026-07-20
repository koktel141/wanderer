use crate::constants::*;
use macroquad::prelude::*;

pub struct Npc {
    pub position: Vec2,
}

impl Npc {
    pub fn new(x: f32, y: f32) -> Self {
        Self { position: vec2(x, y) }
    }

    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, PLAYER_SIZE, PLAYER_SIZE, BLUE);
        draw_rectangle_lines(self.position.x, self.position.y, PLAYER_SIZE, PLAYER_SIZE, 2.0, WHITE);
    }

    pub fn is_player_nearby(&self, player_pos: Vec2) -> bool {
        self.position.distance(player_pos) <= NPC_INTERACT_RANGE
    }
}