pub mod adapters;
pub mod adhoc;
pub mod adhoc2;
pub mod color;
pub mod count_bytes;
pub mod count_chars;
pub mod either;
pub mod hex;
pub mod hexslice;
pub mod hexviewer;
pub mod joined;
pub mod joined2;
pub mod logic;
pub mod pad;
pub mod producer;
pub mod replace;
#[cfg(feature = "serde")]
pub mod serde_json;
pub mod slice;
pub mod style;
pub mod utils;

#[derive(Copy, Clone)]
pub enum Align {
    Left,
    Right,
}

#[macro_export]
macro_rules! disp {
    ($d:expr) => {{
        $crate::adhoc::Disp::new($d)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_disp() {
        let expected = "test";

        let actual = disp!(|f| write!(f, "{}", expected)).to_string();

        assert_eq!(actual, expected)
    }
}
