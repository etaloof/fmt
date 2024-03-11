use std::fmt::{Display, Write};

/// Counts the number of bytes produced by a `Display` implementation.
pub struct CountChars {
    counter: usize,
}

impl CountChars {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    /// Counts the number of characters produced by a `Display` implementation.
    pub fn count(item: impl Display) -> usize {
        let mut c = Self::new();
        write!(&mut c, "{}", item).unwrap();
        c.counter
    }
}

impl Write for CountChars {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.counter += s.chars().count();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_chars_empty_string() {
        let item = "";
        assert_eq!(CountChars::count(item), item.len());
    }

    #[test]
    fn test_count_chars_hello() {
        let item = "hello";
        assert_eq!(CountChars::count(item), item.len());
    }

    #[test]
    fn test_count_chars_hello_world() {
        let item = "hello world";
        assert_eq!(CountChars::count(item), item.len());
    }

    #[test]
    fn test_count_chars_hello_world2() {
        let item = "hello world!";
        assert_eq!(CountChars::count(item), item.len());
    }

    #[test]
    fn test_count_chars_multiline() {
        let item = r#"
        "#;
        assert_eq!(CountChars::count(item), item.len());
    }

    #[test]
    fn test_count_chars_single_quote() {
        let item = "'";
        assert_eq!(CountChars::count(item), item.len());
    }

    #[test]
    fn test_count_chars_double_quote() {
        let item = "\"";
        assert_eq!(CountChars::count(item), item.len());
    }

    #[test]
    fn test_count_chars_rust_code() {
        let item = "fn main() { println!(\"Hello, world!\"); }";
        assert_eq!(CountChars::count(item), item.len());
    }

    #[test]
    fn test_count_chars_comment() {
        let item = "// This is a comment";
        assert_eq!(CountChars::count(item), item.len());
    }

    #[test]
    fn test_count_chars_sharp_s() {
        let item = "ß";
        assert_eq!(CountChars::count(item), 1);
    }

    #[test]
    fn test_count_chars_smile_emoji() {
        let item = "😊";
        assert_eq!(CountChars::count(item), 1);
    }

    #[test]
    fn test_count_chars_smile_emoji_twice() {
        let item = "😊😊";
        assert_eq!(CountChars::count(item), 2);
    }

    #[test]
    fn test_count_chars_pi() {
        let item = "π";
        assert_eq!(CountChars::count(item), 1);
    }

    #[test]
    fn test_count_chars_hao() {
        let item = "好";
        assert_eq!(CountChars::count(item), 1);
    }
}
