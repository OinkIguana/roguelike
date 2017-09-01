use std::fmt::{Display,Formatter,Result};

/// A `TileType` determines the geography of each `Tile`
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TileType {
    Floor,
    Wall,
    Hall,
    Door,
    Empty,
}

impl TileType {
    /// The symbol that represents this `TileType`
    ///
    /// *   `Floor` → `.`
    /// *   `Wall`  → `|`
    /// *   `Hall`  → `#`
    /// *   `Door`  → `+`
    /// *   `Empty` → ` `
    pub fn symbol(&self) -> char {
        match *self {
            TileType::Floor => '.',
            TileType::Wall  => '|',
            TileType::Hall  => '#',
            TileType::Door  => '+',
            TileType::Empty => ' ',
        }
    }
}

impl Display for TileType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.symbol())
    }
}
