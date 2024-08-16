use core::f32::consts::PI;

#[derive(Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

pub struct Circle {
    pub width: f32,
    pub height: f32,
}

pub trait Shape {
    fn new(width: f32, height: f32) -> Self;
    fn area(&self) -> f32;
}

impl Shape for Rectangle {
    fn new(width: f32, height: f32) -> Rectangle {
        Rectangle { width, height }
    }
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn new(width: f32, height: f32) -> Circle {
        Circle { width, height }
    }
    fn area(&self) -> f32 {
        (self.width / 2.0).powf(2.0) * PI
    }
}

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod test {
    use shape::Rectangle;

    use crate::*;
    use std::mem;

    #[test]
    fn check_mem_size() {
        assert_eq!(mem::size_of::<Rectangle>(), 2 * mem::size_of::<f32>());
        assert_eq!(
            mem::size_of::<[Rectangle; 2]>(),
            2 * mem::size_of::<f32>() * 2
        );
    }
}
