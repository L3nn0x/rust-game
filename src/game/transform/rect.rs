extern crate std;
extern crate sdl2;

pub struct Rect {
    pub top: i32,
    pub left: i32,
    pub width: i32,
    pub height: i32
}

impl Rect {
    pub fn new(top: i32, left: i32, width: i32, height: i32) -> Rect {
        Rect {
            top: top,
            left: left,
            width: width,
            height: height
        }
    }
}

impl From<(i32, i32, i32, i32)> for Rect {
    fn from(tuple: (i32, i32, i32, i32)) -> Self {
        std::ops::Fn::call(&Rect::new, tuple)
    }
}

impl From<sdl2::rect::Rect> for Rect {
    fn from(rect: sdl2::rect::Rect) -> Self {
        Rect::new(rect.x(), rect.y(), rect.width() as i32, rect.height() as i32)
    }
}