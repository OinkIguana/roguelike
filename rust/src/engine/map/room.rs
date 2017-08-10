use super::tile::Tile;

/// A Room groups tiles into chunks that should be considered as a whole for visibility purposes
#[allow(dead_code)]
pub struct Room<'a> {
    tiles: Vec<&'a Tile<'a>>,
}
