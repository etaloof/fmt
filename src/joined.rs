use std::{
    cell::Cell,
    fmt::{Display, Formatter},
};

use crate::utils::LastIterationIterator;

pub struct DisplayIteratorJoined<Iter, I, S>
where
    Iter: Iterator<Item = I>,
    I: Display,
    S: Display,
{
    iter: Cell<Option<Iter>>,
    separator: S,
}

impl<Iter, I, S> DisplayIteratorJoined<Iter, I, S>
where
    Iter: Iterator<Item = I>,
    I: Display,
    S: Display,
{
    pub fn new<II: IntoIterator<IntoIter = Iter>>(iter: II, separator: S) -> Self {
        Self {
            iter: Cell::new(Some(iter.into_iter())),
            separator,
        }
    }
}

impl<Iter, I, S> Display for DisplayIteratorJoined<Iter, I, S>
where
    Iter: Iterator<Item = I>,
    I: Display,
    S: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let iter = self.iter.replace(None).ok_or(Default::default())?;
        for (is_last_iteration, item) in LastIterationIterator::new(iter) {
            if is_last_iteration {
                write!(f, "{}", item)?;
            } else {
                write!(f, "{}{}", item, self.separator)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let iter = &[1, 2, 3, 4, 5];
        let separator = " ";

        assert_eq!(
            &DisplayIteratorJoined::new(iter, separator).to_string(),
            "1 2 3 4 5"
        );

        let separator = ", ";
        assert_eq!(
            &DisplayIteratorJoined::new(iter, separator).to_string(),
            "1, 2, 3, 4, 5"
        );
    }
}
