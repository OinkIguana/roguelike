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
                .map(|i| (QueryValue(i), DistanceTo(here, i))).collect::<Vec<_>>())
            .then(|golds|
                QueryValue(golds.into_iter()
                    .fold((0 as usize, usize::MAX), |prev, cur| if cur.1 < prev.1 { cur } else { prev })))
            .then(move |gold| (
                QueryValue(gold),
                Direction::cardinals().into_iter()
                    .map(|dir| NeighbourOf(gold.0, dir).then(move |n| (QueryValue(n), DistanceTo(here, n)))).collect::<Vec<_>>()
            ))
            .then(|(gold, mut dists)| {
                dists.sort_by(|&(_, a), &(_, b)| (a as i32 - gold.1 as i32).cmp(&(b as i32 - gold.1 as i32)));
                dists.iter()
                    .map(|loc| QueryValue(loc.0))
                    .collect::<Vec<_>>()
            })
            .then(move |goals| goals.into_iter().map(|goal| DirectionTo(here, goal)).collect::<Vec<_>>());
        Box::new(ExecQuery(towards_gold, |dirs| {
            Switch(dirs.into_iter().map(|dir| IfEnterable(dir, Perform(Action::Move(dir)))).collect())
        }))
    }

    fn can_be_attacked(&self, other: &Actor) -> bool {
        other.affinity() >= 0
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
