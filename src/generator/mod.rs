use std::cmp::min;
use rand::{thread_rng,Rng};
use rand::distributions::{IndependentSample,Range,Normal};
use engine::{Map,Tile,TileType,Direction,Generator};


pub struct Standard;
impl Generator for Standard {
    /// Given a complexity and dimensions, an entire dungeon map is created
    ///
    /// 1.  Generate the set of tiles, some floors and some empty
    /// 2.  Make a graph of which tiles are connected in which rooms
    /// 3.  Connect the entire graph of rooms by hallways
    /// 4.  Merge the hallways with the floors
    /// 5.  Strip out all the unnecessary halls (double wide, dead end)
    /// 6.  Add wall tiles around the rooms, and doors where the hallways enter the rooms
    fn generate(&self, complexity: u32, width: usize, height: usize) -> Map {
        let room_count = min(MIN_ROOMS + (complexity / 3) as u8, MAX_ROOMS);
        let rooms = generate_tiles(room_count, width, height);
        let graph = graph_tiles(&rooms);
        let halls = generate_halls(graph);
        let rooms_and_halls = add_halls(rooms, halls);
        let trimmed = trim_halls(rooms_and_halls);
        let tiles = add_walls(trimmed);
        Map{ tiles: tiles.grid, width, height }
    }
}

const MIN_ROOMS: u8 = 5;
const MAX_ROOMS: u8 = 30;

struct Grid<T> {
    width: usize,
    height: usize,
    grid: Vec<T>,
}

struct Rectangle {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}
impl Rectangle {
    fn collides(&self, rect: &Rectangle) -> bool {
        rect.x + rect.w >= self.x && rect.x <= self.x + self.w &&
        rect.y + rect.h >= self.y && rect.y <= self.y + self.h
    }
}

