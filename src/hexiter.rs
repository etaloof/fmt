use std::{
    cell::Cell,
    fmt::{Display, Formatter, LowerHex},
    iter::Map,
};

use crate::{adhoc2::Disp, joined::DisplayIteratorJoined};

type MapFunc<I> = fn(<I as Iterator>::Item) -> Disp<<I as Iterator>::Item>;

pub struct HexIter<I>
where
    I: Iterator,
    I::Item: LowerHex,
{
    item: Cell<Option<DisplayIteratorJoined<Map<I, MapFunc<I>>, &'static str>>>,
}

impl<I> HexIter<I>
where
    I: Iterator,
    I::Item: LowerHex,
{
    pub fn new<J>(iter: J) -> Self
    where
        J: IntoIterator<IntoIter = I>,
    {
        let map: MapFunc<I> = |x| Disp::new(x, LowerHex::fmt);
        let iter = iter.into_iter().map(map);
        let iter = DisplayIteratorJoined::new(iter, ", ");
        Self {
            item: Cell::new(Some(iter)),
        }
    }
}

impl<I> Display for HexIter<I>
where
    I: Iterator,
    I::Item: LowerHex,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let item = self.item.replace(None).ok_or(Default::default())?;
        item.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::slice::Iter;

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
        let slice: HexIter<Iter<i32>> = HexIter::new(&[]);
        assert_eq!(slice.to_string(), "");
    }

    #[test]
    fn test_single_element_slice() {
        let slice = HexIter::new(&[0xa]);
        assert_eq!(slice.to_string(), "a");
    }
}
