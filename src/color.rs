use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_colors(color: Color) {
    let ir = (255.999 * color.x()) as i32;
    let ig = (255.999 * color.y()) as i32;
    let ib = (255.999 * color.z()) as i32;
    println!("{ir} {ig} {ib}");
}