/// The first step of room creation is to create the tiles.
///
/// Some rectangles are generated randomly, and placed on the map. Some collisions are allowed, but
/// only up to a maximum number of times so that the map always has some disconnnected rooms.
fn generate_tiles(room_count: u8, width: usize, height: usize) -> Grid<Tile> {
    let mut rng = thread_rng();
    let mut merges_available: usize = room_count as usize / 2;

    let mut tiles: Vec<Tile> = (0..width * height).into_iter().map(|i| Tile::new(TileType::Empty, i)).collect();
    let x_range = Range::new(1, width);
    let y_range = Range::new(1, height);
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

            if room.x + room.w >= width || room.y + room.h >= height { continue; }

            let collides = rooms.iter().map(|ref rm| rm.collides(&room)).filter(|b| *b).count();
            if collides > 0 {
                if merges_available < collides || coll_range.ind_sample(&mut rng) != 0 { continue; }
                merges_available -= collides;
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

    Grid{ grid: tiles, width, height }
}

/// Turns the set of floor/empty tiles into a graph of numbered rooms which will later be used to
/// ensure that the whole map is connected.
fn graph_tiles(&Grid{ grid: ref tiles, width, height }: &Grid<Tile>) -> Grid<u8> {
    Grid{
        width,
        height,
        grid: tiles.iter().enumerate()
        .fold((vec![0; tiles.len()], 1), |(graph, room_index), (index, tile)|
            if tile.kind == TileType::Floor && graph[index] == 0 {
                (flood(graph, room_index, index, width, height, tiles, &|tile| tile.kind == TileType::Floor), room_index + 1)
            } else {
                (graph, room_index)
            }
        ).0
    }
}

/// Applies a flood fill to a grid (represented as a vector with width and height), based on
/// another grid of the same dimensions and a predicate
fn flood<T, F: Fn(&T) -> bool>(mut graph: Vec<u8>, room_index: u8, tile_index: usize, width: usize, height: usize, tiles: &Vec<T>, pred: &F) -> Vec<u8> {
    if graph[tile_index] > 0 { return graph; }
    if pred(&tiles[tile_index]) {
        graph[tile_index] = room_index;
        let neighbours: Vec<Option<usize>> = Direction::cardinals()
            .into_iter()
            .map(|d| Map::neighbouring_tile_index(tile_index, width, height, d))
            .collect();
        for neighbour in neighbours.iter().flat_map(|n| n.iter()).map(|i| *i) {
            graph = flood(graph, room_index, neighbour, width, height, tiles, pred)
        }
    }
    graph
}

/// Connects the graph of tiles by hallways. Starting with room 1, hallways are generated between
/// a random tile in a connected room and a tile in a random disconnected room. In the end, a vec
/// of booleans is produced, where true represents a "hall" tile and false an "empty".
fn generate_halls(Grid{ grid: graph, width, height}: Grid<u8>) -> Grid<bool> {
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
        let mut dir: bool = rng.gen(); // dir ? horizontal : vertical
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
    Grid{ grid: halls, width, height }
}

/// Picks a random tile from the room with the given index, returning the index of the chosen tile
///
/// Requires: `graph.iter().any(|r| r == target) == true`
fn find_tile_in_room(graph: &Vec<u8>, target: u8) -> usize {
    let options: Vec<usize> =
        graph.iter().enumerate()
            .filter_map(|(index, room)| if *room == target { Some(index) } else { None })
            .collect();
    *thread_rng()
        .choose(&options)
        .expect(&format!("find_tile_in_room: room {} should have some cells", target))
}

/// Merges the list of hall flags with the actual tiles. Tiles which are empty but have a hall flag
/// set to true are turned into halls, while others are unaffected
fn add_halls(rooms: Grid<Tile>, halls: Grid<bool>) -> Grid<Tile> {
    Grid{ grid: rooms
        .grid
        .iter()
        .to_owned()
        .zip(halls.grid.iter().map(|b| *b))
        .enumerate()
        .map(|(i, (tile, hall))| if hall && tile.kind == TileType::Empty { Tile::new(TileType::Hall, i) } else { tile.clone() })
        .collect(), width: rooms.width, height: rooms.height }
}

/// Trims excess halls from the set of tiles. Excess halls are either fat or a dead end.
fn trim_halls(Grid{ grid: tiles, width, height}: Grid<Tile>) -> Grid<Tile> {
    let no_fats = (0..width * height)
        .fold(tiles, |mut tiles, index|
            if is_fat_hallway(&tiles, index, width, height) {
                tiles[index].kind = TileType::Empty;
                tiles
            } else {
                tiles
            });
    (0..width * height).fold(Grid{ grid: no_fats, width, height }, |tiles, index| remove_dead_end(tiles, index))
}

/// Determines whether a hallway is "fat". A fat hallway is one that can be removed without
/// creating a dead end.
///
/// Example: a hallway on the spot marked with O would be a "fat" hallway
/// ```
/// ###
///  O#
///   #
/// ```
fn is_fat_hallway(tiles: &Vec<Tile>, index: usize, width: usize, height: usize) -> bool {
    if tiles[index].kind != TileType::Hall { return false; }
    let neighbours: String =
        Direction::variants()
            .into_iter()
            .map(|d| Map::neighbouring_tile_index(index, width, height, d).map(|n| tiles[n].kind))
            .map(|o| match o {
                None | Some(TileType::Empty) => ' ',
                _                            => '.',
            }).collect();
    match &neighbours[..] {
        // TODO: 4 + corner, 3 + corner
        " ...    " | "   ...  " | "     ..." | "..     ." | "..... . " | ". ..... " | ". . ...." | "... . .." |
        "....    " | " ....   " | "  ....  " | "   .... " | "    ...." | ".    ..." | "..    .." | "...    ." |
        ".....   " | " .....  " | "  ..... " | "   ....." | ".   ...." | "..   ..." | "...   .." | "....   ." |
        "......  " | " ...... " | "  ......" | ".  ....." | "..  ...." | "...  ..." | "....  .." | " ..... ." |
        "....... " | " ......." | ". ......" | ".. ....." | "... ...." | ".... ..." | "..... .." | "...... ." |
        "........" => true,
        _ => false,
    }
}

/// Recursively removes dead ends. A dead end is a hallway which only has one adjacent walkable
/// space (hall/floor).
fn remove_dead_end(Grid{ grid: mut tiles, width, height }: Grid<Tile>, index: usize) -> Grid<Tile> {
    let neighbours: Vec<usize> = Direction::cardinals()
        .into_iter()
        .map(|d| Map::neighbouring_tile_index(index, width, height, d))
        .flat_map(|n| n)
        .filter(|n| tiles[*n].kind != TileType::Empty)
        .collect();
    if neighbours.len() > 1 { return Grid{ grid: tiles, width, height }; }
    tiles[index].kind = TileType::Empty;
    if neighbours.len() > 0 {
        remove_dead_end(Grid{ grid: tiles, width, height }, neighbours[0])
    } else {
        Grid{ grid: tiles, width, height }
    }
}

/// Adds a wall tile wherever a floor tile meets an empty tile, or a door where a hall tile meets
/// a floor tile.
fn add_walls(Grid{ grid: tiles, width, height }: Grid<Tile>) -> Grid<Tile> {
    // TODO: place walls inside the rooms to prevent halls along edges?
    Grid{ grid: (0..width * height)
        .map(|i| Direction::variants().into_iter()
                .map(|d| Map::neighbouring_tile_index(i, width, height, d))
                .flat_map(|o| o.map(|n| tiles[n].kind == TileType::Floor).and_then(|b| if b { Some(true) } else { None }))
                .count())
        .zip(tiles.iter())
        .enumerate()
        .map(|(i, (c, t))| match t.kind {
                TileType::Empty if c > 0    => Tile::new(TileType::Wall, i),
                TileType::Hall if c >= 2    => Tile::new(TileType::Door, i),
                _                           => t.clone(),
            })
        .collect(), width, height }
}
