use std::{
    cell::Cell,
    fmt::{Display, Formatter},
};

use crate::utils::LastIterationIterator;

pub struct DisplayIteratorJoined<I, S>
where
    I: Iterator,
    I::Item: Display,
    S: Display,
{
    iter: Cell<Option<I>>,
    separator: S,
}

impl<I, S> DisplayIteratorJoined<I, S>
where
    I: Iterator,
    I::Item: Display,
    S: Display,
{
    pub fn new<J>(iter: J, separator: S) -> Self
    where
        J: IntoIterator<IntoIter = I>,
    {
        Self {
            iter: Cell::new(Some(iter.into_iter())),
            separator,
        }
    }
}

impl<I, S> Display for DisplayIteratorJoined<I, S>
where
    I: Iterator,
    I::Item: Display,
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
