use std::{fmt::Formatter, marker::PhantomData};

pub struct Style<'a, S: FormatStyle> {
    format_style: S,
    item: &'a S::Item,
}

impl<'a, T: std::fmt::Display> Style<'a, Display<T>> {
    pub fn new_display(item: &'a T) -> Self {
        Style {
            format_style: Display(PhantomData),
            item,
        }
    }
}

impl<'a, T: std::fmt::Debug> Style<'a, Debug<T>> {
    pub fn new_debug(item: &'a T) -> Self {
        Style {
            format_style: Debug(PhantomData),
            item,
        }
    }
}

impl<'a, T: std::fmt::Octal> Style<'a, Octal<T>> {
    pub fn new_octal(item: &'a T) -> Self {
        Style {
            format_style: Octal(PhantomData),
            item,
        }
    }
}

impl<'a, T: std::fmt::Binary> Style<'a, Binary<T>> {
    pub fn new_binary(item: &'a T) -> Self {
        Style {
            format_style: Binary(PhantomData),
            item,
        }
    }
}

impl<'a, T: std::fmt::LowerHex> Style<'a, LowerHex<T>> {
    pub fn new_lower_hex(item: &'a T) -> Self {
        Style {
            format_style: LowerHex(PhantomData),
            item,
        }
    }
}

impl<'a, T: std::fmt::UpperHex> Style<'a, UpperHex<T>> {
    pub fn new_upper_hex(item: &'a T) -> Self {
        Style {
            format_style: UpperHex(PhantomData),
            item,
        }
    }
}

impl<'a, T: std::fmt::LowerExp> Style<'a, LowerExp<T>> {
    pub fn new_lower_exp(item: &'a T) -> Self {
        Style {
            format_style: LowerExp(PhantomData),
            item,
        }
    }
}

impl<'a, T: std::fmt::UpperExp> Style<'a, UpperExp<T>> {
    pub fn new_upper_exp(item: &'a T) -> Self {
        Style {
            format_style: UpperExp(PhantomData),
            item,
        }
    }
}

impl<'a, T: std::fmt::Pointer> Style<'a, Pointer<T>> {
    pub fn new_pointer(item: &'a T) -> Self {
        Style {
            format_style: Pointer(PhantomData),
            item,
        }
    }
}

impl<'a, S: FormatStyle> std::fmt::Display for Style<'a, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format_style.fmt(self.item, f)
    }
}

pub trait FormatStyle: Copy {
    type Item;
    fn fmt<'a>(&self, item: &'a Self::Item, f: &mut Formatter<'_>) -> std::fmt::Result;
}

struct Display<T>(PhantomData<*const T>);

impl<T> Copy for Display<T> {}

impl<T> Clone for Display<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: std::fmt::Display> FormatStyle for Display<T> {
    type Item = T;

    fn fmt<'a>(&self, item: &'a Self::Item, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(item, f)
    }
}

struct Debug<T>(PhantomData<*const T>);

impl<T> Copy for Debug<T> {}

impl<T> Clone for Debug<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: std::fmt::Debug> FormatStyle for Debug<T> {
    type Item = T;

    fn fmt<'a>(&self, item: &'a Self::Item, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(item, f)
    }
}

struct Octal<T>(PhantomData<*const T>);

impl<T> Copy for Octal<T> {}

impl<T> Clone for Octal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: std::fmt::Octal> FormatStyle for Octal<T> {
    type Item = T;

    fn fmt<'a>(&self, item: &'a Self::Item, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(item, f)
    }
}

struct Binary<T>(PhantomData<*const T>);

impl<T> Copy for Binary<T> {}

impl<T> Clone for Binary<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: std::fmt::Binary> FormatStyle for Binary<T> {
    type Item = T;

    fn fmt<'a>(&self, item: &'a Self::Item, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(item, f)
    }
}

struct LowerHex<T>(PhantomData<*const T>);

impl<T> Copy for LowerHex<T> {}

impl<T> Clone for LowerHex<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: std::fmt::LowerHex> FormatStyle for LowerHex<T> {
    type Item = T;

    fn fmt<'a>(&self, item: &'a Self::Item, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(item, f)
    }
}

struct UpperHex<T>(PhantomData<*const T>);

impl<T> Copy for UpperHex<T> {}

impl<T> Clone for UpperHex<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: std::fmt::UpperHex> FormatStyle for UpperHex<T> {
    type Item = T;

    fn fmt<'a>(&self, item: &'a Self::Item, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(item, f)
    }
}

struct LowerExp<T>(PhantomData<*const T>);

impl<T> Copy for LowerExp<T> {}

impl<T> Clone for LowerExp<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: std::fmt::LowerExp> FormatStyle for LowerExp<T> {
    type Item = T;

    fn fmt<'a>(&self, item: &'a Self::Item, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerExp::fmt(item, f)
    }
}

struct UpperExp<T>(PhantomData<*const T>);

