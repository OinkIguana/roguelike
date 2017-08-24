extern crate rand;
extern crate pancurses;

mod engine;
mod populator;
mod generator;
mod curses;
mod actors;

use pancurses::{initscr,endwin,noecho,curs_set};

use engine::Populator;

fn main() {
    let window = initscr();
    let populator = populator::Easy::new;
    noecho();
    curs_set(0);
    window.keypad(true);
    let mut e = engine::Engine::new(
        curses::Input::new(&window),
        curses::Output::new(&window),
        generator::Standard,
        &populator,
    );
    e.run();
    endwin();
}
