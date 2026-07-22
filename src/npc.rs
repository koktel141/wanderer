use crate::constants::*;
use macroquad::prelude::*;

pub struct Npc {
    pub position: Vec2,
    dialogue: Vec<&'static str>,
    texture: Texture2D,
}

impl Npc {
    pub async fn new(x: f32, y: f32) -> Self {
        let texture = load_texture("assets/npc/npc.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        Self {
            position: vec2(x, y),
            dialogue: vec![
                "Traveler! These woods have gotten dangerous lately.",
                "Wolves have been attacking anyone who wanders too far.",
                "If you can defeat a few of them, I'd be grateful.",
                "Safe travels, and watch your back.",
            ],
            texture,
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(PLAYER_SIZE, PLAYER_SIZE)),
                ..Default::default()
            },
        );
    }

    pub fn is_player_nearby(&self, player_pos: Vec2) -> bool {
        self.position.distance(player_pos) <= NPC_INTERACT_RANGE
    }

    pub fn line(&self, index: usize) -> Option<&str> {
        self.dialogue.get(index).copied()
    }

    pub fn line_count(&self) -> usize {
        self.dialogue.len()
    }
}
