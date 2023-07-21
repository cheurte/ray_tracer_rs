use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

struct HittableList {
    objects: Vec<HitRecord>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn from(obj: HitRecord) -> Self {
        Self { objects: vec![obj] }
    }
    pub fn add(&mut self, obj: HitRecord) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let temp_rec: &mut HitRecord = &mut HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in self.objects.iter() {
            if *obj.hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }
        return hit_anything;
    }
}
