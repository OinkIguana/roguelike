/// An Inputter is used as an interface that the user can provide input which is passed to the
/// Engine
pub trait Inputter {
    /// Retrieves a single action from the user
    fn get(&self) -> Action;
}
/// Directions correspond to the cardinal directions and are used to indicate which way to move
#[allow(dead_code)]
pub enum Direction { N, S, W, E, NW, NE, SW, SE }
/// An Action describes the actions that the player can take
pub enum Action {
    Idle,
    Attack,
    Interact,
    Move(Direction),
    Quit,
}
