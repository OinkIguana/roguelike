pub mod state;
pub mod map;
pub mod actor;
pub mod behavior;
pub mod query;
pub mod outputter;
pub mod inputter;
pub mod messaging;
pub mod direction;
pub mod one_two;

pub use self::outputter::*;
pub use self::inputter::*;
pub use self::state::*;
pub use self::map::*;
pub use self::actor::*;
pub use self::behavior::*;
pub use self::query::*;
pub use self::messaging::*;
pub use self::direction::*;

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