impl<T> Copy for UpperExp<T> {}

impl<T> Clone for UpperExp<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: std::fmt::UpperExp> FormatStyle for UpperExp<T> {
    type Item = T;

    fn fmt<'a>(&self, item: &'a Self::Item, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperExp::fmt(item, f)
    }
}

pub struct Pointer<T>(PhantomData<*const T>);

impl<T> Copy for Pointer<T> {}

impl<T> Clone for Pointer<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: std::fmt::Pointer> FormatStyle for Pointer<T> {
    type Item = T;

    fn fmt<'a>(&self, item: &'a Self::Item, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(item, f)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Formatter;

    use crate::{adhoc::Disp, style::Style};

    struct Dummy(u64);

    impl Dummy {
        fn display_repr(&self) -> String {
            Disp::new(|f| std::fmt::Display::fmt(self, f)).to_string()
        }

        fn debug_repr(&self) -> String {
            Disp::new(|f| std::fmt::Debug::fmt(self, f)).to_string()
        }

        fn binary_repr(&self) -> String {
            Disp::new(|f| std::fmt::Binary::fmt(self, f)).to_string()
        }

        fn octal_repr(&self) -> String {
            Disp::new(|f| std::fmt::Octal::fmt(self, f)).to_string()
        }

        fn lower_hex_repr(&self) -> String {
            Disp::new(|f| std::fmt::LowerHex::fmt(self, f)).to_string()
        }

        fn upper_hex_repr(&self) -> String {
            Disp::new(|f| std::fmt::UpperHex::fmt(self, f)).to_string()
        }

        fn lower_exp_repr(&self) -> String {
            Disp::new(|f| std::fmt::LowerExp::fmt(self, f)).to_string()
        }

        fn upper_exp_repr(&self) -> String {
            Disp::new(|f| std::fmt::UpperExp::fmt(self, f)).to_string()
        }

        fn pointer_repr(&self) -> String {
            Disp::new(|f| std::fmt::Pointer::fmt(self, f)).to_string()
        }
    }

    impl std::fmt::Display for Dummy {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::fmt::Debug for Dummy {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dummy({})", self.0)
        }
    }

    impl std::fmt::Binary for Dummy {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dummy(0b{:b})", self.0)
        }
    }

    impl std::fmt::Octal for Dummy {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dummy(0o{:b})", self.0)
        }
    }

    impl std::fmt::LowerHex for Dummy {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dummy(0x{:x})", self.0)
        }
    }

    impl std::fmt::UpperHex for Dummy {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dummy(0x{:X})", self.0)
        }
    }

    impl std::fmt::LowerExp for Dummy {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dummy({:e})", self.0)
        }
    }

    impl std::fmt::UpperExp for Dummy {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dummy({:E})", self.0)
        }
    }

    impl std::fmt::Pointer for Dummy {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dummy*(0x{:p})", self.0 as *const ())
        }
    }

    #[test]
    fn test_display() {
        let dummy = Dummy(4183263);

        let actual = dummy.display_repr();

        assert_eq!(actual, Style::new_display(&actual).to_string())
    }

    #[test]
    fn test_debug() {
        let dummy = Dummy(4183263);

        let actual = dummy.debug_repr();

        assert_eq!(actual.to_string(), Style::new_debug(&dummy).to_string())
    }

    #[test]
    fn test_binary() {
        let dummy = Dummy(4183263);

        let actual = dummy.binary_repr();

        assert_eq!(actual.to_string(), Style::new_binary(&dummy).to_string())
    }

    #[test]
    fn test_octal() {
        let dummy = Dummy(4183263);

        let actual = dummy.octal_repr();

        assert_eq!(actual.to_string(), Style::new_octal(&dummy).to_string())
    }

    #[test]
    fn test_lower_hex() {
        let dummy = Dummy(4183263);

        let actual = dummy.lower_hex_repr();

        assert_eq!(actual.to_string(), Style::new_lower_hex(&dummy).to_string())
    }

    #[test]
    fn test_upper_hex() {
        let dummy = Dummy(4183263);

        let actual = dummy.upper_hex_repr();

        assert_eq!(actual.to_string(), Style::new_upper_hex(&dummy).to_string())
    }

    #[test]
    fn test_lower_exp() {
        let dummy = Dummy(4183263);

        let actual = dummy.lower_exp_repr();

        assert_eq!(actual.to_string(), Style::new_lower_exp(&dummy).to_string())
    }

    #[test]
    fn test_upper_exp() {
        let dummy = Dummy(4183263);

        let actual = dummy.upper_exp_repr();

        assert_eq!(actual.to_string(), Style::new_upper_exp(&dummy).to_string())
    }

    #[test]
    fn test_pointer() {
        let dummy = Dummy(4183263);

        let actual = dummy.pointer_repr();

        assert_eq!(actual.to_string(), Style::new_pointer(&dummy).to_string())
    }
}
