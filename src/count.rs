use std::fmt::{Display, Write};

pub struct Counter {
    counter: usize,
}

impl Counter {
    fn new() -> Self {
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
