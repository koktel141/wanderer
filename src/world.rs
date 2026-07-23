use crate::constants::*;
use macroquad::prelude::*;

pub struct World {
    background: Texture2D,
    obstacles: Vec<Rect>,
}

impl World {
    pub async fn new() -> Self {
        let background = load_texture("assets/maps/map_background.png")
            .await
            .unwrap();
        background.set_filter(FilterMode::Nearest);

        let mut obstacles = Vec::new();

        let border = 110.0;
        obstacles.push(Rect::new(0.0, 0.0, WORLD_WIDTH, border));
        obstacles.push(Rect::new(0.0, WORLD_HEIGHT - border, WORLD_WIDTH, border));
        obstacles.push(Rect::new(0.0, 0.0, border, WORLD_HEIGHT));
        obstacles.push(Rect::new(WORLD_WIDTH - border, 0.0, border, WORLD_HEIGHT));

        let river_points = [
            (743.0, 219.0),
            (713.0, 280.0),
            (685.0, 342.0),
            (636.0, 407.0),
            (589.0, 470.0),
            (562.0, 538.0),
            (537.0, 701.0),
            (515.0, 757.0),
            (504.0, 803.0),
            (490.0, 856.0),
            (484.0, 911.0),
            (462.0, 969.0),
            //(421.0, 1010.0),
        ];
        let river_tile = 90.0;
        for &(px, py) in river_points.iter() {
            obstacles.push(Rect::new(
                px - river_tile / 1.5,
                py - river_tile / 1.5,
                river_tile,
                river_tile,
            ));
        }
        obstacles.push(Rect::new(960.0, 230.0, 220.0, 190.0)); //farm
        obstacles.push(Rect::new(1030.0, 560.0, 100.0, 60.0)); //
        obstacles.push(Rect::new(730.0, 760.0, 220.0, 180.0)); //house

        obstacles.push(Rect::new(1044.0, 695.0, 51.0, 26.0));
        obstacles.push(Rect::new(560.0, 130.0, 140.0, 180.0));

        let ruin_x = 100.0;
        let ruin_y = 220.0;
        let ruin_w = 220.0;
        let ruin_h = 200.0;
        let wall_thickness = 20.0;
        let door_width = 40.0;

        obstacles.push(Rect::new(ruin_x, ruin_y, ruin_w, wall_thickness)); 
        obstacles.push(Rect::new(
            ruin_x,
            ruin_y + ruin_h - wall_thickness,
            ruin_w,
            wall_thickness,
        ));
        obstacles.push(Rect::new(ruin_x, ruin_y, wall_thickness, ruin_h));

        let door_start = ruin_y + ruin_h / 2.0 - door_width / 2.0;
        obstacles.push(Rect::new(
            ruin_x + ruin_w - wall_thickness,
            ruin_y,
            wall_thickness,
            door_start - ruin_y,
        ));
        obstacles.push(Rect::new(
            ruin_x + ruin_w - wall_thickness,
            door_start + door_width,
            wall_thickness,
            ruin_y + ruin_h - (door_start + door_width),
        ));

        Self {
            background,
            obstacles,
        }
    }

    pub fn ruin_interior_spawns() -> [(f32, f32); 5] {
        [
            (160.0, 280.0),
            (240.0, 350.0),
            (150.0, 330.0),
            (220.0, 260.0),
            (190.0, 370.0),
        ]
    }
    pub fn draw(&self) {
        draw_texture_ex(
            &self.background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.background.width(), self.background.height())),
                ..Default::default()
            },
        );
    }

    pub fn check_collision(&self, rect: Rect) -> bool {
        if rect.x < 0.0 || rect.y < 0.0 {
            return true;
        }
        if rect.x + rect.w > self.background.width() || rect.y + rect.h > self.background.height() {
            return true;
        }

        self.obstacles.iter().any(|o| o.overlaps(&rect))
    }

    pub fn draw_colliders(&self) {
        for o in &self.obstacles {
            draw_rectangle_lines(o.x, o.y, o.w, o.h, 2.0, RED);
        }
    }
}
