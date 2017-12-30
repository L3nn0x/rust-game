extern crate cgmath;
extern crate sdl2;

mod rect;
mod point;

pub struct Transform(cgmath::Matrix3<f64>);

impl Transform {
    pub fn from_identity() -> Transform {
        Transform(cgmath::Matrix3::new(1., 0., 0., 0., 1., 0., 0., 0., 1.))
    }

    pub fn from_translation(x: i32, y: i32) -> Transform {
        let mat = cgmath::Matrix3::new(1., 0., 0., 0., 1., 0., x as f64, y as f64, 1.);
        Transform(mat)
    }

    pub fn from_rotation(degrees: f64) -> Transform {
        let mat = cgmath::Matrix3::from_angle_z(cgmath::Deg(degrees));
        Transform(mat)
    }

    pub fn from_scale(x: f64, y: f64) -> Transform {
        let mat = cgmath::Matrix3::new(x, 0., 0., 0., y, 0., 0., 0., 1.);
        Transform(mat)
    }

    pub fn from_scale_with_center(x: f64, y: f64, x_center: i32, y_center: i32) -> Transform {
        Transform::from_translation(-x_center, -y_center).with_scale(x, y).with_translation(x_center, y_center)
    }

    pub fn with_translation(self, x: i32, y: i32) -> Transform {
        self.combine(Transform::from_translation(x, y))
    }

    pub fn with_rotation(self, degrees: f64) -> Transform {
        self.combine(Transform::from_rotation(degrees))
    }

    pub fn with_scale(self, x: f64, y: f64) -> Transform {
        self.combine(Transform::from_scale(x, y))
    }

    pub fn with_scale_with_center(self, x: f64, y: f64, x_center: i32, y_center: i32) -> Transform {
        self.with_translation(-x_center, -y_center).with_scale(x, y).with_translation(x_center, y_center)
    }

    pub fn combine(self, other: Transform) -> Transform {
        Transform(self.0 * other.0)
    }

    pub fn transform_point(&self, point: point::Point) -> point::Point {
        let p = cgmath::Vector3{x: point.x as f64, y: point.y as f64, z: 1.};
        let p = self.0 * p;
        point::Point::new(p.x as i32, p.y as i32)
    }

    pub fn transform_rect(&self, rect: rect::Rect) -> rect::Rect {
        use self::point::Point;
        let p = (self.transform_point(Point::new(rect.left, rect.top)),
                 self.transform_point(Point::new(rect.left, rect.top + rect.height)),
                 self.transform_point(Point::new(rect.left + rect.width, rect.top)),
                 self.transform_point(Point::new(rect.left + rect.width, rect.top + rect.height)));

        // compute the bounding box of the transformed points
        let mut left = p.0.x;
        let mut top = p.0.y;
        let mut right = p.0.x;
        let mut bottom = p.0.y;
        for point in &[p.0, p.1, p.2, p.3] {
            if point.x < left {
                left = point.x;
            } else if point.x > right {
                right = point.x;
            }
            if point.y < top {
                top = point.y;
            } else if point.y > bottom {
                bottom = point.y;
            }
        }
        rect::Rect::new(left, top, right - left, bottom - top)
    }
}

impl From<point::Point> for Transform {
    fn from(p: point::Point) -> Self {
        Transform::from_translation(p.x, p.y)
    }
}