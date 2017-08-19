use super::Map;

/// A Populator is used to fill a Map with characters and items.
pub trait Populator {
    /// The populate method is the one that does the actual populating
    fn populate(self: &Self, map: Map) -> Map;
}
