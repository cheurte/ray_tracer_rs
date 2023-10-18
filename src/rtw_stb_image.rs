use image::{io::Reader as ImageReader, DynamicImage};
// use std::error::Error;

// use crate::vec3::Vec3;

// #[derive(Debug, Clone)]
// pub struct RtwImage {

// bytes_per_pixel: i32,
// data: Option<Vec<u8>>,
// image_width: i32,
// image_height: i32,
// bytes_per_scanline: i32,
// }

// impl RtwImage {
//     pub fn new() -> Self {
//         Self {
//             bytes_per_pixel: 0,
//             data: None,
//             image_width: 0,
//             image_height: 0,
//             bytes_per_scanline: 0,
//         }
//     }
//
//     pub fn from_filename<S: AsRef<str>>(image_filename: S) -> Result<Self, Box<dyn Error>> {
//         let image = ImageReader::open(image_filename.as_ref())?
//             .decode()?
//             .to_rgb8();
//         let bytes_per_pixel = 3;
//         let image_height = image.height() as i32;
//         let image_width = image.width() as i32;
//         let data = image.to_vec();
//
//         Ok(Self {
//             bytes_per_pixel,
//             data: Some(data),
//             image_width,
//             image_height,
//             bytes_per_scanline: image_width * bytes_per_pixel,
//         })
//     }
//     pub fn width(&self) -> i32 {
//         self.image_width
//     }
//     pub fn height(&self) -> i32 {
//         self.image_height
//     }
//     pub fn get_pixel(&self, x:i32, y: i32)->Option<Vec3>{
//         if self.data.is_none {
//             return None;
//         }
//         // self.data.
//     }
//
//     fn clamp(x: i32, low: i32, high: i32) -> i32 {
//         if x < low {
//             return low;
//         }
//         if x < high {
//             return high;
//         }
//         high - 1
//     }
// }
// pub struct RtwImage {
//
// }
// impl DynamicImage for RtwImage {}
