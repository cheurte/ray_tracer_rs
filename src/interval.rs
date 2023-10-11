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

    pub fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn max(self) -> f64 {
        self.max
    }
    pub fn min(self) -> f64 {
        self.min
    }
}

const UNIVERSE: Interval = Interval::from(-INF, INF);
const EMPTY: Interval = Interval::from(INF, -INF);
