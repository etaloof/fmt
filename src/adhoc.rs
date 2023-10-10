use std::fmt::{Display, Formatter};

pub struct Disp<F>(F)
where
    F: Fn(&mut Formatter) -> std::fmt::Result;

impl<F> Disp<F>
where
    F: Fn(&mut Formatter) -> std::fmt::Result,
{
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F> Display for Disp<F>
where
    F: Fn(&mut Formatter) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (self.0)(f)
    }
}
