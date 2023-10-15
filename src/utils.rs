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

#[cfg(test)]
mod tests {
    use crate::utils::{FiniteIterator, LastIterationIterator};

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
}
