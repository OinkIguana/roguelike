use pancurses::{Window,Input as UInput};
use engine::{Direction,Inputter,Action};

pub struct Input<'a> {
    window: &'a Window,
    facing: Direction,
}
impl<'a> Input<'a> {
    pub fn new(window: &'a Window) -> Input<'a> {
        Input{ window, facing: Direction::S }
    }
}
impl<'a> Inputter for Input<'a> {
    fn get(&mut self) -> Action {
        let a = self.window.getch();
        match a {
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
            Some(_) => self.get(), // hope this gets tail call optimized...
            None => Action::Idle,
        }
    }
}
