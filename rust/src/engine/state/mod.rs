use super::map::Map;

/// A `State` represents the current state of the game. By serializing the state, the entire game
/// should be reproducable exactly as it was before.
pub struct State { // TODO: are all the fields pub?
    /// The dungeon map
    pub map: Map,
    /// The total score accumulated by the player
    pub score: i32,
    /// Whether the game has been quit by the user
    pub quit: bool,
}


impl State {
    /// Creates the initial state
    pub fn new() -> State {
        State{ map: Map::new(0, 0), score: 0, quit: false }
    }

    /// Sets the quit field of the State
    pub fn quit(self) -> State {
        State{ map: self.map, score: self.score, quit: true }
    }
}
