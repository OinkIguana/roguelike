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
    let output = curses::Output::new(&window);
    let mut input = curses::Input::new(&window, &output);
    let populator = populator::Easy::new;
    noecho();
    curs_set(0);
    window.keypad(true);
    let mut e = engine::Engine::new(
        &mut input,
        &output,
        generator::Standard,
        &populator,
    );
    e.run();
    endwin();
}
