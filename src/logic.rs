use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
#[must_use = "formatting helpers do nothing unless you use their `Display` implementation, e.g. with `.to_string()` or `format!`"]
struct Empty;

impl Empty {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Display for Empty {
    #[inline]
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[must_use = "formatting helpers do nothing unless you use their `Display` implementation, e.g. with `.to_string()` or `format!`"]
pub struct Optional<D: Display>(Conditional<D, Empty>);

impl<D: Display> Optional<D> {
    pub fn new(cond: bool, value: D) -> Self {
        Self(Conditional::new(cond, value, Empty))
    }
}

impl<D> Display for Optional<D>
where
    D: Display,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[must_use = "formatting helpers do nothing unless you use their `Display` implementation, e.g. with `.to_string()` or `format!`"]
pub struct Conditional<T, F> {
    cond: bool,
    left: T,
    right: F,
}

impl<T, F> Conditional<T, F>
where
    T: Display,
    F: Display,
{
    #[inline]
    pub fn new(cond: bool, left: T, right: F) -> Self {
        Self { cond, left, right }
    }
}

impl<T, F> Display for Conditional<T, F>
where
    T: Display,
    F: Display,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.cond {
            self.left.fmt(f)
        } else {
            self.right.fmt(f)
        }
    }
}

/// Concatenates two `Display`s.
#[must_use = "formatting helpers do nothing unless you use their `Display` implementation, e.g. with `.to_string()` or `format!`"]
pub struct Concat<L, R>
where
    L: Display,
    R: Display,
{
    left: L,
    right: R,
}

impl<L, R> Concat<L, R>
where
    L: Display,
    R: Display,
{
    #[inline]
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L, R> Display for Concat<L, R>
where
    L: Display,
    R: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.left.fmt(f)?;
        self.right.fmt(f)?;

        Ok(())
    }
}

/// Concatenates three `Display`s.
#[must_use = "formatting helpers do nothing unless you use their `Display` implementation, e.g. with `.to_string()` or `format!`"]
pub struct Concat3<L, M, R>
where
    L: Display,
    M: Display,
    R: Display,
{
    left: L,
    middle: M,
    right: R,
}

impl<L, M, R> Concat3<L, M, R>
where
    L: Display,
    M: Display,
    R: Display,
{
    #[inline]
    pub fn new(left: L, middle: M, right: R) -> Self {
        Self {
            left,
            middle,
            right,
        }
    }
}

impl<L, M, R> Display for Concat3<L, M, R>
where
    L: Display,
    M: Display,
    R: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.left.fmt(f)?;
        self.middle.fmt(f)?;
        self.right.fmt(f)?;

        Ok(())
    }
}

/// Concatenates four `Display`s.
#[must_use = "formatting helpers do nothing unless you use their `Display` implementation, e.g. with `.to_string()` or `format!`"]
pub struct Concat4<L, M, R, S>
where
    L: Display,
    M: Display,
    R: Display,
    S: Display,
{
    outer_left: L,
    inner_left: M,
    inner_right: R,
    outer_right: S,
}

impl<L, M, R, S> Concat4<L, M, R, S>
where
    L: Display,
    M: Display,
    R: Display,
    S: Display,
{
    #[inline]
    pub fn new(outer_left: L, inner_left: M, inner_right: R, outer_right: S) -> Self {
        Self {
            outer_left,
            inner_left,
            inner_right,
            outer_right,
        }
    }
}

impl<L, M, R, S> Display for Concat4<L, M, R, S>
where
    L: Display,
    M: Display,
    R: Display,
    S: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.outer_left.fmt(f)?;
        self.inner_left.fmt(f)?;
        self.inner_right.fmt(f)?;
        self.outer_right.fmt(f)?;

        Ok(())
    }
}

/// Concatenates `N` `Display`s.
#[must_use = "formatting helpers do nothing unless you use their `Display` implementation, e.g. with `.to_string()` or `format!`"]
pub struct ConcatN<D, const N: usize>
where
    D: Display,
{
    inner: [D; N],
}

impl<D, const N: usize> ConcatN<D, N>
where
    D: Display,
{
    #[inline]
    pub fn new(inner: [D; N]) -> Self {
        Self { inner }
    }
}

impl<D, const N: usize> Display for ConcatN<D, N>
where
    D: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for inner in self.inner.iter() {
            inner.fmt(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bytes_empty() {
        let item = Empty::new();
        assert_eq!(item.to_string(), "");
    }

    #[test]
    fn test_count_bytes_empty_or_empty() {
        let a = Empty::new();
        let b = Empty::new();

        let item = Conditional::new(true, a, b);
        assert_eq!(item.to_string(), "");

        let item = Conditional::new(false, a, b);
        assert_eq!(item.to_string(), "");
    }

    #[test]
    fn test_count_bytes_empty_not() {
        let a = "not";
        let b = Empty::new();

        let item = Conditional::new(true, a, b);
        assert_eq!(item.to_string(), a);

        let item = Conditional::new(false, a, b);
        assert_eq!(item.to_string(), "");
    }

    #[test]
    fn optional() {
        let item = Optional::new(true, "optional");
        assert_eq!(item.to_string(), "optional");

        let item = Optional::new(false, "optional");
        assert_eq!(item.to_string(), "");
    }

    #[test]
    fn concat() {
        let item = Concat::new("", "");
        assert_eq!(item.to_string(), "");

        let item = Concat::new("a", "b");
        assert_eq!(item.to_string(), "ab");
    }

    #[test]
    fn concat3() {
        let item = Concat3::new("", "", "");
        assert_eq!(item.to_string(), "");

        let item = Concat3::new("a", "bc", "d");
        assert_eq!(item.to_string(), "abcd");
    }

    #[test]
    fn concat4() {
        let item = Concat4::new("", "", "", "");
        assert_eq!(item.to_string(), "");

        let item = Concat4::new("a", "b", "c", "d");
        assert_eq!(item.to_string(), "abcd");
    }

    #[test]
    fn concat_n() {
        let item: [&str; 0] = [];
        let item = ConcatN::new(item);
        assert_eq!(item.to_string(), "");

        let item: [&str; 1] = ["a"];
        let item = ConcatN::new(item);
        assert_eq!(item.to_string(), "a");

        let item: [&str; 2] = ["a", "b"];
        let item = ConcatN::new(item);
        assert_eq!(item.to_string(), "ab");

        let item: [&str; 3] = ["a", "b", "c"];
        let item = ConcatN::new(item);
        assert_eq!(item.to_string(), "abc");

        let item: [&str; 4] = ["a", "b", "c", "d"];
        let item = ConcatN::new(item);
        assert_eq!(item.to_string(), "abcd");

        let item: [&str; 5] = ["a", "b", "c", "d", "e"];
        let item = ConcatN::new(item);
        assert_eq!(item.to_string(), "abcde");
    }
}
