pub mod count;
//pub mod hexiter;
mod adhoc;
pub mod custom;
pub mod hex;
pub mod hexslice;
pub mod hexviewer;
pub mod pad;
pub mod slice;
mod utils;

pub enum Align {
    Left,
    Right,
}

macro_rules! disp {
    ($d:expr) => {{
        $crate::adhoc::Disp::new(|f| $d)
    }};
}
