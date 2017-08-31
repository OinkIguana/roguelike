use super::BState;

/// An Outputter is used to display the game to the user
pub trait Outputter {
    /// Renderes the current game state to the screen
    fn render(&mut self, state: BState);
}
