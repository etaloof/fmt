use std::fmt::Formatter;

pub struct Display<T>(T)
where
    T: std::fmt::Display;

impl<T> Display<T>
where
    T: std::fmt::Display,
{
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T> std::fmt::Display for Display<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl<T> std::fmt::Debug for Display<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

pub struct Debug<T>(T)
where
    T: std::fmt::Debug;

impl<T> Debug<T>
where
    T: std::fmt::Debug,
{
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T> std::fmt::Display for Debug<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl<T> std::fmt::Debug for Debug<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

/// A wrapper type for formatting values as hexadecimal strings with a specific case.
pub struct LowerHex<T: std::fmt::LowerHex> {
    item: T,
}

impl<T: std::fmt::LowerHex> LowerHex<T>
where
    T: std::fmt::LowerHex,
{
    /// Creates a new `LowerHex` value with the given value.
    pub fn new(item: T) -> Self {
        Self { item }
    }
}

impl<T: std::fmt::LowerHex> std::fmt::Display for LowerHex<T> {
    /// Formats the value as lowercase hexadecimal.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.item, f)
    }
}

impl<T: std::fmt::LowerHex> std::fmt::Debug for LowerHex<T> {
    /// Formats the value as lowercase hexadecimal.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.item, f)
    }
}

/// A wrapper type for formatting values as hexadecimal strings with a specific case.
pub struct UpperHex<T: std::fmt::UpperHex> {
    /// The value that is being formatted.
    item: T,
}

impl<T: std::fmt::UpperHex> UpperHex<T> {
    /// Creates a new `UpperHex` value with the given value.
    pub fn new(item: T) -> Self {
        Self { item }
    }
}

impl<T: std::fmt::UpperHex> std::fmt::Display for UpperHex<T> {
    /// Formats the value as uppercase hexadecimal.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.item, f)
    }
}

impl<T: std::fmt::UpperHex> std::fmt::Debug for UpperHex<T> {
    /// Formats the value as uppercase hexadecimal.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.item, f)
    }
}
