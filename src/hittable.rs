use std::rc::Rc;

use crate::aabb::Aabb;
use crate::color::Color;
use crate::interval::Interval;
use crate::material::{Material, Metal};
use crate::ray::Ray;
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
