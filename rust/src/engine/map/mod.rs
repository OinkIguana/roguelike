struct Actor; // TODO: figure out what this should be

/// A TileType determines the geography of each tile
#[allow(dead_code)]
enum TileType {
    Floor,
    Wall,
    Hall,
    Door,
    Empty,
}

/// A Tile represents one space in the dungeon. It can have one of a few types, and can
/// optionally hold one Actor
#[allow(dead_code)]
struct Tile {
    kind: TileType,
    contents: Option<Actor>,
}

/// A Map contains together tiles in a grid, which make up the whole dungeon
#[allow(dead_code)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
}

impl Map {
    /// Creates a new map with the provided dimensions
    pub fn new(width: i32, height: i32) -> Map {
        let tiles = Vec::new();
        Map{ tiles, width, height }
    }
}
