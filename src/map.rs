pub const MAP_WIDTH: usize = 100;
pub const MAP_HEIGHT: usize = 100;

pub fn create_map() -> Vec<Vec<u8>> {
    let mut map = vec![vec![0; MAP_WIDTH]; MAP_HEIGHT];

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {

            if y > 60 {
                map[y][x] = 1;
            }

            if x > 70 && y > 20 {
                map[y][x] = 2;
            }
        }
    }

    map
}