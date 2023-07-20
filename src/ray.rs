use crate::vec3::Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}
fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> bool {
    let oc: Vec3 = r.origin() - *center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self { orig, dir }
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn origin(&self) -> Vec3 {
        self.orig
    }
    pub fn at(self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
    pub fn ray_color(self) -> Vec3 {
        if hit_sphere(&Vec3::from(0, 0, -1), 0.5, &self) {
            return Vec3::new(1.0, 0.0, 0.0);
        }
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}
