use std::iter::{Fuse, FusedIterator, Peekable};

/// An iterator which knows when it has reached its final element.
pub struct FiniteIterator<I>
where
    I: Iterator,
{
    iter: Peekable<Fuse<I>>,
}

impl<I> FiniteIterator<I>
where
    I: Iterator,
{
    /// Creates a new `FiniteIterator` from the given iterator.
    pub fn new(iter: I) -> Self {
        let iter = iter.fuse().peekable();
        Self { iter }
    }

    /// Returns `true` if the iterator is exhausted, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fmt::utils::FiniteIterator;
    /// let mut v = vec![1, 2, 3];
    /// let mut iter = FiniteIterator::new(v.iter());
    ///
    /// assert!(!iter.done());
    /// assert_eq!(iter.next(), Some(&1));
    /// assert!(!iter.done());
    /// assert_eq!(iter.next(), Some(&2));
    /// assert!(!iter.done());
    /// assert_eq!(iter.next(), Some(&3));
    /// assert!(iter.done());
    /// assert_eq!(iter.next(), None);
    /// assert!(iter.done());
    /// ```
    pub fn done(&mut self) -> bool {
        self.iter.peek().is_none()
    }
}

impl<I> Iterator for FiniteIterator<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<I> FusedIterator for FiniteIterator<I> where I: Iterator {}

pub struct LastIterationIterator<I>
where
    I: Iterator,
{
    iter: FiniteIterator<I>,
}

/// An iterator adapter which turns an `Iterator<Item=T>` in an iterator
/// over tuples of `(is_last_element, T)`.
/// `is_last_element` is `false` until the last element of the iterator is reached
/// and `true` for the last element.
impl<I> LastIterationIterator<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        let iter = FiniteIterator::new(iter);
        Self { iter }
    }
}

impl<I> Iterator for LastIterationIterator<I>
where
    I: Iterator,
{
    type Item = (bool, I::Item);

    /// Returns the next element of the iterator, or `None` if the iterator is
    /// exhausted.
    ///
    /// This function returns `Some((is_last_iteration, element))`,
    /// where `is_last_iteration`
    /// is a boolean indicating whether this is the last iteration of the iterator.
    /// If this is the last iteration, then `element` is the last element of the iterator
    /// and `is_last_iteration` is `true`.
    /// Subsequent calls to `next` will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fmt::utils::LastIterationIterator;
    /// let mut v = vec![1, 2, 3];
    /// let mut iter = LastIterationIterator::new(v.iter());
    ///
    /// assert_eq!(iter.next(), Some((false, &1)));
    /// assert_eq!(iter.next(), Some((false, &2)));
    /// assert_eq!(iter.next(), Some((true, &3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.iter.next();
        let is_last_iter = self.iter.done();
        value.map(|v| (is_last_iter, v))
    }
}

impl<I> FusedIterator for LastIterationIterator<I> where I: Iterator {}

/// An iterator adapter similar to [std::iter::Map](std::iter::Map) which applies a
/// function `F` to all but the final element of an iterator.
/// The final element is mapped using the function `G`.
pub struct MapLast<I, F, G>
where
    I: Iterator,
{
    iter: FiniteIterator<I>,
    f: F,
    g: G,
}

impl<B, I, F, G> MapLast<I, F, G>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
    G: FnMut(I::Item) -> B,
{
    /// Creates a new `MapLast` iterator.
    pub fn new(iter: I, map: F, map_last: G) -> Self {
        let iter = FiniteIterator::new(iter);
        Self {
            iter,
            f: map,
            g: map_last,
        }
    }
}

impl<B, I: Iterator, F, G> Iterator for MapLast<I, F, G>
where
    F: FnMut(I::Item) -> B,
    G: FnMut(I::Item) -> B,
{
    type Item = B;

    /// Returns the next element of the iterator, or `None` if the iterator is
    /// exhausted.
    ///
    /// The element returned is the result of applying `f` to all but the final
    /// element of the iterator, and then applying `g` to the final element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fmt::utils::MapLast;
    /// let mut v = vec![1, 2, 3];
    /// let mut iter = MapLast::new(v.iter(), |x| x * 2, |x| x * 3);
    ///
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(4));    
    /// assert_eq!(iter.next(), Some(9));
    /// ```
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.iter.next();
        let is_last_iter = self.iter.done();
        if is_last_iter {
            value.map(&mut self.g)
        } else {
            value.map(&mut self.f)
        }
    }
}

impl<B, I, F, G> FusedIterator for MapLast<I, F, G>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
    G: FnMut(I::Item) -> B,
{
}

#[cfg(test)]
mod tests {
    use crate::utils::{FiniteIterator, LastIterationIterator, MapLast};

    #[test]
    fn test_finite_iterator() {
        let iter = &[1, 2, 3, 4, 5];
        let mut iter = FiniteIterator::new(iter.iter());

        assert!(!iter.done());
        assert_eq!(iter.next(), Some(&1));
        assert!(!iter.done());
        assert_eq!(iter.next(), Some(&2));
        assert!(!iter.done());
        assert_eq!(iter.next(), Some(&3));
        assert!(!iter.done());
        assert_eq!(iter.next(), Some(&4));
        assert!(!iter.done());
        assert_eq!(iter.next(), Some(&5));
        assert!(iter.done());
        assert_eq!(iter.next(), None);
        assert!(iter.done());
    }

    #[test]
    fn test_last_iteration_iterator() {
        let iter = &[1, 2, 3, 4, 5];
        let mut iter = LastIterationIterator::new(iter.iter());

        assert_eq!(iter.next(), Some((false, &1)));
        assert_eq!(iter.next(), Some((false, &2)));
        assert_eq!(iter.next(), Some((false, &3)));
        assert_eq!(iter.next(), Some((false, &4)));
        assert_eq!(iter.next(), Some((true, &5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_last_map_iterator() {
        let iter = &[1, 2, 3, 4, 5];

        let identity = |x| x;
        let double = |x| x * 2;
        let zero = |_| 0;

        let actual = MapLast::new(iter.iter().copied(), identity, identity);
        assert_eq!(&actual.collect::<Vec<_>>(), &[1, 2, 3, 4, 5]);

        let actual = MapLast::new(iter.iter().copied(), double, double);
        assert_eq!(&actual.collect::<Vec<_>>(), &[2, 4, 6, 8, 10]);

        let actual = MapLast::new(iter.iter().copied(), double, zero);
        assert_eq!(&actual.collect::<Vec<_>>(), &[2, 4, 6, 8, 0]);

        let actual = MapLast::new(iter.iter().copied(), zero, double);
        assert_eq!(&actual.collect::<Vec<_>>(), &[0, 0, 0, 0, 10]);

        let actual = MapLast::new(iter.iter().copied(), zero, zero);
        assert_eq!(&actual.collect::<Vec<_>>(), &[0, 0, 0, 0, 0]);
    }
}
