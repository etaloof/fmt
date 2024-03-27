//! This module contains a number of colors which can be applied to any type implementing the `Display` trait.
use std::fmt::{Display, Formatter};

use ansi_term::{Color, Style};

fn write<T: Display>(f: &mut Formatter<'_>, value: &T, color: Color) -> std::fmt::Result {
    let style = Style::new().fg(color);
    write!(f, "{}{}{}", style.prefix(), value, style.suffix())
}

/// Displays a value in black.
pub struct Black<T: Display>(pub T);

impl<T: Display> Display for Black<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = Color::Black;
        write(f, &self.0, color)
    }
}

/// Displays a value in red.
pub struct Red<T: Display>(pub T);

impl<T: Display> Display for Red<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = Color::Red;
        write(f, &self.0, color)
    }
}

/// Displays a value in green.
pub struct Green<T: Display>(pub T);

impl<T: Display> Display for Green<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = Color::Green;
        write(f, &self.0, color)
    }
}

/// Displays a value in yellow.
pub struct Yellow<T: Display>(pub T);

impl<T: Display> Display for Yellow<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = Color::Yellow;
        write(f, &self.0, color)
    }
}

/// Displays a value in blue.
pub struct Blue<T: Display>(pub T);

impl<T: Display> Display for Blue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = Color::Blue;
        write(f, &self.0, color)
    }
}

/// Displays a value in magenta.
pub struct Magenta<T: Display>(pub T);

impl<T: Display> Display for Magenta<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = Color::Purple;
        write(f, &self.0, color)
    }
}

/// Displays a value in cyan.
pub struct Cyan<T: Display>(pub T);

impl<T: Display> Display for Cyan<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = Color::Cyan;
        write(f, &self.0, color)
    }
}

/// Displays a value in white.
pub struct White<T: Display>(pub T);

impl<T: Display> Display for White<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = Color::White;
        write(f, &self.0, color)
    }
}

/// Tests for black, red, green, yellow, blue, magenta, cyan and white
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_color() {
        let black = Black("Hello World!");
        assert_eq!(black.to_string(), "\x1b[30mHello World!\x1b[0m");
    }

    #[test]
    fn red_color() {
        let red = Red("Hello World!");
        assert_eq!(red.to_string(), "\x1b[31mHello World!\x1b[0m");
    }

    #[test]
    fn green_color() {
        let green = Green("Hello World!");
        assert_eq!(green.to_string(), "\x1b[32mHello World!\x1b[0m");
    }

    #[test]
    fn yellow_color() {
        let yellow = Yellow("Hello World!");
        assert_eq!(yellow.to_string(), "\x1b[33mHello World!\x1b[0m");
    }

    #[test]
    fn blue_color() {
        let blue = Blue("Hello World!");
        assert_eq!(blue.to_string(), "\x1b[34mHello World!\x1b[0m");
    }

    #[test]
    fn magenta_color() {
        let magenta = Magenta("Hello World!");
        assert_eq!(magenta.to_string(), "\x1b[35mHello World!\x1b[0m");
    }

    #[test]
    fn cyan_color() {
        let cyan = Cyan("Hello World!");
        assert_eq!(cyan.to_string(), "\x1b[36mHello World!\x1b[0m");
    }

    #[test]
    fn white_color() {
        let white = White("Hello World!");
        assert_eq!(white.to_string(), "\x1b[37mHello World!\x1b[0m");
    }
}
