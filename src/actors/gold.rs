use engine::{Actor};

#[derive(Clone)]
pub struct Gold{ value: u32 }
impl Gold {
    pub fn new(value: u32) -> Gold {
        Gold{ value }
    }
}

impl Actor for Gold {
    fn can_be_stepped_on(&self) -> bool { true }
    fn be_stepped_on(&mut self, other: &mut Actor) {
        other.gain_money(self.value as i32);
    }
    fn symbol(&self) -> char { 'G' }
}
