use engine::{Actor};

#[derive(Clone)]
pub struct Gold{ value: i32 }
impl Gold {
    pub fn new(value: i32) -> Gold {
        Gold{ value }
    }
}

impl Actor for Gold {
    fn can_be_stepped_on(&self) -> bool { true }
    fn be_stepped_on(&mut self, other: &mut Actor) {
        other.gain_money(self.value);
    }
    fn symbol(&self) -> char { 'G' }
}
