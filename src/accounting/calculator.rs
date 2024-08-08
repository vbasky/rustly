use std::ops::Add;

pub fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
