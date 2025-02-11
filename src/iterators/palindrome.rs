#[allow(dead_code)]
pub trait Palindrome {
    fn is_palindrome(val: impl Into<String>) -> bool;
}

impl Palindrome for String {
    fn is_palindrome(val: impl Into<String>) -> bool {
        let val: String = val.into();
        val.to_lowercase().chars().eq(val.chars().rev())
    }
}

#[cfg(test)]
mod tests {

    use super::Palindrome;

    #[test]
    fn test_is_palindrome() {
        assert!(String::is_palindrome("".to_string()));
        assert!(String::is_palindrome(String::from("a")));
        assert!(String::is_palindrome("madam"));
        assert!(String::is_palindrome("racecar"));
    }
}
