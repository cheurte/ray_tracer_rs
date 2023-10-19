use crate::rtweekend::{random_double, random_double_interval};
use std::{cmp, fmt, ops};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn from(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn random() -> Self {
        Self {
            x: random_double(),
            y: random_double(),
            z: random_double(),
        }
    }
    pub fn random_interval(min: f64, max: f64) -> Self {
        Self {
            x: random_double_interval(min, max),
            y: random_double_interval(min, max),
            z: random_double_interval(min, max),
        }
    }
    pub fn random_in_unit_sphere() -> Self {
        let mut p = Self::random_interval(-1.0, 1.0);
        while p.length_squared() < 1.0 {
            p = Self::random_interval(-1.0, 1.0);
        }
        p
    }
    pub fn random_unit_vector() -> Self {
        Self::unit_vector(Self::random_in_unit_sphere())
    }

    pub fn random_on_hetmisphere(&self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(*self) > 0.0 {
            return on_unit_sphere;
        }
        on_unit_sphere * -1.0
    }
    #[allow(non_snake_case)]
    pub fn X(reverse: bool) -> Self {
        Self {
            x: match reverse {
                true => -1.0,
                false => 1.0,
            },
            y: 0.0,
            z: 0.0,
        }
    }
    #[allow(non_snake_case)]
    pub fn Y(reverse: bool) -> Self {
        Self {
            x: 0.0,
            y: match reverse {
                true => -1.0,
                false => 1.0,
            },
            z: 0.0,
        }
    }
    #[allow(non_snake_case)]
    pub fn Z(reverse: bool) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: match reverse {
                true => -1.0,
                false => 1.0,
            },
        }
    }
    pub fn x(self) -> f64 {
        self.x
    }
    pub fn y(self) -> f64 {
        self.y
    }
    pub fn z(self) -> f64 {
        self.z
    }
    pub fn zeros() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn ones() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(self, other: Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn unit_vector(self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
    pub fn reflect(&self, n: &Vec3) -> Self {
        *self - *n * self.dot(*n) * 2.0
    }
    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = -self.dot(*n).min(1.0);
        let r_out_perp = (*self + *n * cos_theta) * etai_over_etat;
        let r_out_parallel = *n * (-1.0) * (1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
    pub fn random_in_unit_disk() -> Self {
        let mut p = Vec3::from(
            random_double_interval(-1.0, 1.0),
            random_double_interval(-1.0, 1.0),
            0.0,
        );
        while p.length_squared() < 1.0 {
            p = Vec3::from(
                random_double_interval(-1.0, 1.0),
                random_double_interval(-1.0, 1.0),
                0.0,
            );
        }
        p
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl ops::Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Div<i32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        Self {
            x: self.x / rhs as f64,
            y: self.y / rhs as f64,
            z: self.z / rhs as f64,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("unknown field {}", index),
        }
    }
}

impl ops::Index<i32> for Vec3 {
    type Output = f64;
    fn index(&self, index: i32) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("unknown field {}", index),
        }
    }
}

impl ops::IndexMut<i32> for Vec3 {
    fn index_mut(&mut self, index: i32) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("unknown field {}", index),
        }
    }
}
impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("unknown field {}", index),
        }
    }
}
