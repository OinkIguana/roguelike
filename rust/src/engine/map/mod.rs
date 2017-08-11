mod tile;

pub use self::tile::Tile;

use std::mem::replace;
use self::tile::TileType;
use super::actors::Player;
use inputter::{Action,Direction};

/// A Map contains tiles in a grid, which make up the whole dungeon
pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    /// Creates a new map with the provided dimensions
    pub fn new(width: usize, height: usize) -> Map {
        let mut tiles: Vec<Tile> = vec![
            TileType::Empty, TileType::Empty, TileType::Empty, TileType::Hall, TileType::Empty, TileType::Empty, TileType::Empty,
            TileType::Empty, TileType::Wall, TileType::Wall, TileType::Door, TileType::Wall, TileType::Wall, TileType::Empty,
            TileType::Empty, TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall, TileType::Empty,
            TileType::Empty, TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall, TileType::Empty,
            TileType::Empty, TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall, TileType::Empty,
            TileType::Empty, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Empty,
            TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty,
        ].iter().map(|kind| Tile::new(kind.clone())).collect();
        tiles[24].fill(Box::new(Player{}));
        Map{ tiles, width, height }
    }

    pub fn process(&self, action: Action) -> Vec<Action> {
        self.tiles.iter().map(|tile| tile.process(action.clone())).collect()
    }

    pub fn react(mut self, action: Action, tile_index: usize) -> Map {
        match action {
            Action::Move(dir) => {
                for neighbour in self.get_neighbouring_tile_index(tile_index, dir) {
                    let (a, b) = self.tiles[tile_index].clone().move_to(self.tiles[neighbour].clone());
                    replace(&mut self.tiles[tile_index], a);
                    replace(&mut self.tiles[neighbour], b);
                }
                self
            }
            _ => self
        }
    }

    pub fn get_neighbouring_tile_index(&self, tile_index: usize, direction: Direction) -> Option<usize> {
        match direction {
            Direction::N if tile_index >= self.width =>
                Some(tile_index - self.width),
            Direction::S if tile_index < self.tiles.len() - self.width =>
                Some(tile_index + self.width),
            Direction::W if tile_index % self.width > 0 =>
                Some(tile_index - 1),
            Direction::E if tile_index % self.width < self.width - 1 =>
                Some(tile_index + 1),
            _ => None
        }
    }
}
