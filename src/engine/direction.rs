use super::one_two::OneTwo;
use self::Direction::*;

/// Directions correspond to the cardinal directions and are used to indicate which side to move
/// to, attack, and interact with
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction { NW, N, NE, E, SE, S, SW, W, C }
impl Direction {
    /// A vector of the 4 cardinal directions, N, E, S, W
    pub fn cardinals() -> Vec<Direction> {
        vec![
            N,
            E,
            S,
            W,
        ]
    }
    /// A vector of all the the variants, starting from NW and cycling clockwise around the compass
    pub fn variants() -> Vec<Direction> {
        vec![
            NW,
            N,
            NE,
            E,
            SE,
            S,
            SW,
            W,
        ]
    }

    pub fn between((fx, fy): (usize, usize), (tx, ty): (usize, usize)) -> Self{
        match ((tx as i32 - fx as i32).signum(), (ty as i32 - fy as i32).signum()) {
            (-1, -1)    => NW,
            (0, -1)     => N,
            (1, -1)     => NE,
            (1, 0)      => E,
            (-1, 0)     => W,
            (-1, 1)     => SW,
            (0, 1)      => S,
            (1, 1)      => SE,
            (0, 0)      => C,
            _           => panic!(),
        }
    }

    pub fn split(self) -> OneTwo<Self> {
        use self::OneTwo::*;
        match self {
            C | N | W | S | E => One(self),
            NE => Two(N,E),
            NW => Two(N,W),
            SE => Two(S,E),
            SW => Two(S,W),
        }
    }
}
