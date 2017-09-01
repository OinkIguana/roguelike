use super::Direction;

/// An `Inputter` processes the user inputs and converts them into basic `Actions` to be consumed
/// by the `Engine`
pub trait Inputter {
    /// Retrieves a single action from the user
    fn get(&mut self) -> Action;
}

/// An `Action` describes the most basic actions that the player can take
#[derive(Clone, Debug)]
pub enum Action {
    /// Do nothing
    Idle,
    /// Attack in a direction
    Attack(Direction),
    /// Interact with a neighbour
    Interact(Direction),
    /// Move in a direction
    Move(Direction),
    /// Use an item from the inventory
    Use(usize),
    /// Quit the game
    Quit,
}
