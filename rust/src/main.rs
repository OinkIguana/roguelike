extern crate pancurses;

mod engine;
mod outputter;
mod inputter;
mod curses;

use pancurses::{initscr, endwin};

fn main() {
    let window = initscr();
    let e = engine::Engine::new(curses::Input{ window: &window }, curses::Display{ window: &window });
    e.run();
    endwin();
}
