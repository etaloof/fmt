use std::fmt::{Debug, Display, Formatter};

use crate::utils::LastIterationIterator;

pub struct Slice<'a, I: Display> {
    item: &'a [I],
}

impl<'a, I> Slice<'a, I>
where
    I: Display,
{
    pub fn new(item: &'a [I]) -> Self {
        Self { item }
    }
}

impl<'a, I: Display> Display for Slice<'a, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for (is_last_iteration, item) in LastIterationIterator::new(self.item.iter()) {
            if is_last_iteration {
                write!(f, "{}", item)?;
            } else {
                write!(f, "{}, ", item)?;
            }
        }
        f.write_str("]")
    }
}

impl<'a, I: Display> Debug for Slice<'a, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice() {
        let slice = Slice::new(&["a", "b", "c"]);
        assert_eq!(slice.to_string(), "[a, b, c]");
    }

    #[test]
    fn test_slice_with_numbers() {
        let slice = Slice::new(&[1, 2, 3]);
        assert_eq!(slice.to_string(), "[1, 2, 3]");
    }

    #[test]
    fn test_empty_slice() {
        let slice: Slice<i32> = Slice::new(&[]);
        assert_eq!(slice.to_string(), "[]");
    }

    #[test]
    fn test_single_element_slice() {
        let slice = Slice::new(&["a"]);
        assert_eq!(slice.to_string(), "[a]");
    }
}
