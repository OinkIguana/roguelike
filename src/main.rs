extern crate rand;
extern crate pancurses;

mod engine;
mod populator;
mod generator;
mod curses;
mod actors;

use pancurses::{initscr, endwin, noecho};

fn main() {
    let window = initscr();
    noecho();
    window.keypad(true);
    let e = engine::Engine::new(curses::Input::new(&window), curses::Output::new(&window));
    e.run();
    endwin();
}
