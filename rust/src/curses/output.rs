use std::str::from_utf8;
use engine::state::State;
use pancurses::{Window};
use outputter::Outputter;

pub struct Output<'a> {
    window: &'a Window,
}
impl<'a> Output<'a> {
    pub fn new(window: &'a Window) -> Output<'a> {
        Output{ window }
    }
}
impl<'a> Outputter for Output<'a> {
    fn render(&self, state: &State) {
        let map_str: String = state.map.tiles.iter().map(|ref tile| tile.symbol()).collect();
        self.window.mv(0, 0);
        if map_str.len() > 0 {
            for row in map_str.as_bytes().chunks(state.map.width).map(|row| from_utf8(row).unwrap()) {
                self.window.addstr(&(row.to_owned() + "\n"));
            }
        }
        self.window.refresh();
    }
}