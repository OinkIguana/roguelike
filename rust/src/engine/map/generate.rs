use std::cmp::min;
use rand::{thread_rng,Rng};
use rand::distributions::{IndependentSample,Range,Normal};
use engine::Direction;
use super::Map;
use super::tile::{Tile,TileType};

const MIN_ROOMS: u8 = 5;
const MAX_ROOMS: u8 = 30;

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
    let room_count = min(MIN_ROOMS + (complexity / 3) as u8, MAX_ROOMS);

    let rooms = generate_tiles(room_count, width, height);
    let graph = graph_tiles(&rooms, width, height);
    let halls = generate_halls(graph, width, height);
    let rooms_and_halls = add_halls(rooms, halls);
    let tiles = add_walls(rooms_and_halls, width, height);

    tiles
}

fn generate_tiles(room_count: u8, width: usize, height: usize) -> Vec<Tile>{
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
        if tile.kind == TileType::Floor && graph[index] == 0 {
            graph = flood(graph, room_index, index, width, height, tiles, &|tile| tile.kind == TileType::Floor);
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

fn generate_halls(graph: Vec<u8>, width: usize, height: usize) -> Vec<bool> {
    let room_count = *graph.iter().max().unwrap();
    let row = |i| i / width;
    let col = |i| i % width;
    let ind = |x, y| y as usize * width + x as usize;

    let mut connected: Vec<u8> = vec![1];
    let mut rng = thread_rng();
    let r_range = Range::new(1, room_count + 1);
    let mut halls = vec![false; width * height];

    while connected.len() as u8 != room_count {
        let from_r = r_range.ind_sample(&mut rng);
        if connected.contains(&from_r) { continue; }
        let to_r = *rng.choose(&connected).expect("generate_halls: connected should not be empty");
        let from_t = find_tile_in_room(&graph, from_r);
        let to_t = find_tile_in_room(&graph, to_r);
        let mut x1 = col(from_t) as i32;
        let mut y1 = row(from_t) as i32;
        let x2 = col(to_t) as i32;
        let y2 = row(to_t) as i32;
        let mut dir: bool = rng.gen();
        while x1 != x2 || y1 != y2 {
            halls[ind(x1, y1)] = true;
            if dir {
                x1 += (x2 - x1).signum();
                dir = x1 != x2;
            } else {
                y1 += (y2 - y1).signum();
                dir = y1 == y2;
            }
        }
        connected.push(from_r);
    }
    halls
}

fn find_tile_in_room(graph: &Vec<u8>, target: u8) -> usize {
    let mut rng = thread_rng();
    let mut options: Vec<usize> = Vec::new();
    for (index, room) in graph.iter().enumerate() {
        if *room == target { options.push(index); }
    }
    *rng.choose(&options).expect(&format!("find_tile_in_room: room {} should have some cells", target))
}

fn add_halls(rooms: Vec<Tile>, halls: Vec<bool>) -> Vec<Tile> {
    rooms
        .iter()
        .to_owned()
        .zip(halls.iter().map(|b| *b))
        .map(|(tile, hall)| if hall && tile.kind == TileType::Empty { Tile::new(TileType::Hall) } else { tile.clone() })
        .collect()
}

fn add_walls(tiles: Vec<Tile>, width: usize, height: usize) -> Vec<Tile> {
    tiles
}
