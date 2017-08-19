// TODO: make this into a trait, and move the implementation out of the engine
use super::Map;

pub trait Generator {
    /// Generates a map of the specified dimensions, maybe based on the complexity
    fn generate(&self, complexity: u32, width: usize, height: usize) -> Map;
}
