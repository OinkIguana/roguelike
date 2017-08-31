use engine::*;

#[derive(Clone)]
pub struct Key;
impl Key {
    pub fn id() -> &'static str { "Chest Key" }
}

impl Actor for Key {
    fn can_be_stepped_on(&self, _: &Actor) -> bool { true }
    fn be_stepped_on(&mut self, other: &mut Actor) {
        other.pick_up(Box::new(self.clone()));
    }

    fn symbol(&self) -> char { 'K' }
    fn long_name(&self) -> &str { Key::id() }
}
