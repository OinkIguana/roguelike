use engine::Actor;

/// `Gold` is currency, which appears on the ground and is dropped by enemies. It can be picked up
/// by some characters by stepping on it.
///
/// Symbol: `G`
#[derive(Clone)]
pub struct Gold{ value: i32 }
impl Gold {
    pub fn new(value: i32) -> Gold {
        Gold{ value }
    }

    pub fn id() -> &'static str { "Gold" }
}

impl Actor for Gold {
    fn can_be_stepped_on(&self, _: &Actor) -> bool { true }
    fn be_stepped_on(&mut self, other: &mut Actor) {
        other.set_money_rel(self.value);
    }
    fn symbol(&self) -> char { 'G' }

    fn long_name(&self) -> &str { Gold::id() }
}
