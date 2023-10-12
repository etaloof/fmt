use std::fmt::{Debug, Display, Formatter};

struct Disp<F>
where
    F: for<'a> Fn(&mut Formatter<'a>) -> std::fmt::Result,
{
    f: F,
}

impl<F> Disp<F>
where
    F: for<'a> Fn(&mut Formatter<'a>) -> std::fmt::Result,
{
    fn new(f: F) -> Self {
        Self { f }
    }

    fn dbg(self) -> Dbg<Self> {
        Dbg { f: self }
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

struct Dbg<F>
where
    F: Display,
{
    f: F,
}

impl<F: Display> Debug for Dbg<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.f, f)
    }
}

#[test]
fn t() {
    use std::io::Write;

    #[derive(Copy, Clone)]
    struct EchoPoint {
        row_idx: usize,
        ampl: usize,
    }
    struct ColView {
        col_idx: usize,
        echo_points: [EchoPoint; 2],
    }
    struct F {
        cols: Vec<[EchoPoint; 4]>,
    }
    impl F {
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

    let f = F { cols: vec![] };
    let option = Some(f);

    let mut file = std::io::sink();

    match option {
        None => writeln!(&mut file, ""),
        Some(f) => {
            writeln!(
                &mut file,
                "{}",
                Disp::new(|formatter| {
                    for c in f.cols() {
                        formatter
                            .debug_list()
                            .entries(c.echo_points.iter().map(|echo| {
                                Disp::new(|formatter| {
                                    formatter
                                        .debug_struct("Echo")
                                        .field("col_idx", &c.col_idx)
                                        .field("row_idx", &echo.row_idx)
                                        .field("ampl", &echo.ampl)
                                        .finish()
                                })
                                .dbg()
                            }))
                            .finish()?;
                    }
                    Ok(())
                })
            )
        },
    }
    .unwrap();
}
