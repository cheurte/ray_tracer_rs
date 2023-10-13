use rtweekend::random_double;
use vec3::Vec3;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Materials, Metal};
use crate::rtweekend::PI;
use crate::sphere::Sphere;
use crate::vec3::Point3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    let mut world = HittableList::new();

    let ground_material = Materials::Lambertian(Color::from(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::from(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::from(4.0, 0.2, 0.0)).length() > 0.9 {}
        }
    }

    let mut cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20,
        Point3::from(-2.0, 2.0, 1.0),
        Point3::from(0.0, 0.0, -1.0),
        Vec3::from(0.0, 1.0, 0.0),
        10.0,
        3.4,
    );

    cam.render(&world);
}
