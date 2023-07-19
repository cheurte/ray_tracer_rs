use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn from(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
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

    fn length_squared(self) -> f64 {
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
            x: self.y * other.z - self.z - other.y,
            y: self.z * other.x - self.x - other.z,
            z: self.x * other.y - self.y - other.x,
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
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
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
