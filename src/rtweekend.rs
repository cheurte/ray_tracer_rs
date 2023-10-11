use std::f64::INFINITY;

pub const INF: f64 = INFINITY;
const PI: f64 = 3.1415926535897932385;

fn degrees2radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
