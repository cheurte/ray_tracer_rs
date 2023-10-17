use crate::rtweekend::INF;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new() -> Self {
        Self {
            min: -INF,
            max: INF,
        }
    }

    pub const fn from(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: Interval, b: Interval) -> Self {
        Self {
            min: a.min().min(b.min()),
            max: a.max().max(b.max()),
        }
    }

    pub fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(self, x: f64) -> f64 {
        if x < self.min {
            return self.max;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
    pub fn max(self) -> f64 {
        self.max
    }
    pub fn min(self) -> f64 {
        self.min
    }
    pub fn size(self) -> f64 {
        self.max - self.min
    }
    pub fn modify_min(&mut self, value: f64) {
        self.min = value;
    }
    pub fn modify_max(&mut self, value: f64) {
        self.max = value;
    }
    pub fn expand(self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max - padding,
        }
    }
}

const UNIVERSE: Interval = Interval::from(-INF, INF);
const EMPTY: Interval = Interval::from(INF, -INF);
