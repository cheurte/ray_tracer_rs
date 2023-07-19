mod vec3;

impl Ray {
    pub fn new(orig: Point3, dir: vec3::Vec3) -> Self {
        Self { orig, dir }
    }
    pub fn at(self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
