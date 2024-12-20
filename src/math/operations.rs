use num_bigint::BigInt;

pub fn factorial(num: u128) -> BigInt {
    (1..=num).product()
}

pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[allow(dead_code)]
pub fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

#[allow(dead_code)]
pub fn sum_of_squares(input: &[i32]) -> i32 {
    input.iter().map(|&i| i * i).sum()
}

#[allow(dead_code)]
pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

#[allow(dead_code)]
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    pub fn value(&mut self, args: u32) -> u32 {
        self.value.unwrap_or_else(|| {
            let v = (self.calculation)(args);
            self.value = Some(v);
            v
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn factorial_should_compute() {
        assert_eq!(factorial(5), BigInt::from(120));
    }

    #[test]
    fn factorial_of_a_bigint() {
        let expected_result =
            BigInt::parse_bytes(b"815915283247897734345611269596115894272000000000", 10).unwrap();
        assert_eq!(factorial(40), expected_result);
    }

    #[test]
    fn divide_by_zero() {
        assert_eq!(divide(2.0, 0.0), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let iter_v1 = v1.iter();
        let total: i32 = iter_v1.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn call_cacher_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(2);
        let v2 = c.value(3);
        assert_eq!(v1, 2);
        assert_ne!(v2, 3);
    }
}
