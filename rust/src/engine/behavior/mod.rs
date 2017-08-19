use std::iter::FromIterator;
use std::mem::replace;
use super::{Action,Map};

pub struct Start;
impl Behavior for Start {
    fn exec(&self, i: usize, map: &mut Map) {}
}

pub struct Perform(Action);
impl Behavior for Perform {
    fn exec(&self, i: usize, map: &mut Map) {
        match self.0 {
            Action::Move(dir) => {
                for neighbour in map.get_neighbouring_tile_index(i, dir) {
                    let (a, b) = map.tiles[i].clone().move_to(map.tiles[neighbour].clone());
                    replace(&mut map.tiles[i], a);
                    replace(&mut map.tiles[neighbour], b);
                }
            },
            _ => ()
        }
    }
}

pub struct Then<T: Behavior, U: Behavior>(T, U);
impl<T: Behavior, U: Behavior> Behavior for Then<T, U> {
    fn exec(&self, i: usize, map: &mut Map) {
        &self.0.exec(i, map);
        &self.1.exec(i, map);
    }
}

pub trait Behavior {
    fn then<U: Behavior>(self, next: U) -> Then<Self, U> where Self: Behavior + Sized {
        Then(self, next)
    }

    fn exec(&self, i: usize, &mut Map);
}

pub fn start() -> Box<Start> {
    Box::new(Start)
}

pub fn perform(action: Action) -> Box<Perform> {
    Box::new(Perform(action))
}
