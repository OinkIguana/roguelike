use super::BState;

/// An `Outputter` is used to display the game to the user
pub trait Outputter {
    /// Renders the current game state, provided as a simplified `BState`, to the screen
    fn render(&mut self, state: BState);
}
