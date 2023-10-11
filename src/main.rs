use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::rtweekend::INF;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

use kdam::tqdm;

mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INF, &mut rec) {
        return (rec.normal + Color::ones()) * 0.5;
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::ones() * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    // IMAGE
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = match image_height {
        0..=1 => 1,
        _ => image_height,
    };

    // World

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::Z(true), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_lenght = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::zeros();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate the location of the upper left pixel.
    let viewport_uper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_lenght) - viewport_u / 2 - viewport_v / 2;
    let pixel00_loc = viewport_uper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in tqdm!(0..image_height) {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_directon = pixel_center - camera_center;
            let r: Ray = Ray::new(camera_center, ray_directon);

            let pixel_color = ray_color(&r, &world);
            color::write_colors(pixel_color);
        }
    }
    print!("Done");
}
