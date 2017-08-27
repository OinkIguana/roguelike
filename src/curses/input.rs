use pancurses::{Window,Input as UInput};
use engine::{Direction,Inputter,Action};
use super::{Output,InputSignal};

pub struct Input<'a> {
    window: &'a Window,
    facing: Direction,
    output: &'a Output<'a>,
}
impl<'a> Input<'a> {
    pub fn new(window: &'a Window, output: &'a Output<'a>) -> Input<'a> {
        Input{ window, facing: Direction::S, output }
    }

    pub fn get_item(&mut self, index: usize) -> Action {
        match self.window.getch() {
            Some(UInput::KeyEnter)          => Action::Use(index),
            Some(UInput::KeyUp)             => self.get_item(index - 1),
            Some(UInput::KeyDown)           => self.get_item(index + 1),
            Some(UInput::Character('i')) |
            // is that escape?
            Some(UInput::KeyF0)   => {
                self.output.receive_input(InputSignal::CloseInventory);
                self.get()
            }
            _ => self.get_item(index),
        }
    }
}
impl<'a> Inputter for Input<'a> {
    fn get(&mut self) -> Action {
        match self.window.getch() {
            Some(UInput::Character('q')) => Action::Quit,
            Some(UInput::Character('z')) => Action::Interact(self.facing),
            Some(UInput::Character('x')) => Action::Attack(self.facing),
            Some(UInput::KeyUp) => {
                self.facing = Direction::N;
                Action::Move(self.facing)
            }
            Some(UInput::KeyDown) => {
                self.facing = Direction::S;
                Action::Move(self.facing)
            }
            Some(UInput::KeyLeft) => {
                self.facing = Direction::W;
                Action::Move(self.facing)
            }
            Some(UInput::KeyRight) => {
                self.facing = Direction::E;
                Action::Move(self.facing)
            }
            None => Action::Idle,
            // hope this has tail call optimization
            Some(UInput::Character('i')) => {
                self.output.receive_input(InputSignal::OpenInventory);
                self.get_item(0)
            }
            Some(_) => self.get(),
        }
    }
}
