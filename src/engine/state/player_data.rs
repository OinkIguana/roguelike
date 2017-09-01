use std::cell::{Cell,RefCell};
use super::super::Actor;
/// The `PlayerData` holds all the information about the player that the game needs to render a
/// meaningful display.
pub struct PlayerData {
    /// The total score accumulated by the player
    pub score: Cell<i32>,
    /// The current money owned by the player
    pub money: Cell<i32>,
    /// The health of the player, to display on the HUD
    pub health: Cell<i32>,
    /// The items currently in the player's inventory
    pub inventory: RefCell<Vec<Box<Actor>>>,
}

impl PlayerData {
    /// Creates a new default `PlayerData`
    pub fn new() -> Self {
        Self {
            score: Cell::new(0),
            money: Cell::new(0),
            health: Cell::new(100),
            inventory: RefCell::new(vec![]),
        }
    }
}
