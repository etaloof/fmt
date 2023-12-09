use std::{
    cell::Cell,
    fmt::{Display, Formatter, LowerHex},
    iter::Map,
};

use crate::{
    adhoc2::{Disp, FormatFunc},
    joined::DisplayIteratorJoined,
};

pub struct HexIter<I>
where
    I: Iterator,
    I::Item: LowerHex,
{
    item: Cell<
        Option<
            DisplayIteratorJoined<
                Map<I, fn(I::Item) -> Disp<I::Item>>,
                Disp<I::Item>,
                &'static str,
            >,
        >,
    >,
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
        type Item<I> = <I as Iterator>::Item;
        type MapFunc<I> = fn(Item<I>) -> Disp<Item<I>>;

        let separator = ", ";
        let iter = DisplayIteratorJoined::new(
            iter.into_iter().map(
                (|x| Disp::new(x, (|x, f| write!(f, "{:x}", x)) as FormatFunc<_>)) as MapFunc<I>,
            ),
            separator,
        );
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
        write!(f, "{}", item)
    }
}
