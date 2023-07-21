use kdam::tqdm;
mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

pub fn write_colors(color: Vec3) {
    println!(
        "{} {} {}",
        (255.999 * color.x()) as i32,
        (255.999 * color.y()) as i32,
        (255.999 * color.z()) as i32,
    )
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    // let a = r.direction().dot(r.direction());
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    // let c = oc.dot(oc) - radius * radius;
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (-half_b - discriminant.sqrt()) / a
}

pub fn ray_color(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::Z(true), 0.5, r);
    if t > 0.0 {
        let norm = (r.at(t) - Vec3::Z(true)).unit_vector();
        return Vec3::new(norm.x() + 1.0, norm.y() + 1.0, norm.z() + 1.0) * 0.5;
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::zeros();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2 - vertical / 2 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");
    for j in tqdm!((0..IMAGE_HEIGHT).rev()) {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let v = j as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let color = ray_color(&r);
            write_colors(color);
        }
    }
}
