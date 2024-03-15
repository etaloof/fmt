use std::{
    fmt::{Display, Formatter, LowerHex},
    mem::ManuallyDrop,
    ops::Deref,
};

use crate::{adhoc2::Disp, joined::DisplayIteratorJoined};

union HexIterInner<I, F> {
    i: ManuallyDrop<I>,
    f: ManuallyDrop<F>,
}

struct HexIter<I, F>
where
    F: Fn() -> I,
    I: Iterator,
    I::Item: LowerHex,
{
    inner: HexIterInner<I, F>,
    produce_iterator: fn(&HexIterInner<I, F>) -> I,
    destruct: fn(&mut HexIterInner<I, F>),
}

impl<I, F> HexIter<I, F>
where
    F: Fn() -> I,
    I: Iterator,
    I::Item: LowerHex,
{
    fn new_fn(closure: F) -> HexIter<I, F> {
        HexIter {
            inner: HexIterInner {
                f: ManuallyDrop::new(closure),
            },
            produce_iterator: |inner| (unsafe { &inner.f }.deref())(),
            destruct: |inner| unsafe { ManuallyDrop::drop(&mut inner.f) },
        }
    }

    fn new_copy<II>(iter: II) -> HexIter<I, fn() -> I>
    where
        II: IntoIterator<IntoIter = I>,
        I: Copy,
    {
        HexIter {
            inner: HexIterInner {
                i: ManuallyDrop::new(iter.into_iter()),
            },
            produce_iterator: |inner| *unsafe { &inner.i }.deref(),
            destruct: |inner| unsafe { ManuallyDrop::drop(&mut inner.i) },
        }
    }

    fn new_clone<II>(iter: II) -> HexIter<I, fn() -> I>
    where
        II: IntoIterator<IntoIter = I>,
        I: Clone,
    {
        HexIter {
            inner: HexIterInner {
                i: ManuallyDrop::new(iter.into_iter()),
            },
            produce_iterator: |inner| unsafe { &inner.i }.deref().clone(),
            destruct: |inner| unsafe { ManuallyDrop::drop(&mut inner.i) },
        }
    }

    fn get_iterator(&self) -> I {
        let f = self.produce_iterator;
        f(&self.inner)
    }

    fn destruct(&mut self) {
        let d = self.destruct;
        d(&mut self.inner)
    }
}

impl<I, F> Display for HexIter<I, F>
where
    F: Fn() -> I,
    I: Iterator,
    I::Item: LowerHex,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let iter = self.get_iterator().map(|x| Disp::new(x, LowerHex::fmt));
        let iter = DisplayIteratorJoined::new(iter, ", ");
        iter.fmt(f)
    }
}

impl<I, F> Drop for HexIter<I, F>
where
    F: Fn() -> I,
    I: Iterator,
    I::Item: LowerHex,
{
    fn drop(&mut self) {
        self.destruct()
    }
}

type MapFunc<I> = fn(<I as Iterator>::Item) -> Disp<<I as Iterator>::Item>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice() {
        let data: [i32; 3] = [0xa, 0xb, 0xc];
        let slice = HexIter::new_clone(&data);
        assert_eq!(slice.to_string(), "a, b, c");
    }

    #[test]
    fn test_slice2() {
        let data: [i32; 2] = [0xff, 0xff];
        let slice = HexIter::new_clone(&data);
        assert_eq!(slice.to_string(), "ff, ff");
    }

    #[test]
    fn test_empty_slice() {
        let data: [i32; 0] = [];
        let slice = HexIter::new_clone(&data);
        assert_eq!(slice.to_string(), "");
    }

    #[test]
    fn test_single_element_slice() {
        let data: [i32; 1] = [0xa];
        let slice = HexIter::new_clone(&data);
        assert_eq!(slice.to_string(), "a");
    }

    #[test]
    fn test_filtered_slice() {
        let slice = HexIter::new_fn(|| [0xa, 0xb, 0xc].iter().filter(|x| *x & 0x1 == 0x1));
        assert_eq!(slice.to_string(), "a, b");
    }

    #[test]
    fn test_filtered_vec() {
        #[derive(Copy, Clone)]
        struct CopyIter;

        impl Iterator for CopyIter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                Some(0)
            }
        }

        let slice = HexIter::new_copy(CopyIter);
        assert_eq!(slice.to_string(), "a, b");
    }
}
