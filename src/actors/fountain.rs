use engine::Actor;

/// A Fountain is a structure which the Player can interact with once to restore 25 health.
///
/// Symbol: `F` unused, `f` used
#[derive(Clone)]
pub struct Fountain {
    used: bool
}

impl Fountain {
    pub fn new() -> Fountain {
        Fountain{ used: false }
    }
}

impl Actor for Fountain {
    fn can_be_interacted_with(&self, _: &Actor) -> bool { !self.used }
    fn be_interacted_with(&mut self, other: &mut Actor) {
        self.used = true;
        other.set_health_rel(25);
    }

    fn symbol(&self) -> char { if self.used { 'f' } else { 'F' } }
}
