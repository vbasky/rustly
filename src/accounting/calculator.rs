use std::ops::Add;

#[allow(dead_code)]
pub fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
