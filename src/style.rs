use std::{fmt::Formatter, marker::PhantomData};

pub struct F<'a, S: FormatStyle> {
    s: S,
    st: &'a S::Type,
}

impl<'a, T: std::fmt::Display> F<'a, Display<T>> {
    pub fn new_display(st: &'a T) -> Self {
        F {
            s: Display(PhantomData),
            st,
        }
    }
}

impl<'a, T: std::fmt::Debug> F<'a, Debug<T>> {
    pub fn new_debug(st: &'a T) -> Self {
        F {
            s: Debug(PhantomData),
            st,
        }
    }
}

impl<'a, T: std::fmt::Octal> F<'a, Octal<T>> {
    pub fn new_octal(st: &'a T) -> Self {
        F {
            s: Octal(PhantomData),
            st,
        }
    }
}

impl<'a, T: std::fmt::Binary> F<'a, Binary<T>> {
    pub fn new_binary(st: &'a T) -> Self {
        F {
            s: Binary(PhantomData),
            st,
        }
    }
}

impl<'a, T: std::fmt::LowerHex> F<'a, LowerHex<T>> {
    pub fn new_lower_hex(st: &'a T) -> Self {
        F {
            s: LowerHex(PhantomData),
            st,
        }
    }
}

impl<'a, T: std::fmt::UpperHex> F<'a, UpperHex<T>> {
    pub fn new_upper_hex(st: &'a T) -> Self {
        F {
            s: UpperHex(PhantomData),
            st,
        }
    }
}

impl<'a, T: std::fmt::LowerExp> F<'a, LowerExp<T>> {
    pub fn new_lower_exp(st: &'a T) -> Self {
        F {
            s: LowerExp(PhantomData),
            st,
        }
    }
}

impl<'a, T: std::fmt::UpperExp> F<'a, UpperExp<T>> {
    pub fn new_upper_exp(st: &'a T) -> Self {
        F {
            s: UpperExp(PhantomData),
            st,
        }
    }
}

impl<'a, T: std::fmt::Pointer> F<'a, Pointer<T>> {
    pub fn new_pointer(st: &'a T) -> Self {
        F {
            s: Pointer(PhantomData),
            st,
        }
    }
}

impl<'a, S: FormatStyle> std::fmt::Display for F<'a, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.s.fmt(self.st, f)
    }
}

pub trait FormatStyle: Copy {
    type Type;
    fn fmt<'a>(&self, item: &'a Self::Type, f: &mut Formatter<'_>) -> std::fmt::Result;
}

struct Display<T>(PhantomData<*const T>);

impl<T> Copy for Display<T> {}

impl<T> Clone for Display<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: std::fmt::Display> FormatStyle for Display<T> {
    type Type = T;

    fn fmt<'a>(&self, item: &'a Self::Type, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", item)
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
    type Type = T;

    fn fmt<'a>(&self, item: &'a Self::Type, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", item)
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
    type Type = T;

    fn fmt<'a>(&self, item: &'a Self::Type, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:o}", item)
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
    type Type = T;

    fn fmt<'a>(&self, item: &'a Self::Type, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", item)
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
    type Type = T;

    fn fmt<'a>(&self, item: &'a Self::Type, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", item)
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
    type Type = T;

    fn fmt<'a>(&self, item: &'a Self::Type, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", item)
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
    type Type = T;

    fn fmt<'a>(&self, item: &'a Self::Type, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}", item)
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
    type Type = T;

    fn fmt<'a>(&self, item: &'a Self::Type, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:E}", item)
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
    type Type = T;

    fn fmt<'a>(&self, item: &'a Self::Type, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p}", *item)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Formatter;

    use crate::{adhoc::Disp, style::F};

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

        assert_eq!(actual, F::new_display(&actual).to_string())
    }

    #[test]
    fn test_debug() {
        let dummy = Dummy(4183263);

        let actual = dummy.debug_repr();

        assert_eq!(actual.to_string(), F::new_debug(&dummy).to_string())
    }

    #[test]
    fn test_binary() {
        let dummy = Dummy(4183263);

        let actual = dummy.binary_repr();

        assert_eq!(actual.to_string(), F::new_binary(&dummy).to_string())
    }

    #[test]
    fn test_octal() {
        let dummy = Dummy(4183263);

        let actual = dummy.octal_repr();

        assert_eq!(actual.to_string(), F::new_octal(&dummy).to_string())
    }

    #[test]
    fn test_lower_hex() {
        let dummy = Dummy(4183263);

        let actual = dummy.lower_hex_repr();

        assert_eq!(actual.to_string(), F::new_lower_hex(&dummy).to_string())
    }

    #[test]
    fn test_upper_hex() {
        let dummy = Dummy(4183263);

        let actual = dummy.upper_hex_repr();

        assert_eq!(actual.to_string(), F::new_upper_hex(&dummy).to_string())
    }

    #[test]
    fn test_lower_exp() {
        let dummy = Dummy(4183263);

        let actual = dummy.lower_exp_repr();

        assert_eq!(actual.to_string(), F::new_lower_exp(&dummy).to_string())
    }

    #[test]
    fn test_upper_exp() {
        let dummy = Dummy(4183263);

        let actual = dummy.upper_exp_repr();

        assert_eq!(actual.to_string(), F::new_upper_exp(&dummy).to_string())
    }

    #[test]
    fn test_pointer() {
        let dummy = Dummy(4183263);

        let actual = dummy.pointer_repr();

        assert_eq!(actual.to_string(), F::new_pointer(&dummy).to_string())
    }
}
