use std::fmt::{Display, Write};

pub struct Counter {
    counter: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn count(item: impl Display) -> usize {
        let mut c = Self::new();
        write!(&mut c, "{}", item).unwrap();
        c.counter
    }
}

impl Write for Counter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.counter += s.len();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        assert_eq!(Counter::count(""), 0);
        assert_eq!(Counter::count("hello"), 5);
        assert_eq!(Counter::count("hello world"), 11);
    }
}
