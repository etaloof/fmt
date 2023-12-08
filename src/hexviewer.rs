/// Formats an iterator by piping it to `hexyl`.
use std::fmt::{Debug, Display, Formatter};
use std::{
    cell::Cell,
    io::Write,
    process::{Command, Stdio},
};

pub struct Iter<S>
where
    S: Iterator,
    S::Item: AsRef<[u8]>,
{
    item: Cell<Option<S>>,
}

impl<S> Iter<S>
where
    S: Iterator,
    S::Item: AsRef<[u8]>,
{
    pub fn new(item: S) -> Self {
        Self {
            item: Cell::new(Some(item)),
        }
    }
}

impl<S> Display for Iter<S>
where
    S: Iterator,
    S::Item: AsRef<[u8]>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut child = Command::new("hexyl")
            .stdin(Stdio::piped())
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let iter = self.item.replace(None).ok_or(Default::default())?;
        let mut stdin = child.stdin.take().unwrap();
        for item in iter {
            stdin.write(item.as_ref()).unwrap();
        }
        // to prevent deadlock
        drop(stdin);
        let output = child.wait_with_output().unwrap();
        let status = output.status;
        assert!(
            status.success(),
            "child process exited unsuccessfully: {}",
            status
        );
        let vec = output.stdout;
        let data = std::str::from_utf8(&vec).unwrap();
        f.write_str(data)?;
        Ok(())
    }
}

impl<S> Debug for Iter<S>
where
    S: Iterator,
    S::Item: AsRef<[u8]>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

trait Bytes {
    type Item: AsRef<[u8]>;
    type Iter: Iterator<Item = Self::Item>;
    fn to_bytes(self) -> Self::Iter;
}

impl<I, T> Bytes for T
where
    T: Iterator<Item = I>,
    I: AsRef<[u8]>,
{
    type Item = I;
    type Iter = T;
    fn to_bytes(self) -> Self::Iter {
        self
    }
}
