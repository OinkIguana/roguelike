use std::cmp::min;
use rand::{thread_rng};
use rand::distributions::{IndependentSample,Range,Normal};
use super::tile::{Tile,TileType};

const MIN_ROOMS: u32 = 5;
const MAX_ROOMS: u32 = 30;

struct Rectangle {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}
impl Rectangle {
    fn collides(&self, rect: &Rectangle) -> bool {
        rect.x + rect.w >= self.x && rect.x < self.x + self.w &&
        rect.y + rect.h >= self.y && rect.y < self.y + self.h
    }
}

pub fn generate_map(complexity: u32, width: usize, height: usize) -> Vec<Tile> {
    let mut rng = thread_rng();

    let rm_count = min(MIN_ROOMS + complexity / 3, MAX_ROOMS);
    let mut merges_available = rm_count / 2;

    let mut tiles = vec![Tile::new(TileType::Empty); width * height];
    let x_range = Range::new(0, width);
    let y_range = Range::new(0, height);
    let w_range = Normal::new(16., 1.);
    let h_range = Normal::new(6., 1.);
    let coll_range = Range::new(0, 10);

    let mut rooms: Vec<Rectangle> = vec![];

    for _ in 0..rm_count {
        loop {
            let room = Rectangle{
                x: x_range.ind_sample(&mut rng),
                y: y_range.ind_sample(&mut rng),
                w: w_range.ind_sample(&mut rng) as usize + 2,
                h: h_range.ind_sample(&mut rng) as usize + 2,
            };

            if room.x + room.w >= width || room.y + room.h > height { continue; }

            let collides = rooms.iter().any(|ref rm| rm.collides(&room));
            if collides {
                if merges_available == 0 || coll_range.ind_sample(&mut rng) != 0 { continue; }
                merges_available -= 1;
            }
            rooms.push(room);
            break;
        }
    }

    for room in rooms {
        for x in room.x..room.x + room.w {
            for y in room.y..room.y + room.h {
                let index = y * width + x;
                tiles[index].set_kind(TileType::Floor);
            }
        }
    }

    tiles
}
