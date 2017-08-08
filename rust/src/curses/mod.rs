use pancurses::Window;
use inputter::{Inputter,Action};
use outputter::Outputter;

pub struct Display<'a> {
    pub window: &'a Window,
}
impl<'a> Outputter for Display<'a> {
    fn render(&self) {}
}

pub struct Input<'a> {
    pub window: &'a Window,
}
impl<'a> Inputter for Input<'a> {
    fn get(&self) -> Action { Action::Idle }
}
