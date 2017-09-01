use rand::{thread_rng,Rng};
use engine::*;
use super::Gold;

/// A `Goblin` is the most standard enemy. It will move around randomly until the `Player` passes
/// within 5 spaces, then it will chase the `Player` until they have moved out of range again. A
/// `Goblin` cannot pass through doors.
///
/// Symbol: `N`
///
/// Affinity: -4
///
/// Stats:
///
/// *   Health - 25
/// *   Attack - 4
/// *   Defense - 0
///
/// Drops:
///
/// *   3x Gold
#[derive(Clone)]
pub struct Goblin {
    health: i8,
    location: usize,
    messenger: Messenger,
}
impl Goblin {
    pub fn new(messenger: Messenger) -> Goblin {
        Goblin{ health: 25, location: 0, messenger }
    }
}

impl Actor for Goblin {
    fn react(&self, _: Action) -> Box<Behavior> {
        let here = self.get_location();
        let direction = Find(|tile| tile.contents().as_ref().map(|a| a.symbol() == '@').unwrap_or(false)).then(move |i| DirectionTo(here, i));
        let distance = Find(|tile| tile.contents().as_ref().map(|a| a.symbol() == '@').unwrap_or(false)).then(move |i| DistanceTo(here, i));
        Box::new(ExecQuery((distance, direction), |(dist, dir)| {
            let (attacks, moves) = if dist < 5 {
                let mut dirs = dir.split().as_vec();
                thread_rng().shuffle(&mut dirs);
                let attacks: Vec<IfAttackable<Perform>> = dirs.iter().cloned().map(|d| IfAttackable(d, Perform(Action::Attack(d)))).collect();
                let moves: Vec<IfOpen<Perform>> = dirs.iter().cloned().map(|d| IfOpen(d, Perform(Action::Move(d)))).collect();
                (attacks, moves)
            } else {
                let mut moves: Vec<IfOpen<Perform>> = Direction::cardinals().iter().cloned().map(|d| IfOpen(d, Perform(Action::Move(d)))).collect();
                thread_rng().shuffle(&mut moves);
                (vec![], moves)
            };
            Switch(attacks).or_else(Switch(moves))
        }))
    }

    fn can_be_attacked(&self, other: &Actor) -> bool {
        other.affinity() >= 0
    }
    fn be_attacked(&mut self, other: &mut Actor) {
        self.health -= other.attack_power() as i8;
        if self.health <= 0 {
            self.messenger.send(Message::Die(self.get_location()));
            self.messenger.send(Message::Drop(self.get_location(), Box::new(Gold::new(3))));
        }
    }
    fn attack_power(&self) -> u32 { 4 }

    fn symbol(&self) -> char { 'N' }
    fn affinity(&self) -> i8 { -4 }

    fn can_enter(&self, tile: TileType) -> bool {
        use self::TileType::*;
        match tile {
            Wall | Empty | Door => false,
            _ => true,
        }
    }

    fn get_location(&self) -> usize { return self.location; }
    fn set_location(&mut self, location: usize) { self.location = location; }
}
