use pancurses::{Window,Input as UInput};
use inputter::{Inputter,Action,Direction};

pub struct Input<'a> {
    window: &'a Window,
}
impl<'a> Input<'a> {
    pub fn new(window: &'a Window) -> Input<'a> {
        Input{ window }
    }
}
impl<'a> Inputter for Input<'a> {
    fn get(&self) -> Action {
        match self.window.getch() {
            Some(UInput::Character('q')) => Action::Quit,
            Some(UInput::Character('z')) => Action::Interact,
            Some(UInput::Character('x')) => Action::Attack,
            Some(UInput::KeyUp) => Action::Move(Direction::N),
            Some(UInput::KeyDown) => Action::Move(Direction::S),
            Some(UInput::KeyLeft) => Action::Move(Direction::W),
            Some(UInput::KeyRight) => Action::Move(Direction::E),
            Some(_) => Action::Idle,
            None => Action::Idle,
        }
    }
}
