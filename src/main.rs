use std::rc::Rc;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::texture::CheckerTexture;
use crate::vec3::Point3;
use bvh::BvhNode;
use constant_medium::ConstantMedium;
use hittable::{RotateY, Translate};
use material::DiffuseLight;
use quad::{box_volume, Quad};
use rtweekend::{random_double, random_double_interval};
use texture::{ImageTexture, NoiseTexture};
use vec3::Vec3;

mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
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
        Color::from(0.7, 0.8, 1.0),
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

fn earth() {
    let earth_texture = Rc::new(ImageTexture::from("earthmap.jpg").unwrap());
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
        Color::from(0.7, 0.8, 1.0),
    );

    cam.render(&HittableList::from(globe));
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
        Color::from(0.7, 0.8, 1.0),
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
        Color::from(0.7, 0.8, 1.0),
    );
    cam.render(&world);
}

fn simple_light() {
    let mut world = HittableList::new();

    let pertext = Rc::new(NoiseTexture::from_default(4.0));
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

    let diff_light = Rc::new(DiffuseLight::from_color(Color::from(4.0, 4.0, 4.0)));
    world.add(Rc::new(Quad::from(
        Point3::from(3.0, 1.0, -2.0),
        Vec3::from(2.0, 0.0, 0.0),
        Point3::from(0.0, 2.0, 0.0),
        diff_light.clone(),
    )));
    world.add(Rc::new(Sphere::new_stationnary(
        Point3::from(0.0, 7.0, 0.0),
        2.0,
        diff_light,
    )));

    let mut cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20,
        Point3::from(26.0, 3.0, 6.0),
        Point3::from(0.0, 2.0, 0.0),
        Vec3::from(0.0, 1.0, 0.0),
        0.0,
        10.0,
        Color::zeros(),
    );
    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::new();
    let red = Rc::new(Lambertian::from_color(Color::from(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::from_color(Color::from(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::from_color(Color::from(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::from_color(Color::from(15.0, 15.0, 15.0)));

    world.add(Rc::new(Quad::from(
        Point3::from(555.0, 0.0, 0.0),
        Vec3::from(0.0, 555.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Rc::new(Quad::from(
        Point3::from(0.0, 0.0, 0.0),
        Vec3::from(0.0, 555.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Rc::new(Quad::from(
        Point3::from(343.0, 554.0, 332.0),
        Vec3::from(-130.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Rc::new(Quad::from(
        Point3::zeros(),
        Vec3::from(555.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::from(
        Point3::from(555.0, 555.0, 555.0),
        Vec3::from(-555.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::from(
        Point3::from(0.0, 0.0, 555.0),
        Vec3::from(555.0, 0.0, 0.0),
        Vec3::from(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // world.add(Rc::new(box_volume(
    //     Point3::from(130.0, 0.0, 65.0),
    //     Point3::from(295.0, 165.0, 230.0),
    //     white.clone(),
    // )));
    // world.add(Rc::new(box_volume(
    //     Point3::from(265.0, 0.0, 295.0),
    //     Point3::from(430.0, 330.0, 460.0),
    //     white.clone(),
    // )));

    let box1 = Rc::new(box_volume(
        Point3::zeros(),
        Point3::from(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Rc::new(RotateY::from(box1, 15.0));
    let box1 = Rc::new(Translate::from(box1, Vec3::from(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = Rc::new(box_volume(
        Point3::zeros(),
        Point3::from(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Rc::new(RotateY::from(box2, -18.0));
    let box2 = Rc::new(Translate::from(box2, Vec3::from(130.0, 0.0, 65.0)));
    world.add(box2);

    let mut cam = Camera::new(
        1.0,
        600,
        200,
        50,
        40,
        Point3::from(278.0, 278.0, -800.0),
        Point3::from(278.0, 278.0, 0.0),
        Vec3::from(0.0, 1.0, 0.0),
        0.0,
        10.0,
        Color::zeros(),
    );
    cam.render(&world);
}

fn cornell_smoke() {
    let mut world = HittableList::new();

    let red = Rc::new(Lambertian::from_color(Color::from(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::from_color(Color::from(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::from_color(Color::from(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::from_color(Color::from(7.0, 7.0, 7.0)));

    world.add(Rc::new(Quad::from(
        Point3::from(555.0, 0.0, 0.0),
        Vec3::from(0.0, 555.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Rc::new(Quad::from(
        Point3::from(0.0, 0.0, 0.0),
        Vec3::from(0.0, 555.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Rc::new(Quad::from(
        Point3::from(113.0, 554.0, 127.0),
        Vec3::from(330.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Rc::new(Quad::from(
        Point3::zeros(),
        Vec3::from(555.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::from(
        Point3::from(555.0, 555.0, 555.0),
        Vec3::from(-555.0, 0.0, 0.0),
        Vec3::from(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::from(
        Point3::from(0.0, 0.0, 555.0),
        Vec3::from(555.0, 0.0, 0.0),
        Vec3::from(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = Rc::new(box_volume(
        Point3::zeros(),
        Point3::from(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Rc::new(RotateY::from(box1, 15.0));
    let box1 = Rc::new(Translate::from(box1, Vec3::from(265.0, 0.0, 295.0)));

    let box2 = Rc::new(box_volume(
        Point3::zeros(),
        Point3::from(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Rc::new(RotateY::from(box2, -18.0));
    let box2 = Rc::new(Translate::from(box2, Vec3::from(130.0, 0.0, 65.0)));

    world.add(Rc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Color::zeros(),
    )));
    world.add(Rc::new(ConstantMedium::from_color(
        box2,
        0.01,
        Color::ones(),
    )));
    let mut cam = Camera::new(
        1.0,
        600,
        200,
        50,
        40,
        Point3::from(278.0, 278.0, -800.0),
        Point3::from(278.0, 278.0, 0.0),
        Vec3::from(0.0, 1.0, 0.0),
        0.0,
        10.0,
        Color::zeros(),
    );
    cam.render(&world);
}

fn final_scene(image_width: i32, samples_per_pixel: i32, max_depth: i32) {
    let mut boxes1 = HittableList::new();

    let ground = Rc::new(Lambertian::from_color(Color::from(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_interval(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Rc::new(box_volume(
                Point3::from(x0, y0, z0),
                Point3::from(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HittableList::new();

    world.add(Rc::new(boxes1));

    let light = Rc::new(DiffuseLight::from_color(Color::from(7.0, 7.0, 7.0)));
    world.add(Rc::new(Quad::from(
        Point3::from(125.0, 554.0, 147.0),
        Vec3::from(300.0, 0.0, 0.0),
        Point3::from(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point3::from(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::from(30.0, 0.0, 0.0);
    let sphere_material = Rc::new(Lambertian::from_color(Color::from(0.7, 0.3, 0.1)));
    world.add(Rc::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Rc::new(Sphere::new_stationnary(
        Point3::from(260.0, 150.0, 45.0),
        50.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Rc::new(Sphere::new_stationnary(
        Point3::from(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(Color::from(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Rc::new(Sphere::new_stationnary(
        Point3::from(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Rc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::from(0.2, 0.4, 0.9),
    )));
    let boundary = Rc::new(Sphere::new_stationnary(
        Point3::zeros(),
        5000.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.add(Rc::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Color::ones(),
    )));

    let emat = Rc::new(Lambertian::from_texture(Rc::new(
        ImageTexture::from("earthmap.jpg").unwrap(),
    )));
    world.add(Rc::new(Sphere::new_stationnary(
        Point3::from(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Rc::new(NoiseTexture::from_default(0.1));
    world.add(Rc::new(Sphere::new_stationnary(
        Point3::from(220.0, 2880.0, 300.0),
        80.0,
        Rc::new(Lambertian::from_texture(pertext)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Rc::new(Lambertian::from_color(Color::from(0.73, 0.73, 0.73)));
    let ns = 100;
    for _ in 0..ns {
        boxes2.add(Rc::new(Sphere::new_stationnary(
            Point3::random_interval(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    let r = RotateY::from(Rc::new(BvhNode::from_list(&mut boxes2)), 15.0);
    let t = Translate::from(Rc::new(r), Vec3::from(-100.0, 270.0, 395.0));
    world.add(Rc::new(t));

    let mut cam = Camera::new(
        1.0,
        image_width,
        samples_per_pixel,
        max_depth,
        40,
        Point3::from(478.0, 278.0, -600.0),
        Point3::from(278.0, 278.0, 0.0),
        Vec3::from(0.0, 1.0, 0.0),
        0.0,
        10.0,
        Color::zeros(),
    );

    cam.render(&world);
}

fn main() {
    let choice = 9;
    match choice {
        0 => random_sphere(),
        1 => earth(),
        2 => two_sphere(),
        3 => two_perlin_noise(),
        4 => quads(),
        5 => simple_light(),
        6 => cornell_box(),
        7 => cornell_smoke(),
        9 => final_scene(800, 10000, 40),
        _ => final_scene(400, 50, 4),
    }
}
