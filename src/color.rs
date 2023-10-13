use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_colors(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    // Apply the linear to gamma transform.
    r = linear2gamma(r);
    g = linear2gamma(g);
    b = linear2gamma(b);

    // Write the translated [0,255] value of each color component.
    let intensity = Interval::from(0.000, 0.999);
    println!(
        "{} {} {}",
        (256.0 * intensity.clamp(r)) as i32,
        (256.0 * intensity.clamp(g)) as i32,
        (256.0 * intensity.clamp(b)) as i32
    );
}

fn linear2gamma(linear_copenant: f64) -> f64 {
    linear_copenant.sqrt()
}
