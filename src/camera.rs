use crate::color::{write_colors, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::INF;
use crate::vec3::{Point3, Vec3};

use kdam::tqdm;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    camera_center: Point3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: 0,
            camera_center: Point3::zeros(),
            pixel00_loc: Point3::zeros(),
            pixel_delta_u: Vec3::zeros(),
            pixel_delta_v: Vec3::zeros(),
        }
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initialize();
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in tqdm!(0..self.image_height) {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * i as f64)
                    + (self.pixel_delta_v * j as f64);
                let ray_directon = pixel_center - self.camera_center;
                let r: Ray = Ray::new(self.camera_center, ray_directon);

                let pixel_color = Camera::ray_color(&r, &world);
                write_colors(pixel_color);
                // pixel_color.write_colors()
            }
        }
        print!("Done");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = match self.image_height {
            0..=1 => 1,
            _ => self.image_height,
        };

        self.camera_center = Point3::zeros();

        // Camera
        let focal_lenght = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        // Calculate the location of the upper left pixel.
        let viewport_uper_left = self.camera_center
            - Vec3::from(0.0, 0.0, focal_lenght)
            - viewport_u / 2
            - viewport_v / 2;
        self.pixel00_loc = viewport_uper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn ray_color(r: &Ray, world: &HittableList) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(r, Interval::from(0.0, INF), &mut rec) {
            return (rec.normal + Color::ones()) * 0.5;
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::ones() * (1.0 - a) + Color::from(0.5, 0.7, 1.0) * a
    }
}
