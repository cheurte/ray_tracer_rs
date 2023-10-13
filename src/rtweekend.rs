use std::f64::INFINITY;

pub const INF: f64 = INFINITY;
pub const PI: f64 = 3.1415926535897932385;
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
