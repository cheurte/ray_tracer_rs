use std::cmp::Ordering;
use std::rc::Rc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::random_int;

pub struct BvhNode {
    left: Option<Rc<dyn Hittable>>,
    right: Option<Rc<dyn Hittable>>,
    bbox: Aabb,
}
//
impl BvhNode {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            bbox: Aabb::new(),
        }
    }
    pub fn from_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        BvhNode::from(&mut list.objects, 0, len)
    }
    fn from(src_objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut objects = src_objects.clone();

        let axis = random_int(0, 2);

        let comparator: fn(&Rc<dyn Hittable>, &Rc<dyn Hittable>) -> Ordering = match axis {
            0 => BvhNode::box_x_compare,
            1 => BvhNode::box_y_compare,
            _ => BvhNode::box_z_compare,
        };
        let object_span = end - start;

        if object_span == 1 {
            return BvhNode {
                left: Some(objects[start].clone()),
                right: Some(objects[start].clone()),
                bbox: Aabb::from_bbox(
                    objects[start].clone().bounding_box(),
                    objects[start].clone().bounding_box(),
                ),
            };
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Equal {
                return BvhNode {
                    left: Some(objects[start].clone()),
                    right: Some(objects[start + 1].clone()),
                    bbox: Aabb::from_bbox(
                        objects[start].clone().bounding_box(),
                        objects[start + 1].clone().bounding_box(),
                    ),
                };
            } else {
                return BvhNode {
                    left: Some(objects[start + 1].clone()),
                    right: Some(objects[start].clone()),
                    bbox: Aabb::from_bbox(
                        objects[start + 1].clone().bounding_box(),
                        objects[start].clone().bounding_box(),
                    ),
                };
            }
        }
        objects[start..end].sort_by(|a, b| comparator(a, b));

        let mid = start + object_span / 2;
        let left_node = BvhNode::from(&mut objects, start, mid);
        let right_node = BvhNode::from(&mut objects, mid, end);

        let left_bbox = left_node.bbox.clone();
        let right_bbox = right_node.bbox.clone();

        BvhNode {
            left: Some(Rc::new(left_node)),
            right: Some(Rc::new(right_node)),
            bbox: Aabb::from_bbox(left_bbox, right_bbox),
        }
    }
    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis_index: i32) -> Ordering {
        // a.bounding_box().axis(axis_index).min() < b.bounding_box().axis(axis_index).max()
        // let a_bbox = a.bounding_box();
        // let b_bbox = b.bounding_box();

        a.bounding_box()
            .axis(axis_index)
            .min()
            .partial_cmp(&b.bounding_box().axis(axis_index).min())
            .unwrap_or(Ordering::Equal)
    }
    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, &mut ray_t.clone()) {
            return false;
        }

        let hit_left = self
            .left
            .as_ref()
            .map_or(false, |left| left.hit(r, ray_t, rec));
        let hit_right = self.right.as_ref().map_or(false, |right| {
            right.hit(
                r,
                Interval::from(
                    ray_t.min(),
                    match hit_left {
                        true => rec.t,
                        false => ray_t.max(),
                    },
                ),
                rec,
            )
        });
        hit_left || hit_right
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
