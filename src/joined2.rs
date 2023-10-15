use std::fmt::{Display, Formatter};

use crate::utils::LastIterationIterator;

pub struct DisplayIteratorJoined<Iter, I, S, F>
where
    Iter: Iterator<Item = I> + Clone,
    I: Display,
    S: Display,
    F: Display,
{
    iter: Iter,
    separator: S,
    final_separator: F,
}

impl<Iter, I, S, F> DisplayIteratorJoined<Iter, I, S, F>
where
    Iter: Iterator<Item = I> + Clone,
    I: Display,
    S: Display,
    F: Display,
{
    pub fn new<II: IntoIterator<IntoIter = Iter>>(
        iter: II,
        separator: S,
        final_separator: F,
    ) -> Self {
        Self {
            iter: iter.into_iter(),
            separator,
            final_separator,
        }
    }

    pub fn iter(&self) -> Iter {
        self.iter.clone()
    }
}

impl<Iter, I, S, F> Display for DisplayIteratorJoined<Iter, I, S, F>
where
    Iter: Iterator<Item = I> + Clone,
    I: Display,
    S: Display,
    F: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (is_last_iteration, item) in LastIterationIterator::new(self.iter()) {
            if is_last_iteration {
                write!(f, "{}{}", item, self.final_separator)?;
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
        let final_separator = "";

        assert_eq!(
            &DisplayIteratorJoined::new(iter, separator, final_separator).to_string(),
            "1 2 3 4 5"
        );

        let separator = ", ";
        let final_separator = "";
        assert_eq!(
            &DisplayIteratorJoined::new(iter, separator, final_separator).to_string(),
            "1, 2, 3, 4, 5"
        );
    }
}
