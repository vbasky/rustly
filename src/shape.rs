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
