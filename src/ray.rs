use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Point3::zeros(),
            direction: Vec3::zeros(),
        }
    }
    pub fn from(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn origin(self) -> Point3 {
        self.origin
    }
    pub fn direction(self) -> Vec3 {
        self.direction
    }
    pub fn at(self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
