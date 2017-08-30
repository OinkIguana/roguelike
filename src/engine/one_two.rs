use self::OneTwo::*;

pub enum OneTwo<T, U=T> {
    One(T),
    Two(T,U),
}

impl<T> OneTwo<T> {
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> OneTwo<U> {
        match self {
            One(a) => One(f(a)),
            Two(a, b) => Two(f(a), f(b)),
        }
    }

    pub fn as_vec(self) -> Vec<T> {
        match self {
            One(a) => vec![a],
            Two(a, b) => vec![a, b],
        }
    }
}
