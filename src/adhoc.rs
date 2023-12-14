use std::fmt::{Debug, Display, Formatter};

pub struct Disp<F>
where
    F: for<'a> Fn(&mut Formatter<'a>) -> std::fmt::Result,
{
    f: F,
}

impl<F> Disp<F>
where
    F: for<'a> Fn(&mut Formatter<'a>) -> std::fmt::Result,
{
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F> Display for Disp<F>
where
    F: for<'a> Fn(&mut Formatter<'a>) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (self.f)(f)
    }
}

impl<F> Debug for Disp<F>
where
    F: for<'a> Fn(&mut Formatter<'a>) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (self.f)(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{adapters::Display as DisplayAdapter, utils::LastIterationIterator};

    #[derive(Copy, Clone)]
    struct Datum {
        authenticity: u8,
        confidence: u8,
        is_valid: bool,
    }

    struct Data {
        entries: Vec<[Datum; 4]>,
    }

    fn custom_display(data: &Data) -> impl Display + '_ {
        Disp::new(|f| {
            for (is_last, de) in LastIterationIterator::new(data.entries.iter()) {
                let iter = de
                    .iter()
                    .filter(|d| d.is_valid)
                    .filter(|d| d.confidence >= 35)
                    .map(|d| Disp::new(|f| write!(f, "{}", d.authenticity)));
                f.debug_list().entries(iter).finish()?;

                if !is_last {
                    writeln!(f)?;
                }
            }
            Ok(())
        })
    }

    #[test]
    fn test_empty() {
        let frame = Data { entries: vec![] };

        assert_eq!(custom_display(&frame).to_string(), "");
    }

    const ECHOES_UNFORMATTED: &'static str = r#"[3]
[7]
[11]
[13, 15]"#;

    const ECHOES_FORMATTED: &'static str = r#"[
    3,
]
[
    7,
]
[
    11,
]
[
    13,
    15,
]"#;

    #[test]
    fn test_4x4() {
        let frame = Data {
            entries: vec![
                [
                    Datum {
                        authenticity: 0,
                        confidence: 35,
                        is_valid: false,
                    },
                    Datum {
                        authenticity: 1,
                        confidence: 33,
                        is_valid: false,
                    },
                    Datum {
                        authenticity: 2,
                        confidence: 65,
                        is_valid: false,
                    },
                    Datum {
                        authenticity: 3,
                        confidence: 65,
                        is_valid: true,
                    },
                ],
                [
                    Datum {
                        authenticity: 4,
                        confidence: 35,
                        is_valid: false,
                    },
                    Datum {
                        authenticity: 5,
                        confidence: 33,
                        is_valid: false,
                    },
                    Datum {
                        authenticity: 6,
                        confidence: 65,
                        is_valid: false,
                    },
                    Datum {
                        authenticity: 7,
                        confidence: 65,
                        is_valid: true,
                    },
                ],
                [
                    Datum {
                        authenticity: 8,
                        confidence: 35,
                        is_valid: false,
                    },
                    Datum {
                        authenticity: 9,
                        confidence: 30,
                        is_valid: true,
                    },
                    Datum {
                        authenticity: 10,
                        confidence: 65,
                        is_valid: false,
                    },
                    Datum {
                        authenticity: 11,
                        confidence: 65,
                        is_valid: true,
                    },
                ],
                [
                    Datum {
                        authenticity: 12,
                        confidence: 35,
                        is_valid: false,
                    },
                    Datum {
                        authenticity: 13,
                        confidence: 39,
                        is_valid: true,
                    },
                    Datum {
                        authenticity: 14,
                        confidence: 65,
                        is_valid: false,
                    },
                    Datum {
                        authenticity: 15,
                        confidence: 65,
                        is_valid: true,
                    },
                ],
            ],
        };

        let s = custom_display(&frame).to_string();
        assert_eq!(s, ECHOES_UNFORMATTED);

        let s = format!("{}", custom_display(&frame));
        assert_eq!(s, ECHOES_UNFORMATTED);

        let s = format!("{:?}", DisplayAdapter::new(custom_display(&frame)));
        assert_eq!(s, ECHOES_UNFORMATTED);

        let s = format!("{:#?}", DisplayAdapter::new(custom_display(&frame)));
        assert_eq!(s, ECHOES_FORMATTED);
    }
}
