use super::{TileType,Action,Behavior,Perform};

/// An Actor is the basic building block of every item or character in the game
#[allow(unused_variables)]
pub trait Actor: ActorClone {
    /// Consumes an input action and produces the Actor's Behavior
    fn react(&self, action: Action) -> Box<Behavior> { Box::new(Perform(Action::Idle)) }

    fn can_be_stepped_on(&self) -> bool { false }
    fn step_on(&mut self, other: &mut Actor) {}
    fn be_stepped_on(&mut self, other: &mut Actor) {}

    fn can_be_interacted_with(&self) -> bool { false }
    fn interact(&mut self, other: &mut Actor) {}
    fn be_interacted_with(&mut self, other: &mut Actor) {}

    fn can_be_attacked(&self) -> bool { false }
    fn attack(&mut self, other: &mut Actor) {}
    fn be_attacked(&mut self, other: &mut Actor) {}

    fn on_destroy(&mut self) {}

    fn gain_money(&mut self, value: i32) {}

    fn symbol(&self) -> char { ' ' }
    fn can_enter(&self, tile: TileType) -> bool {
        match tile {
            TileType::Wall | TileType::Empty => false,
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
