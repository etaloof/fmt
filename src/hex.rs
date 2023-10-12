use std::fmt::{Display, Formatter, LowerHex};

pub struct Hex<I: LowerHex> {
    item: I,
}

impl<I: LowerHex> Hex<I>
where
    I: LowerHex,
{
    pub fn new(item: I) -> Self {
        Self { item }
    }
}

impl<I: LowerHex> Display for Hex<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.item.fmt(f)
    }
}
