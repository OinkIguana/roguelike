extern crate rand;
extern crate pancurses;

pub mod engine;
pub mod populator;
pub mod generator;
pub mod curses;
pub mod actors;

use pancurses::{initscr,endwin,noecho,curs_set};

use engine::Populator;

fn main() {
    let window = initscr();
    let display = curses::Curses::new(&window);
    let populator = populator::Easy::new;
    noecho();
    curs_set(0);
    window.keypad(true);
    let mut e = engine::Engine::new(
        display,
        generator::Foggy,
        &populator,
    );
    e.run();
    endwin();
}
