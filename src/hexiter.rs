use std::fmt::{Display, Formatter, LowerHex, Write};

use crate::{count::Counter, Align};

pub struct HexIter<I: Iterator<Item = H>, H: LowerHex> {
    item: I,
}

impl<I, H> HexIter<I, H>
where
    I: Iterator<Item = H>,
    H: LowerHex,
{
    pub fn new<J>(item: J) -> Self
    where
        J: IntoIterator<Item = H>,
        J::IntoIter = I,
    {
        Self { item }
    }
}

impl<I: Iterator<Item = H>, H: LowerHex> Display for HexIter<I, H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = format!("{}", self.item);
        match self.align {
            Align::Right => {
                for _ in string.len()..self.len {
                    f.write_char(self.fill_char)?;
                }
                f.write_str(&string)?;
            },
            Align::Left => {
                f.write_str(&string)?;
                for _ in string.len()..self.len {
                    f.write_char(self.fill_char)?;
                }
            },
        }
        f.write_str("")
    }
}
