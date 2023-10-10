use std::fmt::{Display, Formatter, Write};

pub struct DisplayPadded<'a> {
    display: &'a dyn Display,
    max_length: usize,
}

impl<'a> DisplayPadded<'a> {
    pub fn new(display: &'a dyn Display) -> Self {
        Self {
            display,
            max_length: 0,
        }
    }

    pub fn padded(self, max_length: usize) -> Self {
        Self {
            display: self.display,
            max_length,
        }
    }
}

impl<'a> Display for DisplayPadded<'a> {
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

        let mut ff = Counter::new(&mut *f);
        write!(ff, "{}", self.display)?;
        let written = ff.written();
        for _ in written..self.max_length {
            f.write_char(' ')?;
        }
        Ok(())
    }
}
