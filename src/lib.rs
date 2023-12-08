//pub mod hexiter;
pub mod adhoc;
pub mod count_bytes;
pub mod count_chars;
pub mod custom;
pub mod either;
pub mod hex;
pub mod hexslice;
pub mod hexviewer;
pub mod joined;
pub mod joined2;
pub mod pad;
pub mod replace;
#[cfg(feature = "serde")]
pub mod serde_json;
pub mod slice;
pub mod style;
mod utils;

#[derive(Copy, Clone)]
pub enum Align {
    Left,
    Right,
}

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
