mod state;
mod map;
mod actor;
mod behavior;
mod query;
mod outputter;
mod inputter;
mod messaging;
mod direction;
mod one_two;

pub use self::outputter::Outputter;
pub use self::inputter::{Inputter,Action};
pub use self::state::BState;
use self::state::State;
pub use self::map::{Map,Populator,TileType,Tile,Generator};
pub use self::actor::Actor;
pub use self::behavior::*;
pub use self::query::*;
pub use self::messaging::{Message,Messenger};
pub use self::direction::Direction;

/// The Engine manages the internal behaviour of the game
pub struct Engine<'a, IO: Inputter + Outputter, G: Generator, P: Populator, F: Fn(Messenger) -> P + 'static> {
    display: IO,
    generator: G,
    populator: &'a F,
}

impl<'a, IO: Inputter + Outputter, G: Generator, P: Populator, F: Fn(Messenger) -> P + 'static> Engine<'a, IO, G, P, F> {
    /// Creates a new engine using the provided input and output mechanisms
    pub fn new(display: IO, generator: G, populator: &'a F) -> Engine<'a, IO, G, P, F> {
        Engine{ display, generator, populator }
    }

    /// Runs the game, consuming inputs from the input and outputting to the output until the
    /// game ends or the player quits
    pub fn run(&mut self) {
        let mut state = State::new(&self.generator, self.populator);
        while !state.quit {
            self.display.render(state.simplify());
            state = state.process(self.display.get());
        }
    }
}
