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
        let is_last_iter = self.iter.done();
        let value = self.iter.next();
        value.map(|v| (is_last_iter, v))
    }
}

impl<I> FusedIterator for LastIterationIterator<I> where I: Iterator {}
