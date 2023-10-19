use image::{io::Reader as ImageReader, DynamicImage, Pixel, Rgb};

use crate::{color::Color, interval::Interval, perlin::Perlin, vec3::Point3};
use std::{error, rc::Rc};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new() -> Self {
        Self {
            color_value: Color::zeros(),
        }
    }
    pub fn from_color(color_value: Color) -> Self {
        Self { color_value }
    }
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: Color::from(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(inv_scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / inv_scale,
            even,
            odd,
        }
    }
    pub fn from(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Rc::new(SolidColor::from_color(c1)),
            odd: Rc::new(SolidColor::from_color(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_integer = (self.inv_scale * p.x()).floor() as i32;
        let y_integer = (self.inv_scale * p.y()).floor() as i32;
        let z_integer = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        match is_even {
            true => self.even.value(u, v, &p),
            false => self.odd.value(u, v, &p),
        }
    }
}

pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn from<S: AsRef<str>>(image_path: S) -> Result<Self, Box<dyn error::Error>> {
        let image = ImageReader::open(image_path.as_ref())?.decode()?;
        Ok(Self { image })
    }

    pub fn clamp(x: u32, low: u32, high: u32) -> u32 {
        if x < low {
            return low;
        }
        if x < high {
            return x;
        }
        high - 1
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: &Point3) -> Color {
        if self.image.height() <= 0 {
            return Color::from(0.0, 1.0, 1.0);
        }
        let u = Interval::from(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::from(0.0, 1.0).clamp(v);

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;

        let bindings = self.image.to_rgb8();
        let pixel = bindings
            .get_pixel_checked(
                // i,
                // j,
                ImageTexture::clamp(i, 0, self.image.width()),
                ImageTexture::clamp(j, 0, self.image.height()),
            )
            .unwrap_or(Rgb::from_slice(&[255, 0, 255]));

        let color_scale = 1.0 / 255.0;
        Color::from(
            color_scale * pixel.0[0] as f64,
            color_scale * pixel.0[1] as f64,
            color_scale * pixel.0[2] as f64,
        )
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(256),
            scale: 1.0,
        }
    }
    pub fn from(point_count: i32, scale: f64) -> Self {
        Self {
            noise: Perlin::new(point_count),
            scale,
        }
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self {
            noise: Perlin::new(256),
            scale: 1.0,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: &Point3) -> Color {
        let s = *p * self.scale;
        Color::ones() * 0.5 * (1.0 + (s.z() + 10.0 * self.noise.turb(s, 7)).sin())
    }
}
