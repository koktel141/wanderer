mod animation;
mod camera;
mod constants;
mod enemy;
mod game;
mod map;
mod npc;
mod player;
mod quest;
mod state;
mod world;

use game::Game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    let icon_small: [u8; 16 * 16 * 4] = include_bytes!("../assets/icon/icon_16.rgba")[..]
        .try_into()
        .unwrap();
    let icon_medium: [u8; 32 * 32 * 4] = include_bytes!("../assets/icon/icon_32.rgba")[..]
        .try_into()
        .unwrap();
    let icon_big: [u8; 64 * 64 * 4] = include_bytes!("../assets/icon/icon_64.rgba")[..]
        .try_into()
        .unwrap();

    Conf {
        window_title: "Wanderer".to_string(),
        window_width: constants::SCREEN_WIDTH as i32,
        window_height: constants::SCREEN_HEIGHT as i32,
        window_resizable: false,
        icon: Some(miniquad::conf::Icon {
            small: icon_small,
            medium: icon_medium,
            big: icon_big,
        }),
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
