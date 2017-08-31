use super::{TileType,Action,Behavior,Perform};

/// An Actor is the basic building block of every item or character in the game
#[allow(unused_variables)]
pub trait Actor: ActorClone {
    /// Consumes an input action and produces the Actor's Behavior
    fn react(&self, action: Action) -> Box<Behavior> { Box::new(Perform(Action::Idle)) }

    // Standard game events

    /// Whether the Actor can be stepped on and consumed
    fn can_be_stepped_on(&self, other: &Actor) -> bool { false }
    /// An action to perform when stepping on another Actor
    fn step_on(&mut self, other: &mut Actor) {}
    /// An action to perform when being stepped on by another Actor
    fn be_stepped_on(&mut self, other: &mut Actor) {}

    /// Whether the Actor can be interacted with
    fn can_be_interacted_with(&self, other: &Actor) -> bool { false }
    /// An action to perform when interacting with another Actor
    fn interact(&mut self, other: &mut Actor) {}
    /// An action to perform when being interacted with by another Actor
    fn be_interacted_with(&mut self, other: &mut Actor) {}

    /// Whether the Actor can be used from the inventory
    fn can_be_used(&self, other: &Actor) -> bool { false }
    /// An action to perform when using another Actor as an item
    fn use_item(&mut self, other: &mut Actor) {}
    /// An action to perform when being used by another Actor from the inventory
    fn be_used(&mut self, other: &mut Actor) -> Option<Box<Actor>> { None }

    /// Whether the Actor can be attacked
    fn can_be_attacked(&self, other: &Actor) -> bool { false }
    /// An action to perform when attacking another Actor
    fn attack(&mut self, other: &mut Actor) {}
    /// An action to perform when being attacked by another Actor
    fn be_attacked(&mut self, other: &mut Actor) {}

    // Getters and setters for stats that may or may not be implemented for a given Actor

    /// The power of this Actor's attacks
    fn attack_power(&self) -> u32 { 0 }
    /// An action to perform when money is gained by this Actor
    fn set_money_rel(&mut self, value: i32) {}
    /// An action to perform when this Actor's health should be changed
    fn set_health_rel(&mut self, amount: i32) {}
    /// An action to perform when an Actor picks up another
    fn pick_up(&mut self, item: Box<Actor>) {}
    /// Retrieves the item at the given index owned by this Actor if there is one there.
    /// Should remove the item from the Actor's inventory as well
    fn get_item(&mut self, index: usize) -> Option<Box<Actor>> { None }
    /// Gets the index of the item whose long_name matches the given string, or none otherwise
    fn find_item(&self, name: &str) -> Option<usize> { None }

    /// The symbol that identifies this Actor to the display
    fn symbol(&self) -> char { ' ' }
    /// The human readable full name of this Actor
    fn long_name(&self) -> &str { "" }
    /// Determines whether this Actor is able to enter a given type of tile
    fn can_enter(&self, tile: TileType) -> bool {
        use self::TileType::*;
        match tile {
            Wall | Empty => false,
            _ => true,
        }
    }

    /// Affinity determines how strongly associated with good (positive) or bad (negative) this
    /// Actor is. A value of 0 is neutral
    fn affinity(&self) -> i8 { 0 }

    /// Should return current location (tile index) of this Actor
    ///
    /// This method can be ignored if the Actor will not need to know its location
    fn get_location(&self) -> usize { 0 }
    /// Sets the current location (tile index) of this Actor
    ///
    /// This method can be ignored if the Actor will not need to know its location
    fn set_location(&mut self, location: usize) {}
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
