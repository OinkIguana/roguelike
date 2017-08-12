use std::cmp::min;
use rand::{thread_rng};
use rand::distributions::{IndependentSample,Range,Normal};
use engine::Direction;
use super::Map;
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
    let room_count = min(MIN_ROOMS + complexity / 3, MAX_ROOMS);

    let tiles = generate_tiles(room_count, width, height);
    let graph = graph_tiles(&tiles, width, height);


    tiles
}

fn generate_tiles(room_count: u32, width: usize, height: usize) -> Vec<Tile>{
    let mut rng = thread_rng();
    let mut merges_available = room_count / 2;

    let mut tiles = vec![Tile::new(TileType::Empty); width * height];
    let x_range = Range::new(0, width);
    let y_range = Range::new(0, height);
    let w_range = Normal::new(16., 1.);
    let h_range = Normal::new(6., 1.);
    let coll_range = Range::new(0, 10);

    let mut rooms: Vec<Rectangle> = vec![];

    for _ in 0..room_count {
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
                tiles[index].kind = TileType::Floor;
            }
        }
    }

    tiles
}

fn graph_tiles(tiles: &Vec<Tile>, width: usize, height: usize) -> Vec<u8> {
    let mut graph = vec![0; tiles.len()];
    let mut room_index = 1;
    for (index, tile) in tiles.iter().enumerate() {
        if tile.kind == TileType::Floor && graph[index] > 0 {
            graph = flood(graph, room_index, index, width, height, tiles, &|tile: &Tile| tile.kind == TileType::Floor);
            room_index += 1;
        }
    }
    graph
}

fn flood<T, F: Fn(&T) -> bool>(mut graph: Vec<u8>, room_index: u8, tile_index: usize, width: usize, height: usize, tiles: &Vec<T>, pred: &F) -> Vec<u8> {
    if graph[tile_index] > 0 { return graph; }
    if pred(&tiles[tile_index]) {
        graph[tile_index] = room_index;
        let neighbours = vec![
        Map::neighbouring_tile_index(tile_index, width, height, Direction::N),
        Map::neighbouring_tile_index(tile_index, width, height, Direction::S),
        Map::neighbouring_tile_index(tile_index, width, height, Direction::W),
        Map::neighbouring_tile_index(tile_index, width, height, Direction::E),
        ];
        for neighbour in neighbours.iter().flat_map(|n| n.iter()).map(|i| *i) {
            graph = flood(graph, room_index, neighbour, width, height, tiles, pred)
        }
    }
    graph
}
