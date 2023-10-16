use std::cell::RefCell;
use std::rc::Rc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
// use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
// use crate::vec3::Point3;
// use crate::rtweekend::random_int;
#[derive(Debug, Copy, Clone)]
pub enum HittableObjects {
    HittableListObj,
    SphereObj(Point3,f64, Materials, bool, Vec3, Aabb),
}

#[derive(Debug, Clone)]
struct BvhNode {
    left: Rc<HittableObjects>,
    right: Rc<HittableObjects>,
    bbox: Aabb,
}
//
impl BvhNode {
    // pub fn new() -> Self {
    //     Self {
    //         left: Rc::new(HittableObjects::SphereObj),
    //         right: Rc::new(HittableObjects::SphereObj),
    //         bbox: Aabb::new(),
    //     }
    // }
}
//     pub fn create(src_objects: Vec<&Box<dyn Hittable>>, start: usize, end: usize) -> Self {
//         // let axis = random_int(0, 2);
//         // let comparator = match axis {
//         //     0 => BvhNode::box_x_compare,
//         //     1 => BvhNode::box_y_compare,
//         //     _ => BvhNode::box_z_compare,
//         // };
//
//         let object_span = end - start;
//
//         // let mut left: Option<&Box<dyn Hittable>> = None;
//         // let mut right: Option<&Box<dyn Hittable>> = None;
//
//         let left = Rc::new(RefCell::new(*src_objects[start]));
//         let right = Rc::new(RefCell::new(*src_objects[start]));
//
//         let bbox = Aabb::from_bbox(left.borrow().bounding_box(), right.borrow().bounding_box());
//         Self {
//             left: Some(left),
//             right: Some(right),
//             bbox,
//         }
//         // Self::new()
//         // Self {
//         //     left: Rc::new(RefCell::new(left)),
//         //     right: Rc::new(RefCell::new(right)),
//         //     bbox,
//         // }
//         // else if object_span == 2 {
//         //     if comparator(&src_objects[start], &src_objects[start + 1]) {
//         //         left = Some(src_objects[start]);
//         //         right = Some(src_objects[start + 1]);
//         //     } else {
//         //         left = Some(src_objects[start + 1]);
//         //         right = Some(src_objects[start]);
//         //     }
//         // } else {
//         //     // (src_objects.first().unwrap() + start)
//         //     //     .sort(src_objects.first().unwrap() + end, comparator);
//         //     // let mid = start + object_span / 2;
//         //     // left = Box::new(BvhNode::new(src_objects, start, mid));
//         //     // right = Box::new(BvhNode::new(src_objects, mid, end));
//         // }
//         // if left.is_some() && right.is_some() {
//         // if let (Some(right), Some(left)) = (right, left) {
//         //     let bbox = Aabb::from_bbox(left.bounding_box(), right.bounding_box());
//         //     return Some(Self::new(*left, *right, bbox));
//         // }
//
//         // return Some(Self {
//         //     left: left,
//         //     right: right,
//         //     bbox,
//         // });
//         // }
//         // None
//     }
//
//     fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis_index: i32) -> bool {
//         a.bounding_box().axis(axis_index).min() < b.bounding_box().axis(axis_index).max()
//     }
//     fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> bool {
//         BvhNode::box_compare(a, b, 0)
//     }
//     fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> bool {
//         BvhNode::box_compare(a, b, 1)
//     }
//     fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> bool {
//         BvhNode::box_compare(a, b, 2)
//     }
// }
//
impl Hittable for BvhNode {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, &mut ray_t.clone()) {
            return false;
        }
        match self.left {
            HittableObjects::SphereObj => {
                let hit_left = Sphere::from
            }
        }
        // let hit_left = self.left.unwrap().borrow().hit(r, ray_t, rec);
        // let hit_right = self.right.unwrap().borrow().hit(
        //     r,
        //     Interval::from(
        //         ray_t.min(),
        //         match hit_left {
        //             true => rec.t,
        //             false => ray_t.max(),
        //         },
        //     ),
        //     rec,
        // );
        // hit_left || hit_right
        true
    }
}
