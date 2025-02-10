pub trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl Palindrome for String {
    fn is_palindrome(&self) -> bool {
        self.to_lowercase().chars().eq(self.chars().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!("".to_string().is_palindrome());
        assert!("a".to_string().is_palindrome());
        assert!("madam".to_string().is_palindrome());
        assert!("racecar".to_string().is_palindrome());
    }
}
