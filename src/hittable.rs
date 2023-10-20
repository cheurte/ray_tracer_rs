use std::os::unix::prelude::DirEntryExt;
use std::rc::Rc;

use crate::aabb::Aabb;
use crate::color::Color;
use crate::interval::Interval;
use crate::material::{Material, Metal};
use crate::ray::Ray;
use crate::rtweekend::{degrees2radians, INF};
use crate::vec3::{Point3, Vec3};

// #[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::zeros(),
            normal: Vec3::zeros(),
            mat: Rc::new(Metal::new(Color::zeros(), 0.0)),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            // color: Color::zeros(),
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(*outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => *outward_normal * (-1.0),
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Aabb;
}

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn from(object: Rc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            object: object.clone(),
            offset: displacement,
            bbox: object.bounding_box() + displacement,
        }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_r = Ray::from(r.origin() - self.offset, r.direction(), r.time());

        // Determine where (if any) an intersection occurs along the offset ray
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        // Move the intersection point forwards by the offset
        rec.p += self.offset;

        true
    }
}

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn from(p: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees2radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = p.bounding_box();

        let mut min = Point3::from(INF, INF, INF);
        let mut max = Point3::from(-INF, -INF, -INF);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = bbox.x().max() * i as f64 + (1.0 - i as f64) * bbox.x().min();
                    let y = bbox.y().max() * j as f64 + (1.0 - j as f64) * bbox.y().min();
                    let z = bbox.z().max() * k as f64 + (1.0 - k as f64) * bbox.z().min();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -1.0 * sin_theta * x + cos_theta * z;

                    let tester = Vec3::from(newx, y, newz);

                    for c in 0..3 {
                        min[c] = (min[c] as f64).min(tester[c]);
                        max[c] = (max[c] as f64).max(tester[c]);
                    }
                }
            }
        }
        Self {
            object: p,
            sin_theta,
            cos_theta,
            bbox: Aabb::from_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::from(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }
        let mut p = rec.p;
        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -1.0 * self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        let mut normal = rec.normal;
        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -1.0 * self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.normal = normal;
        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
