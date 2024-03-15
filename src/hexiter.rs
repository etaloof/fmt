use std::fmt::{Display, Formatter, LowerHex};

use crate::{adhoc2::Disp, joined::DisplayIteratorJoined, producer::DisplayProducer};

pub trait IntoRefIterator {
    type Item;

    type IntoIter: Iterator<Item = Self::Item>;

    fn ref_iter(&self) -> Self::IntoIter;
}

impl<'a, I, T> IntoRefIterator for &'a I
where
    &'a I: IntoIterator<Item = &'a T>,
    I: ?Sized,
    T: 'a,
{
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = <&'a I as IntoIterator>::IntoIter;

    fn ref_iter(&self) -> Self::IntoIter {
        self.into_iter()
    }
}

struct HexIter<I>
where
    I: IntoRefIterator,
    <I as IntoRefIterator>::Item: LowerHex,
{
    iter: I,
}

impl<I> HexIter<I>
where
    I: IntoRefIterator,
    <I as IntoRefIterator>::Item: LowerHex,
{
    fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Display for HexIter<I>
where
    I: IntoRefIterator,
    <I as IntoRefIterator>::Item: LowerHex,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let map: MapFunc<I::IntoIter> = |x| Disp::new(x, LowerHex::fmt);
        let iter = self.iter.ref_iter().map(map);
        let iter = DisplayIteratorJoined::new(iter, ", ");
        iter.fmt(f)
    }
}

type MapFunc<I> = fn(<I as Iterator>::Item) -> Disp<<I as Iterator>::Item>;

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
