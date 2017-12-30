extern crate std;
extern crate sdl2;

pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        std::ops::Fn::call(&Point::new, tuple)
    }
}

impl From<sdl2::rect::Point> for Point {
    fn from(point: sdl2::rect::Point) -> Self {
        Point::new(point.x(), point.y())
    }
}