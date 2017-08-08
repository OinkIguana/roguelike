pub trait Inputter {
    fn get(&self) -> Action;
}
pub enum Direction { N, S, W, E, NW, NE, SW, SE }
pub enum Action {
    Idle,
    Attack(Direction),
    Interact(Direction),
    Move(Direction),
    Quit,
}
