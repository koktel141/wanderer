use crate::player::Player;

pub struct Game {
    player: Player,
}

impl Game {
    pub async fn new() -> Self {
        Self {
            player: Player::new(),
        }
    }

    pub fn update(&mut self) {
        self.player.update();
    }

    pub fn draw(&self) {
        self.player.draw();
    }
}