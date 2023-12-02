use std::{
    cell::Cell,
    fmt::{Debug, Display, Formatter},
};

pub struct Iter<'a, I, S>
where
    I: Display,
    S: Iterator<Item = I>,
{
    item: Cell<Option<&'a mut S>>,
}

impl<'a, I, S> Iter<'a, I, S>
where
    I: Display,
    S: Iterator<Item = I>,
{
    pub fn new(item: &'a mut S) -> Self {
        Self {
            item: Cell::new(Some(item)),
        }
    }
}

impl<'a, I: Display, S> Display for Iter<'a, I, S>
where
    S: Iterator<Item = I>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let iter = self.item.replace(None).ok_or(Default::default())?;
        for item in iter {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_prints_items() {
        let items: [i32; 3] = [1, 2, 3];
        let mut iter = items.into_iter();
        let iter = Iter::new(&mut iter);
        assert_eq!(iter.to_string(), "1\n2\n3\n");
    }

    #[test]
    fn display_handles_empty_iter() {
        let items: [i32; 0] = [];
        let mut iter = items.into_iter();
        let iter = Iter::new(&mut iter);
        assert_eq!(iter.to_string(), "");
    }

    #[test]
    fn display_handles_none_iter() {
        let items: Option<i32> = None;
        let mut iter = items.into_iter();
        let iter = Iter::new(&mut iter);
        assert_eq!(iter.to_string(), "");
    }

    #[test]
    fn display_handles_one_item_iter() {
        let items: Option<i32> = Some(1);
        let mut iter = items.into_iter();
        let iter = Iter::new(&mut iter);
        assert_eq!(iter.to_string(), "1\n");
    }

    #[test]
    #[should_panic]
    fn panics_when_displaying_twice() {
        let items: [i32; 3] = [1, 2, 3];
        let mut iter = items.into_iter();
        let iter = Iter::new(&mut iter);
        assert_eq!(format!("{}{}", iter, iter), "");
    }
}
