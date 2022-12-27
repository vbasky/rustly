use core::f32::consts::PI;

pub struct Rectangle {
    pub length: f32,
    pub height: f32,
}

pub struct Circle {
    length: f32,
    height: f32,
}

pub trait Shape {
    fn new(length: f32, height: f32) -> Self;
    fn area(&self) -> f32;
}

impl Shape for Rectangle {
    fn new(length: f32, height: f32) -> Rectangle {
        return Rectangle { length, height };
    }
    fn area(&self) -> f32 {
        return self.length * self.height;
    }
}

impl Shape for Circle {
    fn new(length: f32, height: f32) -> Circle {
        return Circle { length, height };
    }
    fn area(&self) -> f32 {
        return (self.length / 2.0).powf(2.0) * PI;
    }
}
