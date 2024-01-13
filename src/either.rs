use std::fmt::{Display, Formatter};

/// An enum that represents values of two different types.
///
/// # Examples
///
/// ```rust
/// # use fmt::either::Either;
///
/// let left: Either<_, String> = Either::new_left("Hello, World!");
///
/// match left {
///     Either::Left(value) => println!("The left value is: {}", value),
///     Either::Right(value) => unreachable!("There is no right value!"),
/// }
/// ```
#[derive(Debug)]
pub enum Either<L, R> {
    /// The left value.
    Left(L),
    /// The right value.
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Creates a new `Either` with a left value.
    pub fn new_left(a: L) -> Self {
        Either::Left(a)
    }

    /// Creates a new `Either` with a right value.
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
