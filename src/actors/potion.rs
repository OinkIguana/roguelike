use engine::Actor;
use super::Player;

/// A `Potion` can be picked up by certain characters by stepping on it. Using a `Potion` consumes
/// it to restore 25 health.
///
/// Symbol: `P`
#[derive(Clone)]
pub struct Potion;

impl Actor for Potion {
    fn can_be_stepped_on(&self, other: &Actor) -> bool { other.long_name() == Player::id() }
    fn be_stepped_on(&mut self, other: &mut Actor) {
        other.pick_up(Box::new(self.clone()));
    }

    fn can_be_used(&self, _: &Actor) -> bool { true }
    fn be_used(&mut self, other: &mut Actor) -> Option<Box<Actor>> {
        other.set_health_rel(25);
        None
    }

    fn symbol(&self) -> char { 'P' }
    fn long_name(&self) -> &str { "Small Health Potion" }
}
