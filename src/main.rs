use fmt::adhoc::Disp;
use fmt::style::F;
use std::fmt::Formatter;

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

use core::arch::asm;

#[inline(never)]
fn test_pointer() {
    let dummy = Dummy(4183263);

    unsafe {
        asm!("ud2");
    }
    let actual = dummy.pointer_repr();
    let actual = actual.to_string();
    unsafe {
        asm!("ud2");
    }

    let string = F::new_pointer(&dummy).to_string();
    unsafe {
        asm!("ud2");
    }

    assert_eq!(actual, string)
}

fn main() {
    test_pointer();
}
