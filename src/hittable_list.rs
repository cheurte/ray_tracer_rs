use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bbox: Aabb::new(),
        }
    }

    pub fn from(object: Box<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
            bbox: Aabb::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = Aabb::from_bbox(self.bbox, object.bounding_box());
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max();

        for object in self.objects.iter() {
            if object.hit(
                r,
                Interval::from(ray_t.min(), closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }

    fn bounding_box(&self) -> crate::aabb::Aabb {
        self.bbox
    }
}
