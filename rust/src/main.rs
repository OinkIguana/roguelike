extern crate pancurses;

mod engine;
mod outputter;
mod inputter;
mod curses;

use pancurses::{initscr, endwin, noecho};

fn main() {
    let window = initscr();
    noecho();
    window.keypad(true);
    let e = engine::Engine::new(curses::Input::new(&window), curses::Output::new(&window));
    e.run();
    endwin();
}
