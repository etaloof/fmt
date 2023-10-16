use std::iter::{Fuse, FusedIterator, Peekable};

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
    fn new(iter: I) -> Self {
        let iter = iter.fuse().peekable();
        Self { iter }
    }

    fn done(&mut self) -> bool {
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

impl<I> LastIterationIterator<I>
where
    I: Iterator,
{
    pub(crate) fn new(iter: I) -> Self {
        let iter = FiniteIterator::new(iter);
        Self { iter }
    }
}

impl<I> Iterator for LastIterationIterator<I>
where
    I: Iterator,
{
    type Item = (bool, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.iter.next();
        let is_last_iter = self.iter.done();
        value.map(|v| (is_last_iter, v))
    }
}

impl<I> FusedIterator for LastIterationIterator<I> where I: Iterator {}

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
    pub(crate) fn new(iter: I, map: F, map_last: G) -> Self {
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
