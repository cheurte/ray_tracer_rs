use crate::rtweekend::random_double;
use crate::texture::{SolidColor, Texture};
use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};
use std::rc::Rc;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

// #[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn from_color(albedo: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::from_color(albedo)),
        }
    }
    pub fn from_texture(a: Rc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::from(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: match fuzz {
                y if y < 1.0 => fuzz,
                _ => 1.0,
            },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.direction().unit_vector().reflect(&rec.normal);
        *scattered = Ray::from(
            rec.p,
            reflected + Vec3::random_unit_vector() * self.fuzz,
            r_in.time(),
        );
        *attenuation = self.albedo;
        scattered.direction().dot(rec.normal) > 0.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::ones();
        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.ir,
            false => self.ir,
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = -unit_direction.dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            match cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                true => unit_direction.reflect(&rec.normal),
                false => unit_direction.refract(&rec.normal, refraction_ratio),
            };

        *scattered = Ray::from(rec.p, direction, r_in.time());

        true
    }
}
