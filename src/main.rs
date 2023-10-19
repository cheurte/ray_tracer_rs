use std::error;
use std::rc::Rc;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::texture::CheckerTexture;
use crate::vec3::Point3;
use bvh::BvhNode;
use quad::Quad;
use rtweekend::{random_double, random_double_interval};
use texture::{ImageTexture, NoiseTexture};
use vec3::Vec3;

mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

fn random_sphere() {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::from(
        0.32,
        Color::from(0.2, 0.3, 0.1),
        Color::from(0.9, 0.9, 0.9),
    ));

    let ground_material = Rc::new(Lambertian::from_texture(checker));
    world.add(Rc::new(Sphere::new_stationnary(
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
                // if choose_mat < 0.8 {
                //     let albedo = Color::random() * Color::random();
                //     let center2 = center + Vec3::from(0.0, random_double_interval(0.0, 0.5), 0.0);
                //     world.add(Rc::new(Sphere::new_moving(
                //         center,
                //         center2,
                //         0.2,
                //         Rc::new(Lambertian::new(albedo)),
                //     )));
                // }
                if choose_mat < 0.95 {
                    let albedo = Color::random_interval(0.5, 1.0);
                    let fuzz = random_double_interval(0.0, 0.5);
                    let center2 = center + Vec3::from(0.0, random_double_interval(0.0, 0.5), 0.0);
                    world.add(Rc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        Rc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    world.add(Rc::new(Sphere::new_stationnary(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new_stationnary(
        Point3::from(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    // let material2 = Rc::new(Lambertian::new(Color::from(0.4, 0.2, 0.1)));
    // world.add(Rc::new(Sphere::new_stationnary(
    //     Point3::from(-4.0, 1.0, 0.0),
    //     1.0,
    //     material2,
    // )));

    let material3 = Rc::new(Metal::new(Color::from(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new_stationnary(
        Point3::from(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world = HittableList::from(Rc::new(BvhNode::from_list(&mut world)));

    let mut cam = Camera::new(
        16.0 / 9.0,
        400,
        50,
        50,
        20,
        Point3::from(13.0, 2.0, 3.0),
        Point3::from(0.0, 0.0, 0.0),
        Vec3::from(0.0, 1.0, 0.0),
        0.02,
        10.0,
    );

    cam.render(&world);
}

fn two_sphere() {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::from(
        0.8,
        Color::from(0.2, 0.3, 0.1),
        Color::from(0.9, 0.9, 0.9),
    ));

    world.add(Rc::new(Sphere::new_stationnary(
        Point3::from(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::from_texture(checker.clone())),
    )));
    world.add(Rc::new(Sphere::new_stationnary(
        Point3::from(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::from_texture(checker.clone())),
    )));

    let mut cam = Camera::default();
    cam.render(&world);
}

fn earth() -> Result<(), Box<dyn error::Error>> {
    let earth_texture = Rc::new(ImageTexture::from("earthmap.jpg")?);
    let earth_surface = Rc::new(Lambertian::from_texture(earth_texture));
    let globe = Rc::new(Sphere::new_stationnary(Point3::zeros(), 2.0, earth_surface));

    let mut cam = Camera::new(
        16.0 / 9.0,
        400,
        50,
        50,
        20,
        Point3::from(0.0, 0.0, 12.0),
        Point3::from(0.0, 0.0, 0.0),
        Vec3::from(0.0, 1.0, 0.0),
        0.00,
        10.0,
    );

    cam.render(&HittableList::from(globe));
    Ok(())
}

fn two_perlin_noise() {
    let mut world = HittableList::new();

    let pertext = Rc::new(NoiseTexture::from(256, 4.0));
    world.add(Rc::new(Sphere::new_stationnary(
        Point3::from(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::from_texture(pertext.clone())),
    )));
    world.add(Rc::new(Sphere::new_stationnary(
        Point3::from(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::from_texture(pertext)),
    )));
    let mut cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20,
        Point3::from(13.0, 2.0, 3.0),
        Point3::from(0.0, 0.0, 0.0),
        Vec3::from(0.0, 1.0, 0.0),
        0.00,
        10.0,
    );

    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::new();

    //Materials
    let left_red = Rc::new(Lambertian::from_color(Color::from(1.0, 0.2, 0.2)));
    let back_green = Rc::new(Lambertian::from_color(Color::from(0.2, 1.0, 0.2)));
    let right_blue = Rc::new(Lambertian::from_color(Color::from(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Lambertian::from_color(Color::from(1.0, 0.5, 0.0)));
    let lower_teal = Rc::new(Lambertian::from_color(Color::from(0.2, 0.8, 0.8)));

    world.add(Rc::new(Quad::from(
        Point3::from(-3.0, -2.0, 5.0),
        Vec3::from(0.0, 0.0, -4.0),
        Vec3::from(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Rc::new(Quad::from(
        Point3::from(-2.0, -2.0, 0.0),
        Vec3::from(4.0, 0.0, 0.0),
        Vec3::from(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Rc::new(Quad::from(
        Point3::from(3.0, -2.0, 1.0),
        Vec3::from(0.0, 0.0, 4.0),
        Vec3::from(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Rc::new(Quad::from(
        Point3::from(-2.0, 3.0, 1.0),
        Vec3::from(4.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Rc::new(Quad::from(
        Point3::from(-2.0, -3.0, 5.0),
        Vec3::from(4.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let mut cam = Camera::new(
        1.0,
        400,
        1000,
        50,
        80,
        Point3::from(0.0, 0.0, 9.0),
        Point3::from(0.0, 0.0, 0.0),
        Vec3::from(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );
    cam.render(&world);
}
fn main() {
    // earth();
    // two_sphere();
    // two_perlin_noise();
    quads();
}
