use macroquad::prelude::*;

pub fn create_camera(target: Vec2) -> Camera2D {
    Camera2D {
        target,

        zoom: vec2(5.0 / screen_width(), 5.0 / screen_height()),

        ..Default::default()
    }
}
