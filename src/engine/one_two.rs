use self::OneTwo::*;

/// A utility providing partial monadic use of one- and two-tuples containing only one type.
pub enum OneTwo<T> {
    One(T),
    Two(T,T),
}

impl<T> OneTwo<T> {
    /// Maps each item of the tuple to a function
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> OneTwo<U> {
        match self {
            One(a) => One(f(a)),
            Two(a, b) => Two(f(a), f(b)),
        }
    }

    /// Converts the tuple to a vector
    pub fn as_vec(self) -> Vec<T> {
        match self {
            One(a) => vec![a],
            Two(a, b) => vec![a, b],
        }
    }
}
