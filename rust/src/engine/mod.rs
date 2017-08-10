pub mod state;
pub mod map;
mod actors;

use outputter::Outputter;
use inputter::{Inputter,Action};
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
            state = self.process(self.input.get(), state);
            self.output.render(&state);
        }
    }

    /// Takes an Action and the previous state and produces the next state
    /// kind of like a flux reducer...
    fn process(&self, input: Action, state: State) -> State {
        match input {
            Action::Quit => state.quit(),
            Action::Idle => state,
            Action::Move(_) => state,
            Action::Attack => state,
            Action::Interact => state,
        }
    }
}
