use super::map::TileType;
use super::inputter::Action;

/// An Actor is the basic building block of every item or character in the game
#[allow(unused_variables)]
pub trait Actor: ActorClone {
    /// Consumes an input action and produces the corresponding game action that should be taken
    fn react(&self, action: Action) -> Action { Action::Idle }

    fn can_be_stepped_on(&self) -> bool { false }
    fn step_on(&self, other: &Actor) {}
    fn be_stepped_on(&self, other: &Actor) {}

    fn can_be_interacted_with(&self) -> bool { false }
    fn interact(&self, other: &Actor) {}
    fn be_interacted_with(&self, other: &Actor) {}

    fn can_be_attacked(&self) -> bool { false }
    fn attack(&self, other: &Actor) {}
    fn be_attacked(&self, other: &Actor) {}

    fn on_destroy(&self) {}

    fn symbol(&self) -> char { ' ' }
    fn can_enter(&self, tile: &TileType) -> bool {
        match tile {
            &TileType::Wall | &TileType::Empty => false,
            _ => true,
        }
    }
}

pub trait ActorClone {
    fn clone_box(&self) -> Box<Actor>;
}
impl<T> ActorClone for T where T: 'static + Actor + Clone {
    fn clone_box(&self) -> Box<Actor> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Actor> {
    fn clone(&self) -> Box<Actor> {
        self.clone_box()
    }
}
