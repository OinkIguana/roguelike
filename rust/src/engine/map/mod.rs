mod tile;
mod room;

pub use self::tile::Tile;
use self::tile::TileType;
pub use self::room::Room;

/// A Map contains tiles in a grid, which make up the whole dungeon
pub struct Map<'a> {
    pub tiles: Vec<Tile<'a>>,
    pub rooms: Vec<Room<'a>>,
    pub width: usize,
    pub height: usize,
}

impl<'a> Map<'a> {
    /// Creates a new map with the provided dimensions
    pub fn new(width: usize, height: usize) -> Map<'a> {
        let tiles = vec![
            TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty,
            TileType::Empty, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Empty,
            TileType::Empty, TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall, TileType::Empty,
            TileType::Empty, TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall, TileType::Empty,
            TileType::Empty, TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall, TileType::Empty,
            TileType::Empty, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Empty,
            TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty,
        ].iter().map(|kind| Tile::new(kind.clone())).collect();
        let rooms = vec![];
        Map{ tiles, width: 7, height: 7, rooms }
    }
}
