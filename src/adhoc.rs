use std::fmt::{Display, Formatter};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{adapters::Display as DisplayAdapter, utils::LastIterationIterator};

    #[derive(Copy, Clone)]
    struct EchoPoint {
        row_idx: usize,
        ampl: usize,
    }

    struct ColView {
        col_idx: usize,
        echo_points: [EchoPoint; 2],
    }

    struct Frame {
        cols: Vec<[EchoPoint; 4]>,
    }

    impl Frame {
        pub(crate) fn cols(&self) -> impl Iterator<Item = ColView> + '_ {
            self.cols.iter().enumerate().flat_map(|(i, &[a, b, c, d])| {
                [
                    ColView {
                        col_idx: i * 2,
                        echo_points: [a, b],
                    },
                    ColView {
                        col_idx: i * 2 + 1,
                        echo_points: [c, d],
                    },
                ]
            })
        }
    }

    fn custom_display(frame: &Frame) -> impl Display + '_ {
        Disp::new(|f| {
            for (is_last, c) in LastIterationIterator::new(frame.cols()) {
                f.debug_list()
                    .entries(c.echo_points.iter().map(|echo| {
                        DisplayAdapter::new(Disp::new(|f| {
                            f.debug_struct("Echo")
                                .field("col_idx", &c.col_idx)
                                .field("row_idx", &echo.row_idx)
                                .field("ampl", &echo.ampl)
                                .finish()
                        }))
                    }))
                    .finish()?;

                if !is_last {
                    writeln!(f)?;
                }
            }
            Ok(())
        })
    }

    #[test]
    fn test_empty() {
        let frame = Frame { cols: vec![] };

        assert_eq!(custom_display(&frame).to_string(), "");
    }

    const ECHOES_UNFORMATTED: &'static str = r#"[Echo { col_idx: 0, row_idx: 0, ampl: 1 }, Echo { col_idx: 0, row_idx: 1, ampl: 2 }]
[Echo { col_idx: 1, row_idx: 2, ampl: 3 }, Echo { col_idx: 1, row_idx: 3, ampl: 4 }]
[Echo { col_idx: 2, row_idx: 0, ampl: 5 }, Echo { col_idx: 2, row_idx: 1, ampl: 6 }]
[Echo { col_idx: 3, row_idx: 2, ampl: 7 }, Echo { col_idx: 3, row_idx: 3, ampl: 8 }]
[Echo { col_idx: 4, row_idx: 0, ampl: 9 }, Echo { col_idx: 4, row_idx: 1, ampl: 10 }]
[Echo { col_idx: 5, row_idx: 2, ampl: 11 }, Echo { col_idx: 5, row_idx: 3, ampl: 12 }]
[Echo { col_idx: 6, row_idx: 0, ampl: 13 }, Echo { col_idx: 6, row_idx: 1, ampl: 14 }]
[Echo { col_idx: 7, row_idx: 2, ampl: 15 }, Echo { col_idx: 7, row_idx: 3, ampl: 16 }]"#;

    const ECHOES_FORMATTED: &'static str = r#"[
    Echo {
        col_idx: 0,
        row_idx: 0,
        ampl: 1,
    },
    Echo {
        col_idx: 0,
        row_idx: 1,
        ampl: 2,
    },
]
[
    Echo {
        col_idx: 1,
        row_idx: 2,
        ampl: 3,
    },
    Echo {
        col_idx: 1,
        row_idx: 3,
        ampl: 4,
    },
]
[
    Echo {
        col_idx: 2,
        row_idx: 0,
        ampl: 5,
    },
    Echo {
        col_idx: 2,
        row_idx: 1,
        ampl: 6,
    },
]
[
    Echo {
        col_idx: 3,
        row_idx: 2,
        ampl: 7,
    },
    Echo {
        col_idx: 3,
        row_idx: 3,
        ampl: 8,
    },
]
[
    Echo {
        col_idx: 4,
        row_idx: 0,
        ampl: 9,
    },
    Echo {
        col_idx: 4,
        row_idx: 1,
        ampl: 10,
    },
]
[
    Echo {
        col_idx: 5,
        row_idx: 2,
        ampl: 11,
    },
    Echo {
        col_idx: 5,
        row_idx: 3,
        ampl: 12,
    },
]
[
    Echo {
        col_idx: 6,
        row_idx: 0,
        ampl: 13,
    },
    Echo {
        col_idx: 6,
        row_idx: 1,
        ampl: 14,
    },
]
[
    Echo {
        col_idx: 7,
        row_idx: 2,
        ampl: 15,
    },
    Echo {
        col_idx: 7,
        row_idx: 3,
        ampl: 16,
    },
]"#;

    #[test]
    fn test_4x4() {
        let frame = Frame {
            cols: vec![
                [
                    EchoPoint {
                        row_idx: 0,
                        ampl: 1,
                    },
                    EchoPoint {
                        row_idx: 1,
                        ampl: 2,
                    },
                    EchoPoint {
                        row_idx: 2,
                        ampl: 3,
                    },
                    EchoPoint {
                        row_idx: 3,
                        ampl: 4,
                    },
                ],
                [
                    EchoPoint {
                        row_idx: 0,
                        ampl: 5,
                    },
                    EchoPoint {
                        row_idx: 1,
                        ampl: 6,
                    },
                    EchoPoint {
                        row_idx: 2,
                        ampl: 7,
                    },
                    EchoPoint {
                        row_idx: 3,
                        ampl: 8,
                    },
                ],
                [
                    EchoPoint {
                        row_idx: 0,
                        ampl: 9,
                    },
                    EchoPoint {
                        row_idx: 1,
                        ampl: 10,
                    },
                    EchoPoint {
                        row_idx: 2,
                        ampl: 11,
                    },
                    EchoPoint {
                        row_idx: 3,
                        ampl: 12,
                    },
                ],
                [
                    EchoPoint {
                        row_idx: 0,
                        ampl: 13,
                    },
                    EchoPoint {
                        row_idx: 1,
                        ampl: 14,
                    },
                    EchoPoint {
                        row_idx: 2,
                        ampl: 15,
                    },
                    EchoPoint {
                        row_idx: 3,
                        ampl: 16,
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
