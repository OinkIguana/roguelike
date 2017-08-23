use super::{State,Generator,Populator,Messenger};

/// An Outputter is used to display the game to the user
pub trait Outputter {
    /// Renderes the current game state to the screen
    fn render<G: Generator, P: Populator, F: Fn(Messenger) -> P>(&self, state: &State<G, P, F>);
}
