use macroquad::prelude::*;

pub struct Animation {
    texture: Texture2D,
    frame_width: f32,
    frame_height: f32,
    frame_count: usize,
    current_frame: usize,
    timer: f32,
    frame_time: f32,
}

impl Animation {
    pub async fn new(
        path: &str,
        frame_width: f32,
        frame_height: f32,
        frame_count: usize,
        frame_time: f32,
    ) -> Self {
        let texture = load_texture(path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        Self {
            texture,
            frame_width,
            frame_height,
            frame_count,
            current_frame: 0,
            timer: 0.0,
            frame_time,
        }
    }

    pub fn update(&mut self) {
        self.timer += get_frame_time();

        if self.timer >= self.frame_time {
            self.timer = 0.0;
            self.current_frame = (self.current_frame + 1) % self.frame_count;
        }
    }

pub fn draw(&self, position: Vec2, flip_x: bool, scale: f32) {
    let width = self.frame_width * scale;
    let height = self.frame_height * scale;

    draw_texture_ex(
        &self.texture,
        position.x,
        position.y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect {
                x: self.current_frame as f32 * self.frame_width,
                y: 0.0,
                w: self.frame_width,
                h: self.frame_height,
            }),
            dest_size: Some(vec2(width, height)),
            flip_x,
            ..Default::default()
        },
    );
}
}
