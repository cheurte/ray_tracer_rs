use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Materials;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Materials,
    is_moving: bool,
    center_vec: Vec3,
}

impl Sphere {
    pub fn from(
        center: Point3,
        radius: f64,
        mat: Materials,
        center_vec: Vec3,
        is_moving: bool,
    ) -> Self {
        Self {
            center,
            radius,
            mat,
            is_moving,
            center_vec,
        }
    }
    pub fn new_stationnary(center: Point3, radius: f64, mat: Materials) -> Self {
        Self {
            center,
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::zeros(),
        }
    }
    pub fn new_moving(center1: Point3, center2: Point3, radius: f64, mat: Materials) -> Self {
        Self {
            center: center1,
            radius,
            mat,
            is_moving: true,
            center_vec: (center2 - center1),
        }
    }
    fn sphere_center(self, time: f64) -> Point3 {
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
        rec.mat = self.mat;
        // rec.color = self.color;
        true
    }
}
