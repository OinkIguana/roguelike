pub mod state;
pub mod map;
mod actors;

use outputter::Outputter;
use inputter::Inputter;
use self::state::State;

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

/// Directions correspond to the cardinal directions and are used to indicate which way to move
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction { NW, N, NE, E, SE, S, SW, W }
impl Direction {
    pub fn cardinals() -> Vec<Direction> {
        vec![
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
        ]
    }
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
