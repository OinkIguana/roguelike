use engine::{Actor};

#[derive(Clone)]
pub struct Potion;

impl Actor for Potion {
    fn can_be_stepped_on(&self, _: &Actor) -> bool { true }
    fn be_stepped_on(&mut self, other: &mut Actor) {
        other.pick_up(Box::new(self.clone()));
    }

    fn be_used(self, other: &mut Actor) -> Option<Box<Actor>> {
        other.set_health_rel(25);
        None
    }

    fn symbol(&self) -> char { 'P' }
}
