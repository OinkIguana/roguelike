mod state;
mod map;
mod actor;

mod outputter;
mod inputter;

pub use self::outputter::Outputter;
pub use self::inputter::{Inputter,Action};
pub use self::state::State;
pub use self::map::{Map,Populator,TileType,Tile,Generator};
pub use self::actor::{Actor};

/// The Engine encapsulates the behaviours of the game
pub struct Engine<I: Inputter, O: Outputter> {
    input: I,
    output: O,
}

impl<I: Inputter, O: Outputter> Engine<I, O> {
    /// Creates a new engine using the provided input and output mechanisms
    pub fn new(input: I, output: O) -> Engine<I, O> {
        Engine{ input, output }
    }

    /// Runs the game, consuming inputs from the input and outputting to the output until the
    /// game ends or the player quits
    pub fn run(&self) {
        let mut state = State::new();
        while !state.quit {
            self.output.render(&state);
            state = state.process(self.input.get());
        }
    }
}

/// Directions correspond to the cardinal directions and are used to indicate which side to move
/// to, attack, and interact with
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction { NW, N, NE, E, SE, S, SW, W }
impl Direction {
    /// A vector of the 4 cardinal directions, N, E, S, W
    pub fn cardinals() -> Vec<Direction> {
        vec![
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
        ]
    }
    /// A vector of all the the variants, starting from NW and cycling clockwise around the compass
    pub fn variants() -> Vec<Direction> {
        vec![
            Direction::NW,
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
        ]
    }
}
