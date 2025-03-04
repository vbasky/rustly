use core::f32::consts::PI;
use std::fmt::Display;

#[derive(Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Circle {
    pub width: f32,
    pub height: f32,
}

#[allow(dead_code)]
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

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Circle of width {:.1} and height {:.1}",
            self.width, self.height
        )
    }
}

#[allow(dead_code)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PointXY<X1, Y1> {
    pub x: X1,
    pub y: Y1,
}

#[allow(dead_code)]
impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

#[allow(dead_code)]
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[allow(dead_code)]
impl<X1, Y1> PointXY<X1, Y1> {
    pub fn mixup<X2, Y2>(self, other: PointXY<X2, Y2>) -> PointXY<X1, Y2> {
        PointXY {
            x: self.x,
            y: other.y,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub enum SquareContent {
    #[default]
    Empty,
    X,
    O,
}

impl From<u8> for SquareContent {
    fn from(c: u8) -> Self {
        match c {
            0 => SquareContent::Empty,
            1 => SquareContent::X,
            2 => SquareContent::O,
            _ => panic!("Invalid square content"),
        }
    }
}

impl From<SquareContent> for u8 {
    fn from(c: SquareContent) -> Self {
        match c {
            SquareContent::Empty => 0,
            SquareContent::X => 1,
            SquareContent::O => 2,
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem;

    use crate::generics::shape::{Circle, Rectangle, SquareContent};

    #[test]
    fn check_mem_size() {
        assert_eq!(mem::size_of::<Rectangle>(), 2 * mem::size_of::<f32>());
        assert_eq!(
            mem::size_of::<[Rectangle; 2]>(),
            2 * mem::size_of::<f32>() * 2
        );
        assert_eq!(mem::size_of::<Rectangle>(), 2 * mem::size_of::<f32>());
    }

    #[test]
    fn test_square_content() {
        let square = SquareContent::default();
        assert_eq!(square, SquareContent::Empty);
    }

    #[test]
    fn test_from() {
        assert_eq!(SquareContent::from(1), SquareContent::X);
    }

    #[test]
    fn test_circle_is_printable() {
        let circle = Circle {
            width: 10.0,
            height: 12.0,
        };
        assert_eq!(circle.to_string(), "Circle of width 10.0 and height 12.0");
    }
}
