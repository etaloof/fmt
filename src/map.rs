use std::{
    cell::Cell,
    fmt::{Debug, Display, Formatter},
};

pub struct Map<I, F, S>
where
    F: FnMut(S::Item) -> I,
    S: Iterator,
    I: Display,
{
    item: Cell<Option<S>>,
    f: Cell<Option<F>>,
}

impl<I, F, S> Map<I, F, S>
where
    F: FnMut(S::Item) -> I,
    S: Iterator,
    I: Display,
{
    pub fn new(item: S, f: F) -> Self {
        Self {
            item: Cell::new(Some(item)),
            f: Cell::new(Some(f)),
        }
    }
}

impl<I, F, S> Display for Map<I, F, S>
where
    F: FnMut(S::Item) -> I,
    S: Iterator,
    I: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let iter = self.item.replace(None).ok_or(Default::default())?;
        let func = self.f.replace(None).ok_or(Default::default())?;
        for item in iter.map(func) {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

impl<I, F, S> Debug for Map<I, F, S>
where
    F: FnMut(S::Item) -> I,
    S: Iterator,
    I: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::identity;

    use super::*;

    #[test]
    fn display_prints_items() {
        let items: [i32; 3] = [1, 2, 3];
        let iter = items.into_iter();
        let iter = Map::new(iter, identity);
        assert_eq!(iter.to_string(), "1\n2\n3\n");
    }

    #[test]
    fn display_handles_empty_iter() {
        let items: [i32; 0] = [];
        let iter = items.into_iter();
        let iter = Map::new(iter, identity);
        assert_eq!(iter.to_string(), "");
    }

    #[test]
    fn display_handles_none_iter() {
        let items: Option<i32> = None;
        let iter = items.into_iter();
        let iter = Map::new(iter, identity);
        assert_eq!(iter.to_string(), "");
    }

    #[test]
    fn display_handles_one_item_iter() {
        let items: Option<i32> = Some(1);
        let iter = items.into_iter();
        let iter = Map::new(iter, identity);
        assert_eq!(iter.to_string(), "1\n");
    }

    #[test]
    #[should_panic]
    fn panics_when_displaying_twice() {
        let items: [i32; 3] = [1, 2, 3];
        let iter = items.into_iter();
        let iter = Map::new(iter, identity);
        assert_eq!(format!("{}{}", iter, iter), "");
    }
}
