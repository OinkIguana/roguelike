mod tile;
mod tile_type;
mod generator;
mod populator;

use std::usize;
use std::cmp::min;
use std::collections::HashMap;
use std::cell::RefCell;
use rand::{thread_rng,Rng};
use super::{Action,Actor,Behavior,Direction};
pub use self::generator::Generator;
pub use self::populator::Populator;
pub use self::tile::Tile;
pub use self::tile_type::TileType;

/// A `Map` contains `Tile`s in a grid, which make up the whole dungeon
#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
    distances: RefCell<HashMap<usize, Vec<usize>>>,
}

const GROWTH_FACTOR: f32 = 1.5;
const MIN_HEIGHT: f32 = 20.0;

impl Map {
    fn split_coordinate(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    /// Instantiates a new `Map`
    pub fn new(tiles: Vec<Tile>, width: usize, height: usize) -> Self {
        Self{ tiles, width, height, distances: RefCell::new(HashMap::new()) }
    }

    /// Creates a new `Map` with the provided dimensions
    pub fn create<T: Generator>(complexity: u32, generator: &T) -> Self {
        let height: usize = (MIN_HEIGHT + GROWTH_FACTOR * complexity as f32).round() as usize;
        let width: usize = (1.618 * height as f32 * 2.0).round() as usize;
        generator.generate(complexity, width, height)
    }

    /// Has every `Tile` process an `Action` to produce the `Behavior` that should be performed on
    /// this `Tile`
    pub fn process(&self, action: Action) -> Vec<Box<Behavior>> {
        self.tiles.iter().map(|tile| tile.process(action.clone())).collect()
    }

    /// Gets the index of the neighbouring `Tile`, using the dimensions of the current `Map`
    pub fn get_neighbouring_tile_index(&self, tile_index: usize, direction: Direction) -> Option<usize> {
        Map::neighbouring_tile_index(tile_index, self.width, self.height, direction)
    }

    /// Calculates the index of a neighbouring `Tile` based on the dimensions of a `Map`
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

    /// Fills a specific `Tile` with the given contents
    pub fn fill_tile<T: Actor + 'static>(mut self, index: usize, contents: T) -> Map {
        self.tiles[index].fill(Box::new(contents));
        self
    }

    /// Fills a random open `Tile` with the given contents
    pub fn fill_random_tile<T: Actor + 'static>(self, contents: T) -> Map {
        match self.get_random_open_tile() {
            Some(t) => self.fill_tile(t, contents),
            None => self
        }
    }

    /// Finds a random `Tile` that is open (available to hold something)
    pub fn get_random_open_tile(&self) -> Option<usize> {
        self.get_random_tile(|i, ref t|
            t.kind() == TileType::Floor &&
            t.contents().is_none() &&
            Direction::cardinals().into_iter().all(|d|
                self.get_neighbouring_tile_index(i, d)
                    .map(|n| self.tiles[n].kind() != TileType::Door)
                    .unwrap_or(false)
            )
        )
    }

    /// Finds the index of a random `Tile` that matches a predicate
    pub fn get_random_tile<F: Fn(usize, &Tile) -> bool>(&self, pred: F) -> Option<usize> {
        let options: Vec<usize> = self.tiles.iter().enumerate().filter(|&(i, tile)| pred(i, tile)).map(|(i, _)| i).collect();
        thread_rng().choose(&options).map(|i| *i)
    }

    /// Determines the direction between two points on the `Map`
    pub fn get_direction(&self, from: usize, to: usize) -> Direction {
        let f = self.split_coordinate(from);
        let t = self.split_coordinate(to);
        Direction::between(f, t)
    }

    /// Determines the walking distance between two points on the `Map`
    pub fn get_distance(&self, from: usize, to: usize) -> usize {
        let mut dists = self.distances.borrow_mut();
        if dists.contains_key(&from) {
            dists.get(&from).unwrap()[to]
        } else if dists.contains_key(&to) {
            dists.get(&to).unwrap()[from]
        } else {
            let mut distances = vec![usize::MAX; self.tiles.len()];
            distances[from] = 0;
            self.calc_distance(&mut distances, from);
            dists.insert(from, distances);
            dists.get(&from).unwrap()[to]
        }
    }

    fn calc_distance(&self, distances: &mut Vec<usize>, from: usize) {
        use self::TileType::*;
        for n in Direction::cardinals().into_iter().filter_map(|dir| self.get_neighbouring_tile_index(from, dir)) {
            if self.tiles[n].kind() == Hall || self.tiles[n].kind() == Floor || self.tiles[n].kind() == Door {
                let dist = min(distances[n], distances[from] + 1);
                if dist != distances[n] {
                    distances[n] = dist;
                    self.calc_distance(distances, n);
                }
            }
        }
    }
}
