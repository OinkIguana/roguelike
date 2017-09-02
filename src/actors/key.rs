use engine::*;
use super::Player;

/// A `Key` is used to open a chest. Each `Key` is single use.
///
/// Symbol: `K`
#[derive(Clone)]
pub struct Key;
impl Key {
    pub fn id() -> &'static str { "Chest Key" }
}

impl Actor for Key {
    fn can_be_stepped_on(&self, other: &Actor) -> bool { other.long_name() == Player::id() }
    fn be_stepped_on(&mut self, other: &mut Actor) {
        other.pick_up(Box::new(self.clone()));
    }

    fn symbol(&self) -> char { 'K' }
    fn long_name(&self) -> &str { Key::id() }
}
