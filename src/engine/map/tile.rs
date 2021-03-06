use std::fmt::{Display,Formatter,Result};
use super::super::{Action,Actor,Behavior,Perform};
use super::tile_type::TileType;

/// A `Tile` represents one space in the dungeon. It can have one of a few types, and can
/// optionally hold one `Actor`
#[derive(Clone)]
pub struct Tile {
    kind: TileType,
    foggy: bool,
    location: usize,
    contents: Option<Box<Actor>>,
}

impl Tile {
    /// Creates a new tile
    pub fn new(kind: TileType, location: usize, foggy: bool) -> Tile {
        Tile{ kind, location, contents: None, foggy }
    }
    /// Produces a new `Tile` with the kind changed
    pub fn set_kind(self, kind: TileType) -> Self {
        Tile{ kind, ..self }
    }

    /// Move this `Tile`'s contents to the provided `Tile`, destroying what was there
    pub fn move_to(self, tile: Tile) -> (Tile, Tile) {
        match self.contents {
            Some(mut actor) => {
                actor.set_location(tile.location);
                (   // TODO: any way to avoid cloning here?
                    Tile{ contents: None, ..self },
                    Tile{ contents: Some(actor), ..tile },
                )
            },
            _ => (self, tile)
        }
    }

    /// Destroys the `Tile`'s contents
    pub fn empty(&mut self) {
        self.contents = None;
    }

    /// Sets this `Tile`'s contents to the provided `Actor`. Destroys any existing contents
    pub fn fill(&mut self, mut actor: Box<Actor>) {
        actor.set_location(self.location);
        self.contents = Some(actor);
    }

    /// Determines what symbol should be displayed for this `Tile`, taking into account its contents
    pub fn symbol(&self) -> char {
        if self.foggy() {
            ' '
        } else {
            self.contents.as_ref().map_or(self.empty_symbol(), |ref c| c.symbol())
        }
    }

    /// Determines what symbol should be displayed for this `Tile` when it is empty
    pub fn empty_symbol(&self) -> char {
        self.kind.symbol()
    }

    /// Have this `Tile`'s contents process the given `Action`
    pub fn process(&self, action: Action) -> Box<Behavior> {
        self.contents.as_ref().map_or(Box::new(Perform(Action::Idle)), |ref c| c.react(action))
    }

    /// The contents of this `Tile`, if any
    pub fn contents(&self) -> &Option<Box<Actor>> {
        &self.contents
    }

    /// Whether this `Tile` is able to hold contents
    pub fn can_hold_contents(&self) -> bool {
        match self.kind {
            TileType::Floor | TileType::Hall | TileType::Door => true,
            _ => false,
        }
    }

    /// Reveals this `Tile` if playing in foggy mode.
    pub fn reveal(&mut self) {
        self.foggy = false;
    }

    /// The `TileType` of this `Tile`
    pub fn kind(&self) -> TileType {
        self.kind
    }

    /// Whether this `Tile` is currently foggy
    pub fn foggy(&self) -> bool {
        self.foggy
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.symbol())
    }
}
