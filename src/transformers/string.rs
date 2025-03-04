use std::fmt::Display;

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[allow(dead_code)]
pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[allow(dead_code)]
pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn is_palindrome<T>(s: &T) -> bool
where
    T: AsRef<str>,
{
    let s = s.as_ref();
    // s.chars().eq(s.chars().rev())
    let mut chars = s.chars();
    let mut rev_chars = s.chars().rev();
    loop {
        match (chars.next(), rev_chars.next()) {
            (Some(c), Some(rc)) if c == rc => continue,
            (None, None) => return true,
            _ => return false,
        }
    }
}

#[allow(dead_code)]
pub fn is_all_palindrome(strings: Vec<&str>) -> bool {
    strings.iter().all(is_palindrome)
}

#[allow(dead_code)]
pub fn calculate_length(s: &str) -> usize {
    s.len()
}

#[allow(dead_code)]
pub fn dangle() -> String {
    String::from("hello")
}

#[allow(dead_code)]
pub fn heart() -> String {
    // some bytes, in a vector
    let sparkle_heart = vec![240, 159, 146, 150];

    // We know these bytes are valid, so we'll use `unwrap()`.
    String::from_utf8(sparkle_heart).unwrap()
}

pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

#[allow(dead_code)]
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            // TODO: This is a bit of a hack, we should probably return None if the remainder is empty
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heart() {
        let expected = "💖";
        assert_eq!(heart(), expected);
    }

    #[test]
    fn test_calculate_length() {
        let test_string = String::from("hello");
        let expected_length = 5;
        assert_eq!(calculate_length(&test_string), expected_length);
    }

    #[test]
    fn test_palindrome() {
        let test_string = "racecar";
        assert_eq!(is_palindrome(&test_string), true);
    }

    #[test]
    fn test_all_palindrome() {
        let test_strings = vec!["racecar", "madam", "level"];
        assert_eq!(is_all_palindrome(test_strings), true);
    }

    #[test]
    fn test_first_word() {
        let test_string = String::from("hello world");
        let expected = "hello";
        assert_eq!(first_word(&test_string), expected);
    }

    #[test]
    fn test_longest() {
        let x = "test hello";
        let y = "world";
        let expected = "test hello";
        assert_eq!(longest(x, y), expected);
    }

    #[test]
    fn test_longest_with_an_announcement() {
        let x = "test hello";
        let y = "world";
        let ann = "Announcement!";
        let expected = "test hello";
        assert_eq!(longest_with_an_announcement(x, y, ann), expected);
    }

    #[test]
    fn test_string_split() {
        let haystack = "a b c d e";
        let delimiter = " ";
        let mut split = StrSplit::new(haystack, delimiter);
        assert_eq!(split.next(), Some("a"));
        assert_eq!(split.next(), Some("b"));
        assert_eq!(split.next(), Some("c"));
        assert_eq!(split.next(), Some("d"));
        assert_eq!(split.next(), Some("e"));
        assert_eq!(split.next(), None);
    }
}
