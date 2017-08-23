use std::str::from_utf8;
use pancurses::{Window};
use engine::{Outputter,State,Generator,Populator,Messenger};

pub struct Output<'a> {
    window: &'a Window,
}
impl<'a> Output<'a> {
    pub fn new(window: &'a Window) -> Output<'a> {
        Output{ window }
    }
}
impl<'a> Outputter for Output<'a> {
    fn render<G: Generator, P: Populator, F: Fn(Messenger) -> P>(&self, state: &State<G, P, F>) {
        let map_str: String = state.map.tiles.iter()
            .map(|ref tile| tile.symbol())
            .collect();
        if state.map.width > 0 {
            // TODO: large map panning
            self.window.mvaddstr(0, 1, &String::from_utf8(vec![b'-'; state.map.width]).unwrap());
            for (i, row) in map_str.as_bytes().chunks(state.map.width).map(|row| from_utf8(row).unwrap()).enumerate() {
                self.window.mvaddstr(i as i32 + 1, 1, &(row.to_owned()));
            }
            self.window.mvaddstr(state.map.height as i32 + 1, 1, &String::from_utf8(vec![b'-'; state.map.width]).unwrap());
            for i in 0..state.map.height as i32 + 2 {
                self.window.mvaddstr(i, 0, "|");
                self.window.mvaddstr(i, state.map.width as i32 + 1, "|");
            }
        }
        self.window.mvaddstr(state.map.height as i32 + 3, 0, &format!("Money: {} | Health: {}", state.money, state.health));
        self.window.refresh();
    }
}
