use std::fmt::{Display, Formatter};

use crate::utils::LastIterationIterator;

pub struct DisplayIteratorJoined<I, S, F>
where
    I: Iterator + Clone,
    I::Item: Display,
    S: Display,
    F: Display,
{
    iter: I,
    separator: S,
    final_separator: F,
}

impl<I, S, F> DisplayIteratorJoined<I, S, F>
where
    I: Iterator + Clone,
    I::Item: Display,
    S: Display,
    F: Display,
{
    pub fn new<J>(iter: J, separator: S, final_separator: F) -> Self
    where
        J: IntoIterator<IntoIter = I>,
    {
        Self {
            iter: iter.into_iter(),
            separator,
            final_separator,
        }
    }

    pub fn iter(&self) -> I {
        self.iter.clone()
    }
}

impl<I, S, F> Display for DisplayIteratorJoined<I, S, F>
where
    I: Iterator + Clone,
    I::Item: Display,
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
