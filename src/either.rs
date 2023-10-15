use std::fmt::{Display, Formatter};

pub enum Either<L: Display, R: Display> {
    Left(L),
    Right(R),
}

impl<L: Display, R: Display> Either<L, R> {
    pub fn new_left(a: L) -> Self {
        Either::Left(a)
    }

    pub fn new_right(b: R) -> Self {
        Either::Right(b)
    }
}

impl<L: Display, R: Display> Display for Either<L, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Either::Left(left) => left.fmt(f),
            Either::Right(right) => right.fmt(f),
        }
    }
}
