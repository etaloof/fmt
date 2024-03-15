use std::fmt::{Display, Formatter, LowerHex};

use crate::{adhoc2::Disp, joined::DisplayIteratorJoined, producer::DisplayProducer};

trait IteratorProducer {
    /// The iterator produced by this IteratorProducer.
    type Iterator: Iterator;

    fn produce_iterator(&self) -> Self::Iterator;
}

impl<I: Iterator, F: Fn() -> I> IteratorProducer for F {
    type Iterator = I;

    fn produce_iterator(&self) -> Self::Iterator {
        self()
    }
}

impl<'a, T> IteratorProducer for &'a [T]
where
    T: LowerHex,
{
    type Iterator = std::slice::Iter<'a, T>;

    fn produce_iterator(&self) -> Self::Iterator {
        self.into_iter()
    }
}

type MapFunc<I> = fn(<I as Iterator>::Item) -> Disp<<I as Iterator>::Item>;

pub struct HexIter<J>
where
    J: IteratorProducer,
    <<J as IteratorProducer>::Iterator as IntoIterator>::Item: LowerHex,
{
    iter: J,
}

impl<J> HexIter<J>
where
    J: IteratorProducer,
    <<J as IteratorProducer>::Iterator as IntoIterator>::Item: LowerHex,
{
    pub fn new(iter: J) -> Self {
        Self { iter }
    }
}

impl<J> Display for HexIter<J>
where
    J: IteratorProducer,
    <<J as IteratorProducer>::Iterator as IntoIterator>::Item: LowerHex,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let map: MapFunc<J::Iterator> = |x| Disp::new(x, LowerHex::fmt);
        let iter = self.iter.produce_iterator().map(map);
        let iter = DisplayIteratorJoined::new(iter, ", ");
        iter.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::*;

    #[test]
    fn test_slice() {
        let slice = HexIter::new(&[0xa, 0xb, 0xc]);
        assert_eq!(slice.to_string(), "a, b, c");
    }

    #[test]
    fn test_slice2() {
        let slice = HexIter::new(&[0xff, 0xff]);
        assert_eq!(slice.to_string(), "ff, ff");
    }

    #[test]
    fn test_empty_slice() {
        let slice: HexIter<&[u32]> = HexIter::new(&[]);
        assert_eq!(slice.to_string(), "");
    }

    #[test]
    fn test_single_element_slice() {
        let slice = HexIter::new(&[0xa]);
        assert_eq!(slice.to_string(), "a");
    }

    #[test]
    fn test_filtered_slice() {
        let slice = HexIter::new([0xa, 0xb, 0xc].iter().filter(|x| *x & 0x1 == 0x1));
        assert_eq!(slice.to_string(), "a, b");
    }
}
