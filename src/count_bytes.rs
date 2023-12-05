use std::fmt::{Display, Write};

pub struct CountBytes {
    counter: usize,
}

impl CountBytes {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn count(item: impl Display) -> usize {
        let mut c = Self::new();
        write!(&mut c, "{}", item).unwrap();
        c.counter
    }
}

impl Write for CountBytes {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.counter += s.len();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bytes_empty_string() {
        let item = "";
        assert_eq!(CountBytes::count(item), item.len());
    }

    #[test]
    fn test_count_bytes_hello() {
        let item = "hello";
        assert_eq!(CountBytes::count(item), item.len());
    }

    #[test]
    fn test_count_bytes_hello_world() {
        let item = "hello world";
        assert_eq!(CountBytes::count(item), item.len());
    }

    #[test]
    fn test_count_bytes_hello_world2() {
        let item = "hello world!";
        assert_eq!(CountBytes::count(item), item.len());
    }

    #[test]
    fn test_count_bytes_multiline() {
        let item = r#"
        "#;
        assert_eq!(CountBytes::count(item), item.len());
    }

    #[test]
    fn test_count_bytes_single_quote() {
        let item = "'";
        assert_eq!(CountBytes::count(item), item.len());
    }

    #[test]
    fn test_count_bytes_double_quote() {
        let item = "\"";
        assert_eq!(CountBytes::count(item), item.len());
    }

    #[test]
    fn test_count_bytes_rust_code() {
        let item = "fn main() { println!(\"Hello, world!\"); }";
        assert_eq!(CountBytes::count(item), item.len());
    }

    #[test]
    fn test_count_bytes_comment() {
        let item = "// This is a comment";
        assert_eq!(CountBytes::count(item), item.len());
    }
}
