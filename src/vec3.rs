use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

// Type alias
pub type Point3 = Vec3;

// Output formatting
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

// Vec3 *= f64
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}

// Vec3 /= f64
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}

// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}

// Vec3 == Vec3
impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.e[0] == other.e[0] && self.e[1] == other.e[1] && self.e[2] == other.e[2]
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        assert_eq!(Vec3::new(1.0, 0.0, 0.0).length(), 1.0);
    }

    #[test]
    fn test_new() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.e[0], 1.0);
        assert_eq!(vec.e[1], 2.0);
        assert_eq!(vec.e[2], 3.0);

        assert_eq!(vec.x(), 1.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 3.0);
    }

    #[test]
    fn test_dot() {
        let u = Vec3::new(1.0, 0.0, 0.0);
        let v = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(dot(u, v), 0.0);

        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(dot(u, v), 32.0);
    }

    #[test]
    fn test_cross() {
        let u = Vec3::new(1.0, 0.0, 0.0);
        let v = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(cross(u, v), Vec3::new(0.0, 0.0, 1.0));

        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(cross(u, v), Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3::new(4.0, 0.0, 0.0);
        assert_eq!(unit_vector(v), Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Vec3::new(1.0, 2.0, 3.0)), "1 2 3");
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v += Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(v, Vec3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 3.5;
        assert_eq!(v, Vec3::new(3.5, 7.0, 10.5));
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v /= 2.0;
        assert_eq!(v, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_eq() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) - Vec3::new(3.0, 4.0, 5.0), Vec3::new(-2.0, -2.0, -2.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) + Vec3::new(3.0, 4.0, 5.0), Vec3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_mul_vec() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v * Vec3::new(2.0, 3.0, 4.0), Vec3::new(2.0, 6.0, 12.0));
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(2.0 * v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }
}