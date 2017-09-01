use engine::*;
use super::Gold;
use std::usize;

/// A `Thief` will wander the map to pick up any `Gold` that is on the ground. they may also
///
/// Symbol: `T`
///
/// Affinity: -3
///
/// Stats:
///
/// *   Health - 15
/// *   Attack - 2
/// *   Defense - 0
///
/// Drops:
///
/// *   (4 + stolen)x Gold
#[derive(Clone)]
pub struct Thief {
    health: i8,
    location: usize,
    messenger: Messenger,
    money: i32,
    has_been_attacked: bool,
}
impl Thief {
    pub fn new(messenger: Messenger) -> Thief {
        Thief{ health: 15, location: 0, money: 4, messenger, has_been_attacked: false }
    }
}

impl Actor for Thief {
    fn react(&self, _: Action) -> Box<Behavior> {
        let here = self.get_location();
        let towards_gold = FindAll(|tile| tile.contents().as_ref().map(|a| a.long_name() == Gold::id()).unwrap_or(false))
            .then(move |golds| golds.into_iter()
                .map(|loc| (QueryValue(loc), DistanceTo(here, loc))).collect::<Vec<_>>())
            .then(|golds|
                QueryValue(golds.into_iter()
                    .fold((0, usize::MAX), |prev, cur| if cur.1 < prev.1 { cur } else { prev })
                    .0))
            .then(move |gold|
                Direction::cardinals().into_iter()
                    .map(move |dir| NeighbourOf(here, dir).then(move |n| (QueryValue(n), DistanceTo(gold, n)))).collect::<Vec<_>>()
            )
            .then(|mut dists| {
                dists.sort_by(|a, b| a.1.cmp(&b.1));
                dists.into_iter()
                    .map(|(loc, _)| QueryValue(loc))
                    .collect::<Vec<_>>()
            })
            .then(move |goals| goals.into_iter().map(|goal| DirectionTo(here, goal)).collect::<Vec<_>>());
        Box::new(
            Switch(Direction::cardinals().into_iter().map(|dir| IfAttackable(dir, Perform(Action::Attack(dir)))).collect())
                .or_else(ExecQuery(towards_gold, |gold| Switch(gold.into_iter().map(|dir| IfEnterable(dir, Perform(Action::Move(dir)))).collect())))
        )
    }

    fn can_be_attacked(&self, other: &Actor) -> bool {
        other.affinity() >= 0
    }
    fn attack(&mut self, other: &mut Actor) {
        let amt = self.attack_power() as i32;
        self.set_money_rel(other.take_money(amt));
    }
    fn be_attacked(&mut self, other: &mut Actor) {
        self.health -= other.attack_power() as i8;
        if self.health <= 0 {
            self.messenger.send(Message::Die(self.get_location()));
            self.messenger.send(Message::Drop(self.get_location(), Box::new(Gold::new(self.money))));
        }
    }
    fn attack_power(&self) -> u32 { 2 }

    fn set_money_rel(&mut self, value: i32) {
        self.money += value;
    }
    fn symbol(&self) -> char { 'T' }
    fn affinity(&self) -> i8 { -3 }

    fn get_location(&self) -> usize { return self.location; }
    fn set_location(&mut self, location: usize) { self.location = location; }
}
