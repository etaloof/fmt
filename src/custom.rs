use std::{
    cell::UnsafeCell,
    fmt::{Debug, Display, Formatter},
};

pub struct Iter<'a, I, S>
where
    I: Display,
    S: Iterator<Item = I>,
{
    item: UnsafeCell<Option<&'a mut S>>,
}

impl<'a, I, S> Iter<'a, I, S>
where
    I: Display,
    S: Iterator<Item = I>,
{
    pub fn new(item: &'a mut S) -> Self {
        Self {
            item: UnsafeCell::new(Some(item)),
        }
    }
}

impl<'a, I: Display, S> Display for Iter<'a, I, S>
where
    S: Iterator<Item = I>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let iter = std::mem::take(unsafe { &mut *self.item.get() });
        for item in iter.unwrap() {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

impl<'a, I: Display, S> Debug for Iter<'a, I, S>
where
    S: Iterator<Item = I>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
