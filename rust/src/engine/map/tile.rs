use super::super::actors::Actor;
use std::mem::replace;

/// A TileType determines the geography of each tile
#[allow(dead_code)]
enum TileType {
    Floor,
    Wall,
    Hall,
    Door,
    Empty,
}

impl TileType {
    /// The symbol that represents this TileType
    ///
    /// *   `Floor` → `.`
    /// *   `Wall` → `x`
    /// *   `Hall` → `#`
    /// *   `Door` → `+`
    /// *   `Empty` → ` `
    fn symbol(&self) -> char {
        match *self {
            TileType::Floor => '.',
            TileType::Wall  => 'x',
            TileType::Hall  => '#',
            TileType::Door  => '+',
            TileType::Empty => ' ',
        }
    }
}

/// A Tile represents one space in the dungeon. It can have one of a few types, and can
/// optionally hold one Actor
pub struct Tile {
    kind: TileType,
    contents: Option<Box<Actor>>,
}

impl Tile {
    /// Move this Cell's contents to the provided cell, destroying what was there
    #[allow(dead_code)]
    pub fn move_to(&mut self, tile: &mut Tile) {
        tile.contents = replace(&mut self.contents, None);
    }

    /// Destroys the Cell's contents
    #[allow(dead_code)]
    pub fn empty(&mut self) {
        self.contents = None;
    }

    /// Determines what symbol should be displayed for this tile, taking into account its contents
    pub fn symbol(&self) -> char {
        self.contents.as_ref().map_or(self.empty_symbol(), |ref c| c.symbol())
    }

    /// Determines what symbol should be displayed for this tile when it is empty
    pub fn empty_symbol(&self) -> char {
        self.kind.symbol()
    }
}
