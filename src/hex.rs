use crate::adapters;

/// A type alias for a `LowerHex`.
pub type LowerHex<T> = adapters::LowerHex<T>;

/// A type alias for a `UpperHex`.
pub type UpperHex<T> = adapters::UpperHex<T>;

/// Type alias for a `LowerHex`.
pub type Hex<I> = LowerHex<I>;
