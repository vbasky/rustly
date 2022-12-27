pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn factorial_should_compute() {
        assert_eq!(factorial(5), 120);
    }
}
