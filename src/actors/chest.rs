use super::Key;
use engine::*;

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
