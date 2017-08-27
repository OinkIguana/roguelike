use engine::Actor;

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
