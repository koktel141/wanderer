mod constants;
mod game;
mod player;
mod camera;
mod animation;
use game::Game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Wanderer".to_string(),
        window_width: constants::SCREEN_WIDTH as i32,
        window_height: constants::SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(BLACK);

        game.update();
        game.draw();

        next_frame().await;
    }
}