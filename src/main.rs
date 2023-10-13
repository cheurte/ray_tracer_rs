use rtweekend::{random_double, random_double_interval};
use vec3::Vec3;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::Materials;
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

            if (center - Point3::from(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    world.add(Box::new(Sphere::from(
                        center,
                        0.2,
                        Materials::Lambertian(albedo),
                    )));
                }
                if choose_mat < 0.95 {
                    let albedo = Color::random_interval(0.5, 1.0);
                    let fuzz = random_double_interval(0.0, 0.5);
                    world.add(Box::new(Sphere::from(
                        center,
                        0.2,
                        Materials::Metal(albedo, fuzz),
                    )));
                } else {
                    world.add(Box::new(Sphere::from(
                        center,
                        0.2,
                        Materials::Dielectric(1.5),
                    )));
                }
            }
        }
    }

    let material1 = Materials::Dielectric(1.5);
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Materials::Lambertian(Color::from(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::from(
        Point3::from(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Materials::Metal(Color::from(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::from(
        Point3::from(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = Camera::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        20,
        Point3::from(13.0, 2.0, 3.0),
        Point3::from(0.0, 0.0, 0.0),
        Vec3::from(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );

    cam.render(&world);
}
