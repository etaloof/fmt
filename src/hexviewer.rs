/// Formats an iterator by piping it to `hexyl`.
use std::{
    cell::UnsafeCell,
    fmt::{Debug, Display, Formatter},
};
use std::{
    io::Write,
    process::{Command, Stdio},
};

pub struct Iter<I, S>
where
    I: AsRef<[u8]>,
    S: Iterator<Item = I>,
{
    item: UnsafeCell<Option<S>>,
}

impl<I, S> Iter<I, S>
where
    I: AsRef<[u8]>,
    S: Iterator<Item = I>,
{
    pub fn new(item: S) -> Self {
        Self {
            item: UnsafeCell::new(Some(item)),
        }
    }
}

impl<I: AsRef<[u8]>, S> Display for Iter<I, S>
where
    S: Iterator<Item = I>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut child = Command::new("hexyl")
            .stdin(Stdio::piped())
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let iter = std::mem::take(unsafe { &mut *self.item.get() });
        let mut stdin = child.stdin.take().unwrap();
        for item in iter.unwrap() {
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

impl<I: AsRef<[u8]>, S> Debug for Iter<I, S>
where
    S: Iterator<Item = I>,
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
