pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn factorial_should_compute() {
        assert_eq!(factorial(5), 120);
    }
}
