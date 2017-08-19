use std::mem::replace;
use super::{Action,Map,Direction};

pub struct Switch<T: Behavior>(pub Vec<T>);
impl<T: Behavior> Behavior for Switch<T> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        for b in self.0.iter() {
            if b.exec(i, map) { return true; }
        }
        false
    }
}

pub struct Perform(pub Action);
impl Behavior for Perform {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        match self.0 {
            Action::Move(dir) => {
                if let (Some(neighbour), Some(mut me)) = (map.get_neighbouring_tile_index(i, dir), map.tiles[i].contents().clone()) {
                    if let Some(mut them) = map.tiles[neighbour].contents().clone() {
                        me.step_on(&mut *them);
                        them.be_stepped_on(&mut *me);
                        map.tiles[i].fill(me);
                        map.tiles[neighbour].fill(them);
                    }
                    let (a, b) = map.tiles[i].clone().move_to(map.tiles[neighbour].clone());
                    replace(&mut map.tiles[i], a);
                    replace(&mut map.tiles[neighbour], b);
                    true
                } else {
                    false
                }
            }
            Action::Attack(dir) => {
                if let (Some(neighbour), Some(mut me)) = (map.get_neighbouring_tile_index(i, dir), map.tiles[i].contents().clone()) {
                    if let Some(mut them) = map.tiles[neighbour].contents().clone() {
                        me.attack(&mut *them);
                        them.be_attacked(&mut *me);
                        map.tiles[i].fill(me);
                        map.tiles[neighbour].fill(them);
                    }
                    true
                } else {
                    false
                }
            }
            Action::Interact(dir) => {
                if let (Some(neighbour), Some(mut me)) = (map.get_neighbouring_tile_index(i, dir), map.tiles[i].contents().clone()) {
                    if let Some(mut them) = map.tiles[neighbour].contents().clone() {
                        me.interact(&mut *them);
                        them.be_interacted_with(&mut *me);
                        map.tiles[i].fill(me);
                        map.tiles[neighbour].fill(them);
                    }
                    true
                } else {
                    false
                }
            }
            _ => true
        }
    }
}

pub struct IfAttackable<T: Behavior>(pub Direction, pub T);
impl<T: Behavior> Behavior for IfAttackable<T> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        let attackable = map
            .get_neighbouring_tile_index(i, self.0)
            .map(|n| &map.tiles[n])
            .and_then(|t| t.contents().clone())
            .map(|a| a.can_be_attacked())
            .unwrap_or(false);
        if attackable {
            self.1.exec(i, map)
        } else {
            false
        }
    }
}

pub struct IfOpen<T: Behavior>(pub Direction, pub T);
impl<T: Behavior> Behavior for IfOpen<T> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        let open = map
            .get_neighbouring_tile_index(i, self.0)
            .map(|n| &map.tiles[n])
            .map(|t| map.tiles[i].contents().clone().map(|a| a.can_enter(t.kind)).unwrap_or(false) && t.contents().is_none())
            .unwrap_or(false);
        if open {
            self.1.exec(i, map)
        } else {
            false
        }
    }
}

pub struct IfEnterable<T: Behavior>(pub Direction, pub T);
impl<T: Behavior> Behavior for IfEnterable<T> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        let open = map
            .get_neighbouring_tile_index(i, self.0)
            .map(|n| &map.tiles[n])
            .map(|t|
                map.tiles[i].contents().clone().map(|a| a.can_enter(t.kind)).unwrap_or(false) &&
                t.contents().clone().map(|a| a.can_be_stepped_on()).unwrap_or(true))
            .unwrap_or(false);
        if open {
            self.1.exec(i, map)
        } else {
            false
        }
    }
}

pub struct Then<T: Behavior, U: Behavior>(T, U);
impl<T: Behavior, U: Behavior> Behavior for Then<T, U> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        let a = self.0.exec(i, map);
        let b = self.1.exec(i, map);
        a || b
    }
}

pub struct OrElse<T: Behavior, U: Behavior>(T, U);
impl<T: Behavior, U: Behavior> Behavior for OrElse<T, U> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        self.0.exec(i, map) || self.1.exec(i, map)
    }
}

pub trait Behavior {
    fn then<U>(self, next: U) -> Then<Self, U>
    where Self: Behavior + Sized, U: Behavior {
        Then(self, next)
    }

    fn or_else<U>(self, next: U) -> OrElse<Self, U>
    where Self: Behavior + Sized, U: Behavior {
        OrElse(self, next)
    }

    fn exec(&self, i: usize, &mut Map) -> bool;
}
