use std::{
    cell::Cell,
    fmt::{Debug, Display, Formatter},
};

pub struct OnceRefIter<'a, S>
where
    S: Iterator,
    S::Item: Display,
{
    item: Cell<Option<&'a mut S>>,
}

impl<'a, S> OnceRefIter<'a, S>
where
    S: Iterator,
    S::Item: Display,
{
    pub fn new(item: &'a mut S) -> Self {
        Self {
            item: Cell::new(Some(item)),
        }
    }
}

impl<'a, S> Display for OnceRefIter<'a, S>
where
    S: Iterator,
    S::Item: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let iter = self.item.replace(None).ok_or(Default::default())?;
        for item in iter {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

impl<'a, S> Debug for OnceRefIter<'a, S>
where
    S: Iterator,
    S::Item: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub struct OnceIter<S>
where
    S: Iterator,
    S::Item: Display,
{
    item: Cell<Option<S>>,
}

impl<S> OnceIter<S>
where
    S: Iterator,
    S::Item: Display,
{
    pub fn new(item: S) -> Self {
        Self {
            item: Cell::new(Some(item)),
        }
    }
}

impl<S> Display for OnceIter<S>
where
    S: Iterator,
    S::Item: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let iter = self.item.replace(None).ok_or(Default::default())?;
        for item in iter {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

impl<S> Debug for OnceIter<S>
where
    S: Iterator,
    S::Item: Display,
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
        let iter = OnceRefIter::new(&mut iter);
        assert_eq!(iter.to_string(), "1\n2\n3\n");
    }

    #[test]
    fn display_handles_empty_iter() {
        let items: [i32; 0] = [];
        let mut iter = items.into_iter();
        let iter = OnceRefIter::new(&mut iter);
        assert_eq!(iter.to_string(), "");
    }

    #[test]
    fn display_handles_none_iter() {
        let items: Option<i32> = None;
        let mut iter = items.into_iter();
        let iter = OnceRefIter::new(&mut iter);
        assert_eq!(iter.to_string(), "");
    }

    #[test]
    fn display_handles_one_item_iter() {
        let items: Option<i32> = Some(1);
        let mut iter = items.into_iter();
        let iter = OnceRefIter::new(&mut iter);
        assert_eq!(iter.to_string(), "1\n");
    }

    #[test]
    #[should_panic]
    fn panics_when_displaying_twice() {
        let items: [i32; 3] = [1, 2, 3];
        let mut iter = items.into_iter();
        let iter = OnceRefIter::new(&mut iter);
        assert_eq!(format!("{}{}", iter, iter), "");
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn display_prints_items() {
        let items: [i32; 3] = [1, 2, 3];
        let iter = items.into_iter();
        let iter = OnceIter::new(iter);
        assert_eq!(iter.to_string(), "1\n2\n3\n");
    }

    #[test]
    fn display_handles_empty_iter() {
        let items: [i32; 0] = [];
        let iter = items.into_iter();
        let iter = OnceIter::new(iter);
        assert_eq!(iter.to_string(), "");
    }

    #[test]
    fn display_handles_none_iter() {
        let items: Option<i32> = None;
        let iter = items.into_iter();
        let iter = OnceIter::new(iter);
        assert_eq!(iter.to_string(), "");
    }

    #[test]
    fn display_handles_one_item_iter() {
        let items: Option<i32> = Some(1);
        let iter = items.into_iter();
        let iter = OnceIter::new(iter);
        assert_eq!(iter.to_string(), "1\n");
    }

    #[test]
    #[should_panic]
    fn panics_when_displaying_twice() {
        let items: [i32; 3] = [1, 2, 3];
        let iter = items.into_iter();
        let iter = OnceIter::new(iter);
        assert_eq!(format!("{}{}", iter, iter), "");
    }
}
