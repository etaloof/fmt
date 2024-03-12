use std::fmt::{Display, Formatter};

const RESET: &str = "\x1b[0m";
const BLACK: &str = "30";
const RED: &str = "31";
const GREEN: &str = "32";
const YELLOW: &str = "33";
const BLUE: &str = "34";
const MAGENTA: &str = "35";
const CYAN: &str = "36";
const WHITE: &str = "37";

// TODO: Document
pub struct Black<T: Display>(pub T);

impl<T: Display> Display for Black<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m{}{}", BLACK, self.0, RESET)
    }
}

pub struct Red<T: Display>(pub T);

impl<T: Display> Display for Red<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m{}{}", RED, self.0, RESET)
    }
}

pub struct Green<T: Display>(pub T);

impl<T: Display> Display for Green<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m{}{}", GREEN, self.0, RESET)
    }
}

pub struct Yellow<T: Display>(pub T);

impl<T: Display> Display for Yellow<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m{}{}", YELLOW, self.0, RESET)
    }
}

pub struct Blue<T: Display>(pub T);

impl<T: Display> Display for Blue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m{}{}", BLUE, self.0, RESET)
    }
}

pub struct Magenta<T: Display>(pub T);

impl<T: Display> Display for Magenta<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m{}{}", MAGENTA, self.0, RESET)
    }
}

pub struct Cyan<T: Display>(pub T);

impl<T: Display> Display for Cyan<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m{}{}", CYAN, self.0, RESET)
    }
}

pub struct White<T: Display>(pub T);

impl<T: Display> Display for White<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m{}{}", WHITE, self.0, RESET)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_color() {
        let white = White("Hello World!");
        assert_eq!(white.to_string(), "\x1b[37mHello World!\x1b[0m");
    }
}
