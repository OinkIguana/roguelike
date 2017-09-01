use super::Key;
use engine::*;

/// A Chest which can hold one item, and must be unlocked using a key.
///
/// The item inside is an Actor which must either be able to be held in the inventory or stepped
/// on. If it can be held, the item is placed into the opener's inventory, otherwise the step on
/// event is triggered.
///
/// Symbol: `C` full, `c` empty
#[derive(Clone)]
pub struct Chest<T> where T: Actor {
    contents: Option<T>,
}

impl<T> Chest<T> where T: Actor {
    pub fn new(contents: T) -> Self {
        Chest{ contents: Some(contents) }
    }
}

impl<T> Actor for Chest<T> where T: Actor + Clone + 'static {
    fn can_be_interacted_with(&self, _: &Actor) -> bool { self.contents.is_some() }
    fn be_interacted_with(&mut self, other: &mut Actor) {
        if self.contents.is_some() {
            if let Some(key) = other.find_item(Key::id()) {
                let mut item = self.contents.clone().unwrap();
                other.get_item(key);
                if item.can_be_used(&*other) {
                    other.pick_up(Box::new(item));
                } else if item.can_be_stepped_on(&*other) {
                    other.step_on(&mut item);
                    item.be_stepped_on(&mut *other);
                } else {
                    // TODO: spawn the item on the ground beside the chest in an open space?
                    panic!("Cannot collect item from chest!");
                }
                self.contents.take();
            }
        }
    }

    fn symbol(&self) -> char { if self.contents.is_some() { 'C' } else { 'c' } }
}
