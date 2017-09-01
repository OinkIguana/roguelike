use std::mem::replace;
use super::{Action,Map,Direction,Query};

/// A `Switch` tries executing each of the `Behavior`s in the vector, stopping after the first
/// success. It is considered a success if one of the `Behaviors` is successful.
pub struct Switch<T: Behavior>(pub Vec<T>);
impl<T: Behavior> Behavior for Switch<T> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        for b in self.0.iter() {
            if b.exec(i, map) { return true; }
        }
        false
    }
}

/// `IfInteractable` executes its `Behavior` if the `Tile` in the given direction has contents
/// which can be attacked. It is considered a success if the `Tile` is attackable and the
/// `Behavior` executes successfully.
pub struct IfAttackable<T: Behavior>(pub Direction, pub T);
impl<T: Behavior> Behavior for IfAttackable<T> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        if let Some(me) = map.tiles[i].contents().clone() {
            let attackable = map
                .get_neighbouring_tile_index(i, self.0)
                .map(|n| &map.tiles[n])
                .and_then(|t| t.contents().clone())
                .map(|a| a.can_be_attacked(&*me))
                .unwrap_or(false);
            if attackable {
                self.1.exec(i, map)
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// `IfInteractable` executes its `Behavior` if the `Tile` in the given direction has contents which
/// can be interacted with. It is considered a success if the `Tile` is interactable and the
/// `Behavior` executes successfully.
pub struct IfInteractable<T: Behavior>(pub Direction, pub T);
impl<T: Behavior> Behavior for IfInteractable<T> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        if let Some(me) = map.tiles[i].contents().clone() {
            let interactable = map
                .get_neighbouring_tile_index(i, self.0)
                .map(|n| &map.tiles[n])
                .and_then(|t| t.contents().clone())
                .map(|a| a.can_be_interacted_with(&*me))
                .unwrap_or(false);
            if interactable {
                self.1.exec(i, map)
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// `IfOpen` executes its `Behavior` if the `Tile` in the given direction is open. An open `Tile`
/// is one that is of a kind that the current Actor can enter which currently has no contents. It
/// is considered a success if the `Tile` is open and the `Behavior` executes successfully.
pub struct IfOpen<T: Behavior>(pub Direction, pub T);
impl<T: Behavior> Behavior for IfOpen<T> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        let open = map
            .get_neighbouring_tile_index(i, self.0)
            .map(|n| &map.tiles[n])
            .map(|t| map.tiles[i].contents().clone().map(|a| a.can_enter(t.kind())).unwrap_or(false) && t.contents().is_none())
            .unwrap_or(false);
        if open {
            self.1.exec(i, map)
        } else {
            false
        }
    }
}

/// `IfEnterable` executes its `Behavior` if the `Tile` in the given direction is enterable. An
/// enterable `Tile` is one that has a `TileType` that the current `Actor` can enter which contains
/// either no contents or contents that can be stepped on. It is considered a success if the `Tile`
/// is enterable and the `Behavior` executes successfully.
pub struct IfEnterable<T: Behavior>(pub Direction, pub T);
impl<T: Behavior> Behavior for IfEnterable<T> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        if let Some(me) = map.tiles[i].contents().clone() {
            let open = map
                .get_neighbouring_tile_index(i, self.0)
                .map(|n| &map.tiles[n])
                .map(|t|
                    map.tiles[i].contents().clone().map(|a| a.can_enter(t.kind())).unwrap_or(false) &&
                    t.contents().clone().map(|a| a.can_be_stepped_on(&*me)).unwrap_or(true))
                .unwrap_or(false);
            if open {
                return self.1.exec(i, map);
            }
        }
        false
    }
}

/// `ExecQuery` executes a `Query`, then passes the result to the given function which can then
/// create a `Behavior` based on the result. It is considered a success if the `Query` and
/// `Behavior` are both executed successfully.
pub struct ExecQuery<R, Q, B, F>(pub Q, pub F)
where   Q: Query<R=R>,
        B: Behavior,
        F: Fn(R) -> B;
impl<R, Q, B, F> Behavior for ExecQuery<R, Q, B, F>
where   Q: Query<R=R>,
        B: Behavior,
        F: Fn(R) -> B {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        if let Some(res) = self.0.exec(map) {
            self.1(res).exec(i, map)
        } else {
            false
        }
    }
}

/// `Then` executes the first `Behavior` and then the second, considering it a success if the
/// second `Behavior` completes successfully.
pub struct Then<T: Behavior, U: Behavior>(T, U);
impl<T: Behavior, U: Behavior> Behavior for Then<T, U> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        self.0.exec(i, map);
        self.1.exec(i, map)
    }
}

/// `OrElse` tries to perform the first `Behavior`. If it fails, it will perform the second one
/// instead. It is considered a success if either `Behavior` completes successfully.
pub struct OrElse<T: Behavior, U: Behavior>(T, U);
impl<T: Behavior, U: Behavior> Behavior for OrElse<T, U> {
    fn exec(&self, i: usize, map: &mut Map) -> bool {
        self.0.exec(i, map) || self.1.exec(i, map)
    }
}

/// `Perform` performs a basic `Action`. It is considered a success if the `Action` is carried out
/// completely.
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
                    if let Some(mut me) = map.tiles[neighbour].contents().clone() {
                        me.on_move();
                        map.tiles[neighbour].fill(me);
                    }
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
            Action::Use(slot) => {
                if let Some(mut me) = map.tiles[i].contents().clone() {
                    if let Some(mut item) = me.get_item(slot) {
                        if item.can_be_used(&*me) {
                            me.use_item(&mut *item);
                            if let Some(used_item) = item.be_used(&mut *me) {
                                me.pick_up(used_item);
                            }
                            return true;
                        } else {
                            me.pick_up(item);
                        }
                    }
                }
                false
            }
            Action::Idle => {
                if let Some(mut me) = map.tiles[i].contents().clone() {
                    me.on_idle();
                    map.tiles[i].fill(me);
                }
                true
            },
            _ => panic!("Quit should not be processed!")
        }
    }
}

/// A `Behavior` represents the behaviour of an `Actor` by combining various properties.
pub trait Behavior {
    /// Chains this `Behavior` with another by a `Then`.
    fn then<U>(self, next: U) -> Then<Self, U>
    where Self: Behavior + Sized, U: Behavior {
        Then(self, next)
    }

    /// Chains this `Behavior` with another by an `OrElse`.
    fn or_else<U>(self, next: U) -> OrElse<Self, U>
    where Self: Behavior + Sized, U: Behavior {
        OrElse(self, next)
    }

    /// Executes this `Behavior`, returning a boolean that indicates whether the `Behavior` was
    /// completed as expected.
    fn exec(&self, usize, &mut Map) -> bool;
}
