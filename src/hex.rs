use std::fmt::{Display, Formatter};

/// Type alias for a `LowerHex`.
pub type Hex<I> = LowerHex<I>;

/// A wrapper type for formatting values as hexadecimal strings with a specific case.
pub struct LowerHex<I: std::fmt::LowerHex> {
    item: I,
}

impl<I: std::fmt::LowerHex> LowerHex<I>
where
    I: std::fmt::LowerHex,
{
    /// Creates a new `LowerHex` value with the given value.
    pub fn new(item: I) -> Self {
        Self { item }
    }
}

impl<I: std::fmt::LowerHex> Display for LowerHex<I> {
    /// Formats the value as lowercase hexadecimal.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.item, f)
    }
}

/// A wrapper type for formatting values as hexadecimal strings with a specific case.
pub struct UpperHex<I: std::fmt::UpperHex> {
    /// The value that is being formatted.
    item: I,
}

impl<I: std::fmt::UpperHex> UpperHex<I>
where
    I: std::fmt::UpperHex,
{
    /// Creates a new `UpperHex` value with the given value.
    pub fn new(item: I) -> Self {
        Self { item }
    }
}

impl<I: std::fmt::UpperHex> Display for UpperHex<I> {
    /// Formats the value as uppercase hexadecimal.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.item, f)
    }
}
