use std::rc::Rc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Material, Metal};
// use crate::material::Materials;
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

// #[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    // mat: Materials,
    mat: Rc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
    bbox: Aabb,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Point3::zeros(),
            radius: 0.0,
            mat: Rc::new(Metal::new(Color::zeros(), 1.5)),
            is_moving: false,
            center_vec: Vec3::zeros(),
            bbox: Aabb::new(),
        }
    }
    pub fn from(
        center: Point3,
        radius: f64,
        mat: Rc<dyn Material>,
        center_vec: Vec3,
        is_moving: bool,
        bbox: Aabb,
    ) -> Self {
        Self {
            center,
            radius,
            mat,
            is_moving,
            center_vec,
            bbox,
        }
    }
    pub fn new_stationnary(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let rvec = Vec3::from(radius, radius, radius);
        let bbox = Aabb::from_points(center - rvec, center + rvec);
        Self {
            center,
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::zeros(),
            bbox,
        }
    }
    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        let rvec = Vec3::from(radius, radius, radius);
        let box1 = Aabb::from_points(center1 - rvec, center2 + rvec);
        let box2 = Aabb::from_points(center2 - rvec, center1 + rvec);
        Self {
            center: center1,
            radius,
            mat,
            is_moving: true,
            center_vec: (center2 - center1),
            bbox: Aabb::from_bbox(box1, box2),
        }
    }
    fn sphere_center(&self, time: f64) -> Point3 {
        self.center + self.center_vec * time
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center = match self.is_moving {
            true => self.sphere_center(r.time()),
            false => self.center,
        };
        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - center) / self.radius;
        // rec.normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();
        // rec.color = self.color;
        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
