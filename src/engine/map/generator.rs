use super::Map;

/// A `Generator` is used to create the dungeon map, and is provided to the `Engine` when the game
/// is started
pub trait Generator {
    /// Generates a map of the specified dimensions, maybe based on the complexity
    fn generate(&self, complexity: u32, width: usize, height: usize) -> Map;
}
