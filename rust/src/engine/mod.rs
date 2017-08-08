use outputter::Outputter;
use inputter::{Inputter,Action};

pub struct Engine<I: Inputter, O: Outputter> {
    input: I,
    output: O,
}

impl<I: Inputter, O: Outputter> Engine<I, O> {
    pub fn new(input: I, output: O) -> Engine<I, O> {
        Engine{ input, output }
    }

    pub fn run(&self) {
        self.process(self.input.get());
        self.output.render();
    }

    fn process(&self, input: Action) {
        match input {
            Action::Quit => (),
            Action::Idle => (),
            Action::Move(dir) => (),
            Action::Attack(dir) => (),
            Action::Interact(dir) => (),
        }
    }
}
