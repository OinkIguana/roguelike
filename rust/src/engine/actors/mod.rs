use super::super::inputter::Action;

/// An Actor is the basic building block of every item or character in the game
#[allow(unused_variables)]
pub trait Actor {
    fn react(&self, action: &Action) {}

    fn step_on(&self, other: &Actor) {}
    fn be_stepped_on(&self, other: &Actor) {}

    fn interact(&self, other: &Actor) {}
    fn be_interacted_with(&self, other: &Actor) {}

    fn attack(&self, other: &Actor) {}
    fn be_attacked(&self, other: &Actor) {}

    fn symbol(&self) -> char { ' ' }
}
