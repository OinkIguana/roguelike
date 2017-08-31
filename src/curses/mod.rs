use std::str::from_utf8;
use pancurses::{Window,Input as UInput};
use engine::{Direction,Outputter,Inputter,Action,BState};

pub struct Curses<'a> {
    window: &'a Window,
    facing: Direction,
    prev_state: Option<BState>,
}

impl<'a> Curses<'a> {
    pub fn new(window: &'a Window) -> Curses<'a> {
        Curses{ window, facing: Direction::S, prev_state: None }
    }

    pub fn get_item(&mut self, index: usize) -> Action {
        self.render_inventory(index);
        match self.window.getch() {
            Some(UInput::Character('z'))    => Action::Use(index),
            Some(UInput::KeyUp)             => {
                let len = self.prev_state.as_ref().map(|ref s| s.inventory.len()).unwrap_or(1);
                self.get_item((index + len - 1) % len)
            }
            Some(UInput::KeyDown)           => {
                let len = self.prev_state.as_ref().map(|ref s| s.inventory.len()).unwrap_or(1);
                self.get_item((index + 1) % len)
            }
            Some(UInput::Character('i'))    => {
                let ps = self.prev_state.clone().unwrap();
                self.render(ps);
                self.get()
            }
            _ => self.get_item(index),
        }
    }

    fn clear(&self) {
        self.window.clear();
    }

    pub fn render_inventory(&self, index: usize) {
        self.clear();
        for (i, item) in self.prev_state.clone().unwrap().inventory.iter().enumerate() {
            if i == index {
                self.window.mvaddch(3 + i as i32, 2, '>');
            }
            self.window.mvaddstr(3 + i as i32, 3, item);
        }
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
                let len = self.prev_state.as_ref().map(|ref s| s.inventory.len()).unwrap_or(1);
                if len > 0 {
                    self.get_item(0)
                } else {
                    self.get()
                }
            }
            Some(_) => self.get(),
        }
    }
}

impl<'a> Outputter for Curses<'a> {
    fn render(&mut self, state: BState) {
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
            self.window.mv(0, 0);
            self.window.vline('|', state.map.height as i32 + 2);
            self.window.mv(0, state.map.width as i32 + 1);
            self.window.vline('|', state.map.height as i32 + 2);
        }
        self.window.mvaddstr(state.map.height as i32 + 3, 0, &format!("Money: {} | Health: {}", state.money, state.health));
        self.window.refresh();
        self.prev_state = Some(state);
    }
}
