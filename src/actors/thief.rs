use engine::*;
// use rand::{Rng,thread_rng};
use super::{Gold,Stairs};
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
        let gold_locations = FindAll(|tile| tile.contents().as_ref().map(|a| a.long_name() == Gold::id()).unwrap_or(false));
        let messenger = self.messenger.clone();
        Box::new(
            // Attack whoever is nearby and take their money if they can be attacked
            Switch(Direction::cardinals().into_iter().map(|dir| IfAttackable(dir, Perform(Action::Attack(dir)))).collect())
                .or_else(
                    IfQuery(gold_locations, |ref gold| gold.len() > 0,
                        move |gold| {
                            // Try to move towards Gold if they exist
                            let towards_gold = gold.into_iter().map(|loc| (QueryValue(loc), DistanceTo(here, loc))).collect::<Vec<_>>()
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
                            ExecQuery(towards_gold, |gold| {
                                Switch(gold.into_iter().map(|dir| IfEnterable(dir, Perform(Action::Move(dir)))).collect())
                            })
                        },
                        // Move towards the Stairs and escape!
                        move |_| {
                            let toward_stairs =
                                Find(|tile| tile.contents().as_ref().map(|a| a.long_name() == Stairs::id()).unwrap_or(false))
                                    .then(move |stairs| (DirectionTo(here, stairs), DistanceTo(here, stairs)));
                            let m = messenger.clone();
                            IfQuery(toward_stairs, |&(_, dist)| dist == 1,
                                move |_| {
                                    m.send(Message::Die(here));
                                    Perform(Action::Idle)
                                },
                                |(dir, _)| Switch(dir.split().as_vec().into_iter().map(|dir| IfOpen(dir, Perform(Action::Move(dir)))).collect())
                            )
                        }
                    )
                )
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
