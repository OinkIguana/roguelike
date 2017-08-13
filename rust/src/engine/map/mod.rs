mod tile;
mod generate;

use std::mem::replace;
use inputter::Action;
use engine::Direction;
use self::generate::generate_map;

pub use self::tile::{Tile,TileType};

/// A Map contains tiles in a grid, which make up the whole dungeon
pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

const GROWTH_FACTOR: f32 = 1.5;
const MIN_HEIGHT: f32 = 20.0;

impl Map {
    /// Creates a new map with the provided dimensions
    pub fn new(complexity: u32) -> Map {
        let height: usize = (MIN_HEIGHT + GROWTH_FACTOR * complexity as f32).round() as usize;
        let width: usize = (1.618 * height as f32 * 2.0).round() as usize;
        let tiles = generate_map(complexity, width, height);
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
        Map::neighbouring_tile_index(tile_index, self.width, self.height, direction)
    }

    pub fn neighbouring_tile_index(tile_index: usize, width: usize, height: usize, direction: Direction) -> Option<usize> {
        match direction {
            Direction::N if tile_index >= width =>
                Some(tile_index - width),
            Direction::S if tile_index < width * height - width =>
                Some(tile_index + width),
            Direction::W if tile_index % width > 0 =>
                Some(tile_index - 1),
            Direction::E if tile_index % width < width - 1 =>
                Some(tile_index + 1),
            Direction::NE =>
                Map::neighbouring_tile_index(tile_index, width, height, Direction::N)
                .and_then(|n| Map::neighbouring_tile_index(n, width, height, Direction::E)),
            Direction::NW =>
                Map::neighbouring_tile_index(tile_index, width, height, Direction::N)
                .and_then(|n| Map::neighbouring_tile_index(n, width, height, Direction::W)),
            Direction::SE =>
                Map::neighbouring_tile_index(tile_index, width, height, Direction::S)
                .and_then(|n| Map::neighbouring_tile_index(n, width, height, Direction::E)),
            Direction::SW =>
                Map::neighbouring_tile_index(tile_index, width, height, Direction::S)
                .and_then(|n| Map::neighbouring_tile_index(n, width, height, Direction::W)),
            _ => None
        }
    }
}
