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
    fn test_counter_empty_string() {
        let item = "";
        assert_eq!(Counter::count(item), item.len());
    }

    #[test]
    fn test_counter_hello() {
        let item = "hello";
        assert_eq!(Counter::count(item), item.len());
    }

    #[test]
    fn test_counter_hello_world() {
        let item = "hello world";
        assert_eq!(Counter::count(item), item.len());
    }

    #[test]
    fn test_counter_hello_world2() {
        let item = "hello world!";
        assert_eq!(Counter::count(item), item.len());
    }

    #[test]
    fn test_counter_multiline() {
        let item = r#"
        "#;
        assert_eq!(Counter::count(item), item.len());
    }

    #[test]
    fn test_count_single_quote() {
        let item = "'";
        assert_eq!(Counter::count(item), item.len());
    }

    #[test]
    fn test_count_double_quote() {
        let item = "\"";
        assert_eq!(Counter::count(item), item.len());
    }

    #[test]
    fn test_count_rust_code() {
        let item = "fn main() { println!(\"Hello, world!\"); }";
        assert_eq!(Counter::count(item), item.len());
    }

    #[test]
    fn test_count_comment() {
        let item = "// This is a comment";
        assert_eq!(Counter::count(item), item.len());
    }
}
