use std::mem::replace;
use super::{Action,Map,Direction};

pub struct Start;
impl Behavior for Start {
    fn exec(&self, _: usize, _: &mut Map) -> bool { false }
}

pub struct Switch<T: Behavior, I>(I) where I: Iterator<Item=Box<T>>;
impl<T: Behavior, I> Behavior for Switch<T, I> where I: Iterator<Item=Box<T>> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        for b in self.0 {
            if b.exec(i, map) { return true; }
        }
        false
    }
}

pub struct Perform(Action);
impl Behavior for Perform {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        match self.0 {
            Action::Move(dir) => {
                for neighbour in map.get_neighbouring_tile_index(i, dir) {
                    let (a, b) = map.tiles[i].clone().move_to(map.tiles[neighbour].clone());
                    replace(&mut map.tiles[i], a);
                    replace(&mut map.tiles[neighbour], b);
                    return true;
                }
                return false;
            },
            _ => false
        }
    }
}

pub struct Then<T: Behavior, U: Behavior>(T, U);
impl<T: Behavior, U: Behavior> Behavior for Then<T, U> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        self.0.exec(i, map) || self.1.exec(i, map)
    }
}

pub struct IfAttackable<T: Behavior>(Direction, T);
impl<T: Behavior> Behavior for IfAttackable<T> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        let attackable = map
            .get_neighbouring_tile_index(i, self.0)
            .map(|n| &map.tiles[n])
            .and_then(|t| t.contents().clone())
            .map(|a| a.can_be_attacked())
            .unwrap_or(false);
        if attackable {
            return self.1.exec(i, map)
        }
        attackable
    }
}

pub trait Behavior {
    fn then<U>(self, next: U) -> Box<Then<Self, U>>
    where Self: Behavior + Sized, U: Behavior {
        Box::new(Then(self, next))
    }

    fn switch<T: Behavior, I>(self, iter: I) -> Box<Switch<T, I>>
    where Self: Behavior + Sized, I: Iterator<Item=Box<T>> {
        Box::new(Switch(iter))
    }

    fn if_attackable<T>(self, dir: Direction, b: T) -> Box<IfAttackable<T>>
    where Self: Behavior + Sized, T: Behavior {
        Box::new(IfAttackable(dir, b))
    }

    fn exec(&self, i: usize, &mut Map) -> bool;
}

pub fn start() -> Box<Start> {
    Box::new(Start)
}

pub fn perform(action: Action) -> Box<Perform> {
    Box::new(Perform(action))
}
