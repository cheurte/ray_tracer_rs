use crate::color::{write_colors, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{degrees2radians, random_double, INF};
use crate::vec3::{Point3, Vec3};

use kdam::tqdm;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: i32,
    pub lookfrom: Point3, // Point camera is looking from
    pub lookat: Point3,   // Point camera is looking at
    pub vup: Vec3,        // Camera-relative "up" direction
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub backround: Color,
    image_height: i32,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    camera_center: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: i32,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
        backround: Color,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            backround,
            image_height: 0,
            camera_center: Point3::zeros(),
            pixel00_loc: Point3::zeros(),
            pixel_delta_u: Vec3::zeros(),
            pixel_delta_v: Vec3::zeros(),
            u: Vec3::zeros(),
            v: Vec3::zeros(),
            w: Vec3::zeros(),
            defocus_disk_u: Vec3::zeros(),
            defocus_disk_v: Vec3::zeros(),
        }
    }

    pub fn render<T: Hittable>(&mut self, world: &T) {
        self.initialize();
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in tqdm!(0..self.image_height) {
            for i in 0..self.image_width {
                let mut pixel_color = Color::zeros();

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&self, &r, self.max_depth, world);
                }
                write_colors(pixel_color, self.samples_per_pixel);
            }
        }
        print!("Done");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = match self.image_height {
            y if y < 1 => 1,
            _ => self.image_height,
        };

        self.camera_center = self.lookfrom;
        let theta = degrees2radians(self.vfov as f64);
        let height = (theta / 2.0).tan();
        let viewport_height = 2.0 * height * self.focus_dist as f64;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(self.w).unit_vector();
        self.v = self.w.cross(self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = self.u * viewport_width; // Vector across viewport horizontal edge
        let viewport_v = self.v * -viewport_height; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        // Calculate the location of the upper left pixel.
        let viewport_uper_left = self.camera_center
            - (self.w * self.focus_dist as f64)
            - viewport_u / 2
            - viewport_v / 2;
        self.pixel00_loc = viewport_uper_left + (self.pixel_delta_u - self.pixel_delta_v) * 0.5;

        let defocus_radius =
            degrees2radians(self.defocus_angle as f64 / 2.0).tan() * self.focus_dist as f64;
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color<T: Hittable>(&self, r: &Ray, depth: i32, world: &T) -> Color {
        let mut rec = HitRecord::new();
        if depth <= 0 {
            return Color::zeros();
        }

        if !world.hit(r, Interval::from(0.001, INF), &mut rec) {
            return self.backround;
        }
        let mut scattered = Ray::new();
        let mut attenuation = Color::ones();
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);
        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission;
        }

        let color_from_scatter = attenuation * self.ray_color(&scattered, depth - 1, world);
        color_from_emission + color_from_scatter
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = match self.defocus_angle <= 0.0 {
            true => self.camera_center,
            false => self.defocus_disk_sample(),
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::from(ray_origin, ray_direction, ray_time)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.camera_center + (self.defocus_disk_u * p[0]) + (self.defocus_disk_v * p[1])
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 100,
            max_depth: 50,
            vfov: 20,
            lookfrom: Point3::from(13.0, 2.0, 3.0),
            lookat: Point3::from(0.0, 0.0, 0.0),
            vup: Vec3::Y(false),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            backround: Color::from(0.7, 0.8, 1.0),
            image_height: 0,
            pixel00_loc: Point3::zeros(),
            pixel_delta_u: Point3::zeros(),
            pixel_delta_v: Point3::zeros(),
            camera_center: Point3::zeros(),
            u: Vec3::zeros(),
            v: Vec3::zeros(),
            w: Vec3::zeros(),
            defocus_disk_u: Vec3::zeros(),
            defocus_disk_v: Vec3::zeros(),
        }
    }
}
