use std::fmt::{Display, Formatter};

pub type FormatFunc<T> = for<'a, 'b, 'c> fn(&'a T, &'b mut Formatter<'c>) -> std::fmt::Result;

pub struct Disp<T> {
    item: T,
    func: FormatFunc<T>,
}

impl<T> Disp<T> {
    #[inline]
    pub fn new(item: T, func: FormatFunc<T>) -> Self {
        Self { item, func }
    }
}

impl<T> Display for Disp<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (self.func)(&self.item, f)
    }
}
