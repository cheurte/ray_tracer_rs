use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new() -> Self {
        Self {
            x: Interval::new(),
            y: Interval::new(),
            z: Interval::new(),
        }
    }
    pub fn from_bbox_vec(bbox: Self, vec: Vec3) -> Self {
        Self {
            x: bbox.x + vec.x(),
            y: bbox.y + vec.y(),
            z: bbox.z + vec.z(),
        }
    }
    pub fn pad(&self) -> Self {
        let delta = 0.0001;
        let x = match self.x().size() >= delta {
            true => self.x,
            false => self.x.expand(delta),
        };
        let y = match self.y().size() >= delta {
            true => self.y,
            false => self.y.expand(delta),
        };
        let z = match self.z().size() >= delta {
            true => self.z,
            false => self.z.expand(delta),
        };
        Self { x, y, z }
    }
    pub fn from_points(a: Point3, b: Point3) -> Self {
        Self {
            x: Interval::from((a[0] as f64).min(b[0]), (a[0] as f64).max(b[0])),
            y: Interval::from((a[1] as f64).min(b[1]), (a[1] as f64).max(b[1])),
            z: Interval::from((a[2] as f64).min(b[2]), (a[2] as f64).max(b[2])),
        }
    }
    pub fn from_bbox(box0: Aabb, box1: Aabb) -> Self {
        Self {
            x: Interval::from_intervals(box0.x(), box1.x()),
            y: Interval::from_intervals(box0.y(), box1.y()),
            z: Interval::from_intervals(box0.z(), box1.z()),
        }
    }
    pub fn axis(&self, n: i32) -> Interval {
        match n {
            1 => return self.y,
            2 => return self.z,
            _ => self.x,
        }
    }
    pub fn x(self) -> Interval {
        self.x
    }
    pub fn y(self) -> Interval {
        self.y
    }
    pub fn z(self) -> Interval {
        self.z
    }
    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let orig = r.origin()[a];

            let mut t0 = (self.axis(a).min() - orig) * inv_d;
            let mut t1 = (self.axis(a).max() - orig) * inv_d;

            if inv_d < 0.0 {
                let save = t0;
                t0 = t1;
                t1 = save;
            }
            if t0 > ray_t.min() {
                ray_t.modify_min(t0);
            }
            if t1 < ray_t.max() {
                ray_t.modify_max(t1);
            }
            if ray_t.max() <= ray_t.min() {
                return false;
            }
        }
        true
    }
}

impl ops::Add<Vec3> for Aabb {
    type Output = Aabb;
    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb::from_bbox_vec(self, rhs)
    }
}
