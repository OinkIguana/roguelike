use engine::Direction;

/// An Inputter is used as an interface that the user can provide input which is passed to the
/// Engine
pub trait Inputter {
    /// Retrieves a single action from the user
    fn get(&self) -> Action;
}

/// An Action describes the actions that the player can take
#[derive(Clone)]
#[derive(Debug)]
pub enum Action {
    Idle,
    Attack(Direction),
    Interact(Direction),
    Move(Direction),
    Quit,
}
