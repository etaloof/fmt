use std::fmt::{Display, Formatter, Write};

pub enum Alignment {
    Left,
    Right,
    Center,
}

pub struct DisplayPadded<D: Display> {
    display: D,
    max_length: usize,
    alignment: Alignment,
}

impl<D: Display> DisplayPadded<D> {
    pub fn new(display: D) -> Self {
        Self {
            display,
            max_length: 0,
            alignment: Alignment::Left,
        }
    }

    pub fn padded(self, max_length: usize) -> Self {
        Self { max_length, ..self }
    }

    pub fn aligned(self, alignment: Alignment) -> Self {
        Self { alignment, ..self }
    }
}

impl<D: Display> Display for DisplayPadded<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        struct Counter<Writer: Write> {
            count: usize,
            writer: Writer,
        }

        impl<Writer: Write> Counter<Writer> {
            fn new(writer: Writer) -> Self {
                Self { count: 0, writer }
            }

            fn written(&self) -> usize {
                self.count
            }
        }

        impl<Writer: Write> Write for Counter<Writer> {
            fn write_str(&mut self, s: &str) -> std::fmt::Result {
                self.count += s.chars().count();
                self.writer.write_str(s)
            }
        }

        struct Sink;

        impl std::fmt::Write for Sink {
            fn write_str(&mut self, _: &str) -> std::fmt::Result {
                Ok(())
            }
        }

        let c = ' ';
        match self.alignment {
            Alignment::Left => {
                let mut ff = Counter::new(Sink);
                write!(ff, "{}", self.display)?;
                let written = ff.written();
                for _ in written..self.max_length {
                    f.write_char(c)?;
                }
                write!(f, "{}", self.display)?;
            }
            Alignment::Right => {
                let mut ff = Counter::new(&mut *f);
                write!(ff, "{}", self.display)?;
                let written = ff.written();
                for _ in written..self.max_length {
                    f.write_char(c)?;
                }
            }
            Alignment::Center => {
                let mut ff = Counter::new(Sink);
                write!(ff, "{}", self.display)?;
                let written = ff.written();
                let distance = (written..self.max_length).len();
                let midpoint = distance / 2;
                for _ in 0..midpoint {
                    f.write_char(c)?;
                }
                write!(f, "{}", self.display)?;
                for _ in midpoint..distance {
                    f.write_char(c)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::pad::{Alignment, DisplayPadded};

    #[test]
    fn test_lpad3() {
        let dummy = "1";

        let actual = DisplayPadded::new(dummy).aligned(Alignment::Left).padded(3);

        assert_eq!(actual.to_string(), "  1")
    }

    #[test]
    fn test_lpad_noop() {
        let dummy = "1";

        let actual = DisplayPadded::new(dummy)
            .aligned(Alignment::Left)
            .padded(dummy.len());

        assert_eq!(actual.to_string(), dummy)
    }

    #[test]
    fn test_rpad3() {
        let dummy = "1";

        let actual = DisplayPadded::new(dummy)
            .aligned(Alignment::Right)
            .padded(3);

        assert_eq!(actual.to_string(), "1  ")
    }

    #[test]
    fn test_rpad_noop() {
        let dummy = "1";

        let actual = DisplayPadded::new(dummy)
            .aligned(Alignment::Right)
            .padded(dummy.len());

        assert_eq!(actual.to_string(), dummy)
    }

    #[test]
    fn test_cpad_noop() {
        let dummy = "1";

        let actual = DisplayPadded::new(dummy)
            .aligned(Alignment::Center)
            .padded(dummy.len());

        assert_eq!(actual.to_string(), dummy)
    }

    #[test]
    fn test_cpad2() {
        let dummy = "1";

        let actual = DisplayPadded::new(dummy)
            .aligned(Alignment::Center)
            .padded(2);

        assert_eq!(actual.to_string(), "1 ")
    }

    #[test]
    fn test_cpad3() {
        let dummy = "1";

        let actual = DisplayPadded::new(dummy)
            .aligned(Alignment::Center)
            .padded(3);

        assert_eq!(actual.to_string(), " 1 ")
    }

    #[test]
    fn test_cpad4() {
        let dummy = "1";

        let actual = DisplayPadded::new(dummy)
            .aligned(Alignment::Center)
            .padded(4);

        assert_eq!(actual.to_string(), " 1  ")
    }
}
