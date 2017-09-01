use std::rc::Rc;
use super::Map;
use super::super::{PlayerData,Messenger};

/// A `Populator` is used to fill a Map with characters and items after it has been generated
pub trait Populator {
    fn new(messenger: Messenger) -> Self;
    /// The populate method is the one that does the actual populating. It is given a blank `Map`
    /// as well as a reference to the current `PlayerData` that it can use to set the player's
    /// stats.
    fn populate(&self, map: Map, pd: Rc<PlayerData>) -> Map;
}
