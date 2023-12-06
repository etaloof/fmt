use std::fmt::{Display, Formatter};

use serde::Serialize;

#[derive(Copy, Clone)]
pub struct SerdeJson<'a, S: Serialize + ?Sized> {
    item: &'a S,
    pretty: bool,
}

impl<'a, S> SerdeJson<'a, S>
where
    S: Serialize + ?Sized,
{
    pub fn new(item: &'a S) -> Self {
        Self {
            item,
            pretty: false,
        }
    }

    pub fn pretty(self, pretty: bool) -> Self {
        Self { pretty, ..self }
    }
}

impl<'a, S> Display for SerdeJson<'a, S>
where
    S: Serialize + ?Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = if self.pretty {
            serde_json::to_string_pretty(self.item)
        } else {
            serde_json::to_string(self.item)
        };
        let s = match s {
            Ok(ok) => ok,
            Err(_) => {
                return Err(Default::default());
            },
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Person {
        age: u8,
        name: String,
    }

    #[test]
    fn test_slice() {
        let slice = SerdeJson::new(&["a", "b", "c"]);
        assert_eq!(slice.to_string(), r#"["a","b","c"]"#);
    }

    #[test]
    fn test_slice_with_numbers() {
        let slice = SerdeJson::new(&[1, 2, 3]);
        assert_eq!(slice.to_string(), r#"[1,2,3]"#);
    }

    #[test]
    fn test_empty_slice() {
        let slice = SerdeJson::<[i32]>::new(&[]);
        assert_eq!(slice.to_string(), "[]");
    }

    #[test]
    fn test_single_element_slice() {
        let slice = SerdeJson::new(&["a"]);
        assert_eq!(slice.to_string(), r#"["a"]"#);
    }

    #[test]
    fn test_person() {
        let person = Person {
            name: String::from("<NAME>"),
            age: 42,
        };
        let person = SerdeJson::new(&person);
        assert_eq!(person.to_string(), r#"{"age":42,"name":"<NAME>"}"#);
    }

    #[test]
    fn test_slice_pretty() {
        let slice = SerdeJson::new(&["a", "b", "c"]).pretty(true);
        assert_eq!(
            slice.to_string(),
            r#"[
  "a",
  "b",
  "c"
]"#
        );
    }

    #[test]
    fn test_slice_with_numbers_pretty() {
        let slice = SerdeJson::new(&[1, 2, 3]).pretty(true);
        assert_eq!(
            slice.to_string(),
            r#"[
  1,
  2,
  3
]"#
        );
    }

    #[test]
    fn test_empty_slice_pretty() {
        let slice = SerdeJson::<[i32]>::new(&[]).pretty(true);
        assert_eq!(slice.to_string(), "[]");
    }

    #[test]
    fn test_single_element_slice_pretty() {
        let slice = SerdeJson::new(&["a"]).pretty(true);
        assert_eq!(
            slice.to_string(),
            r#"[
  "a"
]"#
        );
    }

    #[test]
    fn test_person_pretty() {
        let person = Person {
            name: String::from("<NAME>"),
            age: 42,
        };
        let person = SerdeJson::new(&person).pretty(true);
        assert_eq!(
            person.to_string(),
            r#"{
  "age": 42,
  "name": "<NAME>"
}"#
        );
    }
}
