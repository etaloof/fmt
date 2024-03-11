use std::fmt::Display;

pub trait DisplayProducer {
    type DisplayAdapter: Display;

    fn display(&self) -> Self::DisplayAdapter;
}

impl<T: Display, F: Fn() -> T> DisplayProducer for F {
    type DisplayAdapter = T;

    fn display(&self) -> Self::DisplayAdapter {
        self()
    }
}

struct FnProducer<F, T>(F)
where
    F: Fn() -> T,
    T: Display;

impl<F, T> FnProducer<F, T>
where
    F: Fn() -> T,
    T: Display,
{
    fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F, T> DisplayProducer for FnProducer<F, T>
where
    F: Fn() -> T,
    T: Display,
{
    type DisplayAdapter = T;

    fn display(&self) -> Self::DisplayAdapter {
        self.0()
    }
}

struct CopyProducer<T>(T)
where
    T: Copy + Display;

impl<T> CopyProducer<T>
where
    T: Copy + Display,
{
    fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T> DisplayProducer for CopyProducer<T>
where
    T: Copy + Display,
{
    type DisplayAdapter = T;

    fn display(&self) -> Self::DisplayAdapter {
        self.0
    }
}

struct CloneProducer<T>(T)
where
    T: Clone + Display;

impl<T> CloneProducer<T>
where
    T: Clone + Display,
{
    fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T> DisplayProducer for CloneProducer<T>
where
    T: Clone + Display,
{
    type DisplayAdapter = T;

    fn display(&self) -> Self::DisplayAdapter {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::joined::DisplayIteratorJoined;

    use super::*;

    fn fmt_to_string<T: Display>(t: T) -> Result<String, std::fmt::Error> {
        use std::fmt::Write;
        let mut s = String::new();
        write!(&mut s, "{}", t)?;
        Ok(s)
    }

    #[test]
    fn fn_producer() {
        let p = FnProducer::new(|| "42");
        assert_eq!("42", p.display().to_string());
        assert_eq!("42", p.display().to_string());

        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let p = FnProducer::new(|| {
            let iter = data.iter().filter(|x| **x > 4);
            DisplayIteratorJoined::new(iter, ",")
        });
        assert_eq!("5, 6, 7, 8", p.display().to_string());
        assert_eq!("5, 6, 7, 8", p.display().to_string());
    }

    #[test]
    fn copy_producer() {
        let producer = CopyProducer::new("42");
        assert_eq!("42", producer.display().to_string());
        assert_eq!("42", producer.display().to_string());
    }

    #[test]
    fn clone_producer() {
        let producer = CloneProducer::new("42");
        assert_eq!("42", producer.display().to_string());
        assert_eq!("42", producer.display().to_string());

        let producer = CloneProducer::new("42".to_string());
        assert_eq!("42", producer.display().to_string());
        assert_eq!("42", producer.display().to_string());
    }
}
