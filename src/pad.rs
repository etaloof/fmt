use std::fmt::{Display, Formatter, Write};

use crate::{count::Counter, Align};

pub struct Pad<T: Display> {
    item: T,
    len: usize,
    fill_char: char,
    align: Align,
}

impl<T: Display> Pad<T> {
    pub fn new(item: T) -> Self {
        let len = Counter::count(&item);
        let len = (len % 5 + 1) * 5;
        Self {
            item,
            len,
            fill_char: ' ',
            align: Align::Left,
        }
    }

    pub fn align_left(mut self) -> Self {
        self.align = Align::Left;
        self
    }

    pub fn align_right(mut self) -> Self {
        self.align = Align::Right;
        self
    }
}

impl<T: Display> Display for Pad<T> {
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
