use super::Map;
use super::super::{BState,Messenger};

/// A Populator is used to fill a Map with characters and items.
pub trait Populator {
    fn new(messenger: Messenger) -> Self;
    /// The populate method is the one that does the actual populating
    fn populate(&self, state: BState) -> Map;
}
