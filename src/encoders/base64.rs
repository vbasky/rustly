pub fn encode_base64(data: &str) -> String {
    base64::encode(data)
}

#[cfg(test)]
mod tests {
    use super::encode_base64;

    #[test]
    fn test_encode_base64() {
        assert_eq!(encode_base64("hello world"), "aGVsbG8gd29ybGQ=");
    }
}
