pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;

// ================= World =================

pub const TILE_SIZE: f32 = 32.0;

pub const MAP_WIDTH: usize = 100;
pub const MAP_HEIGHT: usize = 100;

pub const WORLD_WIDTH: f32 = MAP_WIDTH as f32 * TILE_SIZE;
pub const WORLD_HEIGHT: f32 = MAP_HEIGHT as f32 * TILE_SIZE;

// ================= Player =================

pub const PLAYER_SIZE: f32 = 32.0;
pub const PLAYER_SPEED: f32 = 220.0;
//==================Wolf=====================
// ================= Enemy =================
pub const WOLF_SPEED: f32 = 90.0;
// ================= Combat =================
pub const PLAYER_MAX_HP: i32 = 100;
pub const WOLF_ATTACK_DAMAGE: i32 = 10;
pub const WOLF_ATTACK_RANGE: f32 = 30.0;
pub const WOLF_ATTACK_COOLDOWN: f32 = 1.0;
pub const PLAYER_INVINCIBILITY_TIME: f32 = 0.8;