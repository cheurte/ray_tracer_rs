use crate::interval::Interval;
use rand::prelude::*;
use std::f64::INFINITY;

pub const INF: f64 = INFINITY;
const PI: f64 = 3.1415926535897932385;
const RAND_MAX: i32 = 0;

fn degrees2radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn random_double() {
    rng.gen() / (RAND_MAX as f64 + 1)
}

fn random_doucle(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
