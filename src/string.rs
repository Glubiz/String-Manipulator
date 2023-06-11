use crate::traits::{ConvertCase, Padding, Search, Split, Trim};

impl Trim for String {
    fn trim(&self) -> String {
        self.clone().trim_start().trim_end().to_string()
    }
}

impl ConvertCase for String {
    fn to_camel_case(&self) -> String {
        let mut s = String::new();
        let mut capitalize = false;
        for c in self.chars() {
            if c == '_' || c == '-' || c == ' ' {
                capitalize = true;
            } else if capitalize {
                s.push(c.to_ascii_uppercase());
                capitalize = false;
            } else {
                s.push(c);
            }
        }
        s
    }

    fn to_snake_case(&self) -> String {
        let mut s = String::new();
        for c in self.chars() {
            if c.is_ascii_uppercase() {
                s.push('_');
                s.push(c.to_ascii_lowercase());
            } else if c == '-' || c == ' ' {
                s.push('_');
            } else {
                s.push(c);
            }
        }
        s
    }

    fn to_kebab_case(&self) -> String {
        let mut s = String::new();
        for c in self.chars() {
            if c.is_ascii_uppercase() {
                s.push('-');
                s.push(c.to_ascii_lowercase());
            } else if c == '_' || c == ' ' {
                s.push('-');
            } else {
                s.push(c);
            }
        }
        s
    }

    fn to_upper(&self) -> String {
        self.to_ascii_uppercase()
    }

    fn to_lower(&self) -> String {
        self.to_ascii_lowercase()
    }
}

impl Padding for String {
    fn pad_left(&self, length: usize, pad: char) -> String {
        let mut s = String::new();
        for _ in 0..length - self.len() {
            s.push(pad);
        }
        s.push_str(self);
        s
    }

    fn pad_right(&self, length: usize, pad: char) -> String {
        let mut s = String::new();
        s.push_str(self);
        for _ in 0..length - self.len() {
            s.push(pad);
        }
        s
    }

    fn pad(&self, n: usize, c: char) -> Self {
        let mut s = String::new();
        for _ in 0..n {
            s.push(c);
        }
        s.push_str(self);
        for _ in 0..n {
            s.push(c);
        }
        s
    }
}

impl Split for String {
    fn split(&self, delimiter: char) -> Vec<String> {
        let mut v = Vec::new();
        let mut s = String::new();
        for c in self.chars() {
            if c == delimiter {
                v.push(s);
                s = String::new();
            } else {
                s.push(c);
            }
        }
        v.push(s);
        v
    }
}

impl Search for String {
    fn find_first_of(&self, s: &str) -> Option<usize> {
        for (i, c) in self.chars().enumerate() {
            if s.contains(c) {
                return Some(i);
            }
        }
        None
    }

    fn find_last_of(&self, s: &str) -> Option<usize> {
        for (i, c) in self.chars().rev().enumerate() {
            if s.contains(c) {
                return Some(self.len() - i - 1);
            }
        }
        None
    }

    fn find_first_not_of(&self, s: &str) -> Option<usize> {
        for (i, c) in self.chars().enumerate() {
            if !s.contains(c) {
                return Some(i);
            }
        }
        None
    }

    fn find_last_not_of(&self, s: &str) -> Option<usize> {
        for (i, c) in self.chars().rev().enumerate() {
            if !s.contains(c) {
                return Some(self.len() - i - 1);
            }
        }
        None
    }

    fn matches_count(&self, s: &str) -> usize {
        self.matches(s).count()
    }

    fn matches_indices(&self, s: &str) -> Vec<usize> {
        let mut v = Vec::new();
        let mut pos = 0;
        while let Some(start) = self[pos..].find(s) {
            v.push(pos + start);
            pos += start + s.len();
        }
        v
    }

    fn matches_indices_count(&self, s: &str) -> usize {
        let mut count = 0;
        let mut pos = 0;
        while let Some(start) = self[pos..].find(s) {
            count += 1;
            pos += start + s.len();
        }
        count
    }

    fn contains_any(&self, s: &str) -> bool {
        for c in self.chars() {
            if s.contains(c) {
                return true;
            }
        }
        false
    }

    fn contains_all(&self, s: &str) -> bool {
        for c in s.chars() {
            if !self.contains(c) {
                return false;
            }
        }
        true
    }

    fn contains_none(&self, s: &str) -> bool {
        for c in self.chars() {
            if s.contains(c) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim() {
        let s = String::from("  hello world  ");
        assert_eq!(s.trim(), "hello world");
    }

    #[test]
    fn test_to_camel_case() {
        let s = String::from("hello_world");
        assert_eq!(s.to_camel_case(), "helloWorld");
    }

    #[test]
    fn test_to_camel_case_space() {
        let s = String::from("hello world");
        assert_eq!(s.to_camel_case(), "helloWorld");
    }

    #[test]
    fn test_to_snake_case() {
        let s = String::from("helloWorld");
        assert_eq!(s.to_snake_case(), "hello_world");
    }

    #[test]
    fn test_to_snake_case_space() {
        let s = String::from("hello world");
        assert_eq!(s.to_snake_case(), "hello_world");
    }

    #[test]
    fn test_to_kebab_case() {
        let s = String::from("helloWorld");
        assert_eq!(s.to_kebab_case(), "hello-world");
    }

    #[test]
    fn test_to_upper() {
        let s = String::from("hello world");
        assert_eq!(s.to_upper(), "HELLO WORLD");
    }

    #[test]
    fn test_to_lower() {
        let s = String::from("HELLO WORLD");
        assert_eq!(s.to_lower(), "hello world");
    }

    #[test]
    fn test_pad_left() {
        let s = String::from("hello");
        assert_eq!(s.pad_left(10, ' '), "     hello");
    }

    #[test]
    fn test_pad_right() {
        let s = String::from("hello");
        assert_eq!(s.pad_right(10, ' '), "hello     ");
    }

    #[test]
    fn test_pad() {
        let s = String::from("hello");
        assert_eq!(s.pad(2, ' '), "  hello  ");
    }

    #[test]
    fn test_split() {
        let s = String::from("hello world");
        assert_eq!(s.split(' '), vec!["hello", "world"]);
    }

    #[test]
    fn test_find_first_of() {
        let s = String::from("hello world");
        assert_eq!(s.find_first_of("o"), Some(4));
    }

    #[test]
    fn test_find_last_of() {
        let s = String::from("hello world");
        assert_eq!(s.find_last_of("o"), Some(7));
    }

    #[test]
    fn test_find_first_not_of() {
        let s = String::from("hello world");
        assert_eq!(s.find_first_not_of("hello"), Some(5));
    }

    #[test]
    fn test_find_last_not_of() {
        let s = String::from("hello world");
        assert_eq!(s.find_last_not_of("world"), Some(5));
    }

    #[test]
    fn test_matches_count() {
        let s = String::from("hello world");
        assert_eq!(s.matches_count("hello"), 1);
    }

    #[test]
    fn test_matches_indices() {
        let s = String::from("hello world");
        assert_eq!(s.matches_indices("hello"), vec![0]);
    }

    #[test]
    fn test_matches_indices_count() {
        let s = String::from("hello world");
        assert_eq!(s.matches_indices_count("hello"), 1);
    }

    #[test]
    fn test_contains_any() {
        let s = String::from("hello world");
        assert!(s.contains_any("hello"));
    }

    #[test]
    fn test_contains_all() {
        let s = String::from("hello world");
        assert!(s.contains_all("hello"));
    }

    #[test]
    fn test_contains_none() {
        let s = String::from("hello world");
        assert!(!s.contains_none("hello"));
    }
}
