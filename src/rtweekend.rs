use std::f64::{consts::PI, INFINITY};

pub const INF: f64 = INFINITY;
pub const RAND_MAX: i32 = 0;

pub fn degrees2radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::random::<f64>() / (RAND_MAX as f64 + 1.0)
}

pub fn random_double_interval(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
pub fn random_int(min: i32, max: i32) -> i32 {
    random_double_interval(min as f64, max as f64 + 1.0) as i32
}
