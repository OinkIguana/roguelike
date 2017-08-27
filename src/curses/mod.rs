use std::str::from_utf8;
use pancurses::{Window,Input as UInput};
use engine::{Direction,Outputter,Inputter,Action,State,Generator,Populator,Messenger};

pub struct Curses<'a> {
    window: &'a Window,
    facing: Direction,
}

impl<'a> Curses<'a> {
    pub fn new(window: &'a Window) -> Curses<'a> {
        Curses{ window, facing: Direction::S }
    }

    pub fn get_item(&mut self, index: usize) -> Action {
        match self.window.getch() {
            Some(UInput::KeyEnter)          => Action::Use(index),
            Some(UInput::KeyUp)             => self.get_item(index - 1),
            Some(UInput::KeyDown)           => self.get_item(index + 1),
            Some(UInput::Character('i')) |
            // is that escape?
            Some(UInput::KeyExit)   => {
                self.get()
            }
            _ => self.get_item(index),
        }
    }

    fn clear(&self) {
        self.window.clear();
    }
}

impl<'a> Inputter for Curses<'a> {
    fn get(&mut self) -> Action {
        match self.window.getch() {
            Some(UInput::Character('q')) => Action::Quit,
            Some(UInput::Character('z')) => Action::Interact(self.facing),
            Some(UInput::Character('x')) => Action::Attack(self.facing),
            Some(UInput::KeyUp) => {
                self.facing = Direction::N;
                Action::Move(self.facing)
            }
            Some(UInput::KeyDown) => {
                self.facing = Direction::S;
                Action::Move(self.facing)
            }
            Some(UInput::KeyLeft) => {
                self.facing = Direction::W;
                Action::Move(self.facing)
            }
            Some(UInput::KeyRight) => {
                self.facing = Direction::E;
                Action::Move(self.facing)
            }
            None => Action::Idle,
            // hope this has tail call optimization
            Some(UInput::Character('i')) => {
                self.get_item(0)
            }
            Some(_) => self.get(),
        }
    }
}

impl<'a> Outputter for Curses<'a> {
    fn render<G: Generator, P: Populator, F: Fn(Messenger) -> P>(&self, state: &State<G, P, F>) {
        self.clear();
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
