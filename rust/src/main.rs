extern crate pancurses;

mod engine;
mod outputter;
mod inputter;
mod curses;

use pancurses::{initscr, endwin};

fn main() {
    let window = initscr();
    let e = engine::Engine::new(curses::Input::new(&window), curses::Display::new(&window));
    e.run();
    endwin();
}
