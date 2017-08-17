mod tile;
mod generator;
mod populator;

use std::mem::replace;
use rand::{thread_rng,Rng};
use super::{Action,Actor};
use engine::Direction;
pub use self::generator::Generator;
pub use self::populator::Populator;
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
    pub fn new<T: Generator>(complexity: u32, generator: &T) -> Map {
        let height: usize = (MIN_HEIGHT + GROWTH_FACTOR * complexity as f32).round() as usize;
        let width: usize = (1.618 * height as f32 * 2.0).round() as usize;
        generator.generate(complexity, width, height)
    }

    /// Given a populator, poulates the map
    pub fn populate<T: Populator>(self, populator: &T) -> Map {
        populator.populate(self)
    }

    /// Has every tile process an Action to produce the actual Action that should be taken
    pub fn process(&self, action: Action) -> Vec<Action> {
        self.tiles.iter().map(|tile| tile.process(action.clone())).collect()
    }

    /// Has each tile react to the Action that it produced from process, actually performing the Action
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

    /// Gets the index of the neighbouring tile, using the dimensions of the current map
    pub fn get_neighbouring_tile_index(&self, tile_index: usize, direction: Direction) -> Option<usize> {
        Map::neighbouring_tile_index(tile_index, self.width, self.height, direction)
    }

    /// Calculates the index of a neighbouring tile based on the dimensions of a grid
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

    pub fn fill_tile<T: Actor + 'static>(mut self, index: usize, contents: T) -> Map {
        self.tiles[index].fill(Box::new(contents));
        self
    }

    pub fn get_random_open_tile(&self) -> Option<usize> {
        self.find_random_tile(|ref t| t.can_hold_contents() && t.contents().is_none())
    }

    /// Finds the index of a random tile that matches a predicate
    pub fn find_random_tile<F: Fn(&Tile) -> bool>(&self, pred: F) -> Option<usize> {
        let options: Vec<usize> = self.tiles.iter().enumerate().filter(|&(_, tile)| pred(tile)).map(|(i, _)| i).collect();
        thread_rng().choose(&options).map(|i| *i)
    }
}
