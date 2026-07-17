use crate::constants::{MAP_WIDTH, MAP_HEIGHT, SCREEN_WIDTH, SCREEN_HEIGHT, TILE_SIZE};

fn hash(x: i32, y: i32, seed: u32) -> f32 {
    let ux = x as u32;
    let uy = y as u32;

    let mut h = ux
        .wrapping_mul(374761393)
        .wrapping_add(uy.wrapping_mul(668265263))
        .wrapping_add(seed);

    h ^= h >> 13; // شیفت منطقی چون h از نوع u32 هست
    h = h.wrapping_mul(1274126177);
    h ^= h >> 16;

    (h as f32 / u32::MAX as f32) * 2.0 - 1.0
}

fn smooth_noise(x: f32, y: f32, seed: u32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let sx = x - x0 as f32;
    let sy = y - y0 as f32;

    let n00 = hash(x0, y0, seed);
    let n10 = hash(x0 + 1, y0, seed);
    let n01 = hash(x0, y0 + 1, seed);
    let n11 = hash(x0 + 1, y0 + 1, seed);

    let ix0 = n00 + sx * (n10 - n00);
    let ix1 = n01 + sx * (n11 - n01);
    ix0 + sy * (ix1 - ix0)
}

fn octave_noise(x: f32, y: f32, seed: u32, octaves: u32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_value = 0.0;

    for i in 0..octaves {
        value += smooth_noise(x * frequency, y * frequency, seed + i) * amplitude;
        max_value += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    value / max_value
}

pub fn create_map() -> Vec<Vec<u8>> {
    let mut map = vec![vec![0u8; MAP_WIDTH]; MAP_HEIGHT];

    let water_seed = 10;
    let dirt_seed = 200;

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let nx = x as f32 / 18.0;
            let ny = y as f32 / 18.0;

            let water_value = octave_noise(nx, ny, water_seed, 4);
            let dirt_value = octave_noise(nx * 1.6, ny * 1.6, dirt_seed, 3);

            if water_value > 0.28 {
                map[y][x] = 1;
            } else if dirt_value > 0.35 {
                map[y][x] = 2;
            } else {
                map[y][x] = 0;
            }
        }
    }

    let spawn_x = (SCREEN_WIDTH / 2.0 / TILE_SIZE) as i32;
    let spawn_y = (SCREEN_HEIGHT / 2.0 / TILE_SIZE) as i32;
    let clear_radius = 5;

    for dy in -clear_radius..=clear_radius {
        for dx in -clear_radius..=clear_radius {
            let x = spawn_x + dx;
            let y = spawn_y + dy;
            if x >= 0 && y >= 0 && (x as usize) < MAP_WIDTH && (y as usize) < MAP_HEIGHT {
                map[y as usize][x as usize] = 0;
            }
        }
    }

    map
}