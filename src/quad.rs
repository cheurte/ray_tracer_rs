use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Vec3,
    Point3,
};
use std::rc::Rc;
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Rc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn from(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let n = u.cross(v);
        let normal = n.unit_vector();
        let d = normal.dot(q);
        let w = n / n.dot(n);
        Self {
            q,
            u,
            v,
            mat,
            bbox: Self::set_bounding_box(q, u, v),
            d,
            normal,
            w,
        }
    }
    pub fn set_bounding_box(q: Point3, u: Vec3, v: Vec3) -> Aabb {
        Aabb::from_points(q, q + u + v).pad()
    }
    pub fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.
        if (a < 0.0 || 1.0 < a || b < 0.0 || 1.0 < b) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let dnom = self.normal.dot(r.direction());
        // No hit if the ray is parallel to the plane.
        if dnom.abs() < 1e-8 {
            return false;
        }
        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - self.normal.dot(r.origin())) / dnom;
        if !ray_t.contains(t) {
            return false;
        }
        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let plana_hipt_vector = intersection - self.q;
        let alpha = self.w.dot(plana_hipt_vector.cross(self.v));
        let betha = self.w.dot(self.u.cross(plana_hipt_vector));
        if !Self::is_interior(alpha, betha, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);
        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
