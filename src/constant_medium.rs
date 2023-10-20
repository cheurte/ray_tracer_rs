use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::{Interval, UNIVERSE},
    material::{Isotropic, Material},
    rtweekend::{random_double, INF},
    texture::Texture,
    vec3::Vec3,
};
use std::{marker::PhantomData, rc::Rc};

pub struct ConstantMedium {
    boundaries: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn from_texture(
        boundaries: Rc<dyn Hittable>,
        neg_inv_density: f64,
        texture: Rc<dyn Texture>,
    ) -> Self {
        Self {
            boundaries,
            neg_inv_density: -1.0 / neg_inv_density,
            phase_function: Rc::new(Isotropic::from_texture(texture)),
        }
    }
    pub fn from_color(boundaries: Rc<dyn Hittable>, neg_inv_density: f64, color: Color) -> Self {
        Self {
            boundaries,
            neg_inv_density: -1.0 / neg_inv_density,
            phase_function: Rc::new(Isotropic::from_color(color)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        // let enable_debug = false;
        // let debugging = enable_debug && random_double() < 0.00001;

        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundaries.hit(r, UNIVERSE, &mut rec1) {
            return false;
        }
        if !self
            .boundaries
            .hit(r, Interval::from(rec1.t + 0.00001, INF), &mut rec2)
        {
            return false;
        }

        if rec1.t < ray_t.min() {
            rec1.t = ray_t.min()
        }
        if rec2.t > ray_t.max() {
            rec2.t = ray_t.max()
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t <= 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundaries = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundaries {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::zeros();
        rec.front_face = true;
        rec.mat = self.phase_function.clone();
        true
    }
    fn bounding_box(&self) -> crate::aabb::Aabb {
        self.boundaries.bounding_box()
    }
}
