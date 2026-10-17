use crate::generated::*;

pub const PI: f32 = std::f32::consts::PI;
pub const DEG2RAD: f32 = PI / 180.0;
pub const RAD2DEG: f32 = 180.0 / PI;

#[inline]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.clamp(min, max)
}

#[inline]
pub fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    start + amount * (end - start)
}

#[inline]
pub fn remap(value: f32, input_start: f32, input_end: f32, output_start: f32, output_end: f32) -> f32 {
    (value - input_start) / (input_end - input_start) * (output_end - output_start) + output_start
}

//------------------------------------------------------------------------------------
// Vector2 Math & Operators
//------------------------------------------------------------------------------------

impl std::ops::Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl std::ops::MulAssign for Vector2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl std::ops::Div for Vector2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl std::ops::DivAssign for Vector2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Mul<Vector2> for f32 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::Div<f32> for Vector2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl std::ops::Neg for Vector2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Eq for Vector2 {}

impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub const fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }

    pub fn scale(&self, s: f32) -> Vector2 {
        Vector2 {
            x: self.x * s,
            y: self.y * s,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn distance(&self, other: Self) -> f32 {
        (*self - other).length()
    }

    pub fn lerp(&self, other: Self, amount: f32) -> Self {
        *self + (other - *self) * amount
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length > 0.0 {
            let ilength = 1.0 / length;
            Vector2 {
                x: self.x * ilength,
                y: self.y * ilength,
            }
        } else {
            Vector2::zero()
        }
    }
}

//------------------------------------------------------------------------------------
// Vector3 Math & Operators
//------------------------------------------------------------------------------------

impl std::ops::Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<Vector3> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl std::ops::Div<Vector3> for Vector3 {
    type Output = Self;
    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl std::ops::DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, rhs: Vector3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Eq for Vector3 {}

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub const fn one() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length > 0.0 {
            let inv = 1.0 / length;
            Vector3 {
                x: self.x * inv,
                y: self.y * inv,
                z: self.z * inv,
            }
        } else {
            Self::zero()
        }
    }

    pub fn distance(&self, other: Self) -> f32 {
        (*self - other).length()
    }

    pub fn lerp(&self, other: Self, amount: f32) -> Self {
        *self + (other - *self) * amount
    }

    pub fn scale(&self, s: f32) -> Self {
        *self * s
    }
}

//------------------------------------------------------------------------------------
// Vector4 Math & Operators
//------------------------------------------------------------------------------------

impl Eq for Vector4 {}

impl Vector4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub const fn one() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn length_sq(&self) -> f32 {
        self.dot(*self)
    }

    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    pub fn lerp(&self, other: Self, amount: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * amount,
            y: self.y + (other.y - self.y) * amount,
            z: self.z + (other.z - self.z) * amount,
            w: self.w + (other.w - self.w) * amount,
        }
    }
}

//------------------------------------------------------------------------------------
// Matrix Helpers & Operators
//------------------------------------------------------------------------------------

impl Matrix {
    pub const fn identity() -> Self {
        Self {
            m0: 1.0, m1: 0.0, m2: 0.0, m3: 0.0,
            m4: 0.0, m5: 1.0, m6: 0.0, m7: 0.0,
            m8: 0.0, m9: 0.0, m10: 1.0, m11: 0.0,
            m12: 0.0, m13: 0.0, m14: 0.0, m15: 1.0,
        }
    }

    pub const fn from_basis(right: Vector3, up: Vector3, forward: Vector3, position: Vector3) -> Self {
        Self {
            m0: right.x, m1: up.x, m2: forward.x, m3: position.x,
            m4: right.y, m5: up.y, m6: forward.y, m7: position.y,
            m8: right.z, m9: up.z, m10: forward.z, m11: position.z,
            m12: 0.0,    m13: 0.0,  m14: 0.0,       m15: 1.0,
        }
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            m0: self.m0 * rhs.m0 + self.m1 * rhs.m4 + self.m2 * rhs.m8 + self.m3 * rhs.m12,
            m1: self.m0 * rhs.m1 + self.m1 * rhs.m5 + self.m2 * rhs.m9 + self.m3 * rhs.m13,
            m2: self.m0 * rhs.m2 + self.m1 * rhs.m6 + self.m2 * rhs.m10 + self.m3 * rhs.m14,
            m3: self.m0 * rhs.m3 + self.m1 * rhs.m7 + self.m2 * rhs.m11 + self.m3 * rhs.m15,

            m4: self.m4 * rhs.m0 + self.m5 * rhs.m4 + self.m6 * rhs.m8 + self.m7 * rhs.m12,
            m5: self.m4 * rhs.m1 + self.m5 * rhs.m5 + self.m6 * rhs.m9 + self.m7 * rhs.m13,
            m6: self.m4 * rhs.m2 + self.m5 * rhs.m6 + self.m6 * rhs.m10 + self.m7 * rhs.m14,
            m7: self.m4 * rhs.m3 + self.m5 * rhs.m7 + self.m6 * rhs.m11 + self.m7 * rhs.m15,

            m8: self.m8 * rhs.m0 + self.m9 * rhs.m4 + self.m10 * rhs.m8 + self.m11 * rhs.m12,
            m9: self.m8 * rhs.m1 + self.m9 * rhs.m5 + self.m10 * rhs.m9 + self.m11 * rhs.m13,
            m10: self.m8 * rhs.m2 + self.m9 * rhs.m6 + self.m10 * rhs.m10 + self.m11 * rhs.m14,
            m11: self.m8 * rhs.m3 + self.m9 * rhs.m7 + self.m10 * rhs.m11 + self.m11 * rhs.m15,

            m12: self.m12 * rhs.m0 + self.m13 * rhs.m4 + self.m14 * rhs.m8 + self.m15 * rhs.m12,
            m13: self.m12 * rhs.m1 + self.m13 * rhs.m5 + self.m14 * rhs.m9 + self.m15 * rhs.m13,
            m14: self.m12 * rhs.m2 + self.m13 * rhs.m6 + self.m14 * rhs.m10 + self.m15 * rhs.m14,
            m15: self.m12 * rhs.m3 + self.m13 * rhs.m7 + self.m14 * rhs.m11 + self.m15 * rhs.m15,
        }
    }
}

//------------------------------------------------------------------------------------
// Rectangle Helpers
//------------------------------------------------------------------------------------

impl Eq for Rectangle {}

impl Rectangle {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn center(&self) -> Vector2 {
        Vector2 {
            x: self.x + self.width * 0.5,
            y: self.y + self.height * 0.5,
        }
    }

    pub fn contains(&self, point: Vector2) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }

    pub fn overlaps(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}
