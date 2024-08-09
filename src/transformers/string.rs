use std::fmt::Display;

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

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

pub fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn dangle() -> String {
    let s = String::from("hello");
    s
}

pub fn heart() -> String {
    // some bytes, in a vector
    let sparkle_heart = vec![240, 159, 146, 150];

    // We know these bytes are valid, so we'll use `unwrap()`.
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

    sparkle_heart
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
}
