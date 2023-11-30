use std::fmt::{Display, Formatter, Write};

// TODO: Optimize this to avoid the allocations.

pub struct Replace<T: Display, R1: Display, R2: Display> {
    item: T,
    replacement1: R1,
    replacement2: R2,
}

impl<T: Display, R1: Display, R2: Display> Replace<T, R1, R2> {
    pub fn new(item: T, replacement1: R1, replacement2: R2) -> Self {
        Self {
            item,
            replacement1,
            replacement2,
        }
    }
}

impl<T: Display, R1: Display, R2: Display> Display for Replace<T, R1, R2> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let item = stringify(&self.item)?;
        let r1 = stringify(&self.replacement1)?;
        let r2 = stringify(&self.replacement2)?;
        write!(f, "{}", item.replace(&r1, &r2))
    }
}

fn stringify<T: Display>(t: &T) -> Result<String, std::fmt::Error> {
    let mut s = String::new();
    write!(&mut s, "{}", t)?;
    Ok(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_replace() {
        let my_display = "aabcc";

        let replace = Replace::new(my_display, "a", "b");
        assert_eq!("bbbcc", replace.to_string());

        let replace = Replace::new(my_display, "a", "a");
        assert_eq!("aabcc", replace.to_string());

        let replace = Replace::new(my_display, "a", "aaa");
        assert_eq!("aaaaaabcc", replace.to_string());

        let replace = Replace::new(my_display, "b", " ");
        assert_eq!("aa cc", replace.to_string());

        let replace = Replace::new(my_display, "b", "a");
        assert_eq!("aaacc", replace.to_string());

        let replace = Replace::new(my_display, "a", "");
        assert_eq!("bcc", replace.to_string());

        let replace = Replace::new(my_display, "cc", "");
        assert_eq!("aab", replace.to_string());

        let replace = my_display;
        let replace = Replace::new(replace, "b", "a");
        let replace = Replace::new(replace, "c", "a");
        assert_eq!("aaaaa", replace.to_string());

        let replace = my_display;
        let replace = Replace::new(replace, "a", "");
        let replace = Replace::new(replace, "b", "");
        let replace = Replace::new(replace, "c", "");
        assert_eq!("", replace.to_string());
    }

    #[test]
    fn test_replace_a_to_b() {
        let my_display = "aabcc";

        let replace = Replace::new(my_display, "a", "b");
        assert_eq!("bbbcc", replace.to_string());
    }

    #[test]
    fn test_replace_a_to_a() {
        let my_display = "aabcc";

        let replace = Replace::new(my_display, "a", "a");
        assert_eq!("aabcc", replace.to_string());
    }

    #[test]
    fn test_replace_a_to_aaa() {
        let my_display = "aabcc";

        let replace = Replace::new(my_display, "a", "aaa");
        assert_eq!("aaaaaabcc", replace.to_string());
    }

    #[test]
    fn test_replace_b_to_space() {
        let my_display = "aabcc";

        let replace = Replace::new(my_display, "b", " ");
        assert_eq!("aa cc", replace.to_string());
    }

    #[test]
    fn test_replace_b_to_a() {
        let my_display = "aabcc";

        let replace = Replace::new(my_display, "b", "a");
        assert_eq!("aaacc", replace.to_string());
    }

    #[test]
    fn test_replace_a_to_empty_string() {
        let my_display = "aabcc";

        let replace = Replace::new(my_display, "a", "");
        assert_eq!("bcc", replace.to_string());
    }

    #[test]
    fn test_replace_c_to_empty_string() {
        let my_display = "aabcc";

        let replace = Replace::new(my_display, "c", "");
        assert_eq!("aab", replace.to_string());
    }

    #[test]
    fn test_replace_a_and_c_to_empty_string() {
        let my_display = "aabcc";

        let replace = my_display;
        let replace = Replace::new(replace, "a", "");
        let replace = Replace::new(replace, "c", "");
        assert_eq!("b", replace.to_string());
    }

    #[test]
    fn test_replace_everything_with_a() {
        let my_display = "aabcc";

        let replace = my_display;
        let replace = Replace::new(replace, "b", "a");
        let replace = Replace::new(replace, "c", "a");
        assert_eq!("aaaaa", replace.to_string());
    }

    #[test]
    fn test_replace_all_characters() {
        let my_display = "aabcc";

        let replace = my_display;
        let replace = Replace::new(replace, "a", "");
        let replace = Replace::new(replace, "b", "");
        let replace = Replace::new(replace, "c", "");
        assert_eq!("", replace.to_string());
    }
}
