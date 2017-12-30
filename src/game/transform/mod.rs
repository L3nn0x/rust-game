extern crate cgmath;
extern crate sdl2;

pub struct Transform {
    mat: cgmath::Matrix3<f64>
}

impl Transform {
    pub fn translation(x: i32, y: i32) -> Transform {
        let mat = cgmath::Matrix3::new(1., 0., 0., 0., 1., 0., x as f64, y as f64, 1.);
        Transform {
            mat: mat
        }
    }

    pub fn rotation(degrees: f64) -> Transform {
        let mat = cgmath::Matrix3::from_angle_z(cgmath::Deg(degrees));
        Transform {
            mat: mat
        }
    }

    pub fn scaling(x: f64, y: f64) -> Transform {
        let mat = cgmath::Matrix3::new(x, 0., 0., 0., y, 0., 0., 0., 1.);
        Transform {
            mat: mat
        }
    }

    pub fn translate(self, x: i32, y: i32) -> Transform {
        self.combine(Transform::translation(x, y))
    }

    pub fn rotate(self, degrees: f64) -> Transform {
        self.combine(Transform::rotation(degrees))
    }

    pub fn scale(self, x: f64, y: f64) -> Transform {
        self.combine(Transform::scaling(x, y))
    }

    pub fn combine(self, other: Transform) -> Transform {
        Transform {
            mat: self.mat * other.mat
        }
    }

    pub fn transform_point(&self, point: sdl2::rect::Point) -> sdl2::rect::Point {
        let p = cgmath::Vector3{x: point.x() as f64, y: point.y() as f64, z: 1.};
        let p = self.mat * p;
        sdl2::rect::Point::new(p.x as i32, p.y as i32)
    }

    pub fn transform_rect(&self, rect: sdl2::rect::Rect) -> sdl2::rect::Rect {
        use super::sdl2::rect::Point;
        let p = [self.transform_point(Point::new(rect.x(), rect.y())),
                 self.transform_point(Point::new(rect.x(), rect.y() + rect.height() as i32)),
                 self.transform_point(Point::new(rect.x() + rect.width() as i32, rect.y())),
                 self.transform_point(Point::new(rect.x() + rect.width() as i32, rect.y() + rect.height() as i32))];

        // compute the bounding box of the transformed points
        let mut left = p[0].x();
        let mut top = p[0].y();
        let mut right = p[0].x();
        let mut bottom = p[0].y();
        for pp in p.iter() {
            if pp.x() < left {
                left = pp.x();
            } else if pp.x() > right {
                right = pp.x();
            }
            if pp.y() < top {
                top = pp.y();
            } else if pp.y() > bottom {
                bottom = pp.y();
            }
        }
        sdl2::rect::Rect::new(left, top, (right - left) as u32, (bottom - top) as u32)
    }
}