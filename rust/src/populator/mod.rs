use engine::map::{Map,Populator};

/// A Populator for an easy game
pub struct Easy;
impl Populator for Easy {
    fn populate(&self, map: Map) -> Map {
        map
    }
}
