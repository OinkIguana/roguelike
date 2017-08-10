mod tile;
mod room;

pub use self::tile::Tile;
pub use self::room::Room;

/// A Map contains tiles in a grid, which make up the whole dungeon
pub struct Map<'a> {
    pub tiles: Vec<Tile>,
    pub rooms: Vec<Room<'a>>,
    pub width: usize,
    pub height: usize,
}

impl<'a> Map<'a> {
    /// Creates a new map with the provided dimensions
    pub fn new(width: usize, height: usize) -> Map<'a> {
        let tiles = vec![];
        let rooms = vec![];
        Map{ tiles, width, height, rooms }
    }
}
