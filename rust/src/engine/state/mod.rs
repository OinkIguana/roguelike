use inputter::Action;
use super::map::Map;

/// A `State` represents the current state of the game. By serializing the state, the entire game
/// should be reproducable exactly as it was before.
pub struct State { // TODO: are all the fields pub?
    /// The dungeon map
    pub map: Map,
    /// The total score accumulated by the player
    pub score: u32,
    /// The current floor of the dungeon the player is on
    pub level: u32,
    /// Whether the game has been quit by the player
    pub quit: bool,
}


impl State {
    /// Creates the initial state
    pub fn new() -> State {
        State{ map: Map::new(1), score: 0, level: 1, quit: false }
    }

    /// Sets the quit field of the State
    pub fn quit(self) -> State {
        State{ map: self.map, score: self.score, level: self.level, quit: true }
    }

    /// Takes an Action and the previous state and produces the next state
    /// kind of like a flux reducer...
    pub fn process(self, action: Action) -> State {
        match action {
            Action::Quit    => self.quit(),
            _               => {
                let actions = self.map.process(action);
                self.process_all(actions)
            }
        }
    }

    fn process_all(self, actions: Vec<Action>) -> State {
        let mut map = self.map;
        for (index, action) in actions.iter().enumerate() {
            map = map.react(action.clone(), index)
        }
        State{ map, score: self.score, level: self.level, quit: self.quit }
    }
}
