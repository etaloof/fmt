use std::fmt::{Debug, Display, Formatter, LowerHex};

use crate::utils::LastIterationIterator;

pub struct HexSlice<'a, I: LowerHex> {
    item: &'a [I],
}

impl<'a, I> HexSlice<'a, I>
where
    I: LowerHex,
{
    pub fn new(item: &'a [I]) -> Self {
        Self { item }
    }
}

impl<'a, I: LowerHex> Display for HexSlice<'a, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for (is_last_iteration, item) in LastIterationIterator::new(self.item.iter()) {
            if is_last_iteration {
                write!(f, "{:x}", item)?;
            } else {
                write!(f, "{:x}, ", item)?;
            }
        }
        f.write_str("]")
    }
}

impl<'a, I: LowerHex> Debug for HexSlice<'a, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
