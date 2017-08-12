use super::super::actors::Actor;
use inputter::Action;

/// A TileType determines the geography of each tile
#[derive(PartialEq, Eq, Clone)]
pub enum TileType {
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
    /// *   `Wall`  → `x`
    /// *   `Hall`  → `#`
    /// *   `Door`  → `+`
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
#[derive(Clone)]
pub struct Tile {
    pub kind: TileType,
    contents: Option<Box<Actor>>,
}

impl Tile {
    pub fn new(kind: TileType) -> Tile {
        Tile{ kind, contents: None }
    }

    /// Move this Cell's contents to the provided cell, destroying what was there
    pub fn move_to(self, tile: Tile) -> (Tile, Tile) {
        match self.contents {
            None => (self, tile),
            Some(ref actor) if actor.can_enter(&tile.kind) =>
                (   // TODO: any way to avoid cloning here?
                    Tile{ kind: self.kind, contents: None },
                    Tile{ kind: tile.kind, contents: self.contents.clone() },
                ),
            _ => (self, tile)
        }
    }

    /// Destroys the Cell's contents
    pub fn empty(&mut self) {
        self.contents = None;
    }

    pub fn fill(&mut self, actor: Box<Actor>) {
        self.contents = Some(actor);
    }

    /// Determines what symbol should be displayed for this tile, taking into account its contents
    pub fn symbol(&self) -> char {
        self.contents.as_ref().map_or(self.empty_symbol(), |ref c| c.symbol())
    }

    /// Determines what symbol should be displayed for this tile when it is empty
    pub fn empty_symbol(&self) -> char {
        self.kind.symbol()
    }

    pub fn process(&self, action: Action) -> Action {
        self.contents.as_ref().map_or(Action::Idle, |ref c| c.react(action))
    }
}
