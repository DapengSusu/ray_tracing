use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Index, Mul, Neg, Sub, SubAssign}
};
use utils::rtweekend;

pub mod extension;

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Point3 = Vec3;

impl Vec3 {
    /// 零向量
    /// 等价于 `Vec3::new(0., 0., 0.)`
    /// ```
    /// use vector3::Vec3;
    /// let v = Vec3::zero();
    /// assert_eq!(v.x, 0.);
    /// assert_eq!(v.y, 0.);
    /// assert_eq!(v.z, 0.);
    /// ```
    pub fn zero() -> Self {
        Self::isotropic(0.)
    }

    /// 单位向量
    /// 等价于 `Vec3::new(1., 1., 1.)`
    /// ```
    /// use vector3::Vec3;
    /// let v = Vec3::one();
    /// assert_eq!(v.x, 1.);
    /// assert_eq!(v.y, 1.);
    /// assert_eq!(v.z, 1.);
    /// ```
    pub fn one() -> Self {
        Self::isotropic(1.)
    }

    /// 构造各分量相等的向量
    /// 等价于 `Vec3::new(value, value, value)`
    /// ```
    /// use vector3::Vec3;
    /// let v = Vec3::isotropic(1.);
    /// assert_eq!(v.x, 1.);
    /// assert_eq!(v.y, 1.);
    /// assert_eq!(v.z, 1.);
    /// ```
    pub fn isotropic(value: f64) -> Self {
        Self::new(value, value, value)
    }

    /// 构造向量
    /// # Examples
    /// ```
    /// use vector3::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
    /// assert_eq!(v.x, 1.);
    /// assert_eq!(v.y, 2.);
    /// assert_eq!(v.z, 3.);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// 从x轴构造向量
    /// 等价于 `Vec3::new(x, 0., 0.)`
    /// ```
    /// use vector3::Vec3;
    /// let v = Vec3::from_x(1.);
    /// assert_eq!(v.x, 1.);
    /// assert_eq!(v.y, 0.);
    /// assert_eq!(v.z, 0.);
    /// ```
    pub fn from_x(x: f64) -> Self {
        Self::new(x, 0., 0.)
    }

    /// 从y轴构造向量
    /// 等价于 `Vec3::new(0., y, 0.)`
    /// ```
    /// use vector3::Vec3;
    /// let v = Vec3::from_y(1.);
    /// assert_eq!(v.x, 0.);
    /// assert_eq!(v.y, 1.);
    /// assert_eq!(v.z, 0.);
    /// ```
    pub fn from_y(y: f64) -> Self {
        Self::new(0., y, 0.)
    }

    /// 从z轴构造向量
    /// 等价于 `Vec3::new(0., 0., z)`
    /// ```
    /// use vector3::Vec3;
    /// let v = Vec3::from_z(1.);
    /// assert_eq!(v.x, 0.);
    /// assert_eq!(v.y, 0.);
    /// assert_eq!(v.z, 1.);
    /// ```
    pub fn from_z(z: f64) -> Self {
        Self::new(0., 0., z)
    }

    /// 随机向量，范围[0, 1)
    pub fn random() -> Self {
        Self::new(
            rtweekend::random(),
            rtweekend::random(),
            rtweekend::random()
        )
    }

    /// 随机向量，范围[min, max]
    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            rtweekend::random_range(min, max),
            rtweekend::random_range(min, max),
            rtweekend::random_range(min, max)
        )
    }

    /// 向量点积
    /// `x1*x2 + y1*y2 + z1*z2`
    /// ```
    /// use vector3::Vec3;
    /// let v1 = Vec3::new(1., 2., 3.);
    /// let v2 = Vec3::new(4., 5., 6.);
    /// assert_eq!(v1.dot(&v2), 32.);
    /// ```
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// 与自身点积
    /// `x*x + y*y + z*z`
    /// ```
    /// use vector3::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
    /// assert_eq!(v.dot_self(), 14.);
    /// ```
    pub fn dot_self(&self) -> f64 {
        self.dot(self)
    }

    /// 向量叉积
    /// `(y1*z2 - z1*y2, z1*x2 - x1*z2, x1*y2 - y1*x2)`
    /// ```
    /// use vector3::Vec3;
    /// let v1 = Vec3::new(1., 2., 3.);
    /// let v2 = Vec3::new(4., 5., 6.);
    /// assert_eq!(v1.cross(&v2), Vec3::new(-3., 6., -3.));
    /// ```
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    /// 归一化，单位向量
    /// ```
    /// use vector3::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
    /// assert_eq!(v.normalize().norm(), 1.);
    /// ```
    pub fn normalize(&self) -> Self {
        *self / self.norm()
    }

    /// 向量模平方
    /// `x*x + y*y + z*z`
    pub fn squared(&self) -> f64 {
        self.dot_self()
    }

    /// 向量长度（模）
    /// `sqrt(x*x + y*y + z*z)`
    pub fn norm(&self) -> f64 {
        self.squared().sqrt()
    }

    /// 判断向量是否接近零
    /// ```
    /// use vector3::Vec3;
    /// let v = Vec3::new(1e-9, 1e-9, 1e-9);
    /// assert!(v.near_zero());
    /// let v = Vec3::new(1e-8, 1e-8, 1e-8);
    /// assert!(v.near_zero() == false);
    /// ```
    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        self.x < 1e-8 && self.y < 1e-8 && self.z < 1e-8
    }

    /// 向量元素和
    /// `x + y + z`
    /// ```
    /// use vector3::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
    /// assert_eq!(v.sum(), 6.);
    /// ```
    pub fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({},{},{})", self.x, self.y, self.z)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        utils::check_f64_eq!(self.x, other.x) &&
        utils::check_f64_eq!(self.y, other.y) &&
        utils::check_f64_eq!(self.z, other.z)
    }
}

impl Eq for Vec3 {}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self * rhs.x,
            self * rhs.y,
            self * rhs.z
        )
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds: {}", index)
        }
    }
}

#[cfg(test)]
mod tests {
    use utils::assert_f64_eq;

    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Vec3::zero(), Vec3::new(0., 0., 0.));
        assert_eq!(Vec3::one(), Vec3::new(1., 1., 1.));

        assert_eq!(Vec3::from_x(5.), Vec3::new(5., 0., 0.));
        assert_eq!(Vec3::from_y(5.), Vec3::new(0., 5., 0.));
        assert_eq!(Vec3::from_z(5.), Vec3::new(0., 0., 5.));
    }

    #[test]
    fn test_axis() {
        let v = Vec3::new(1., 2., 3.);
        assert_f64_eq!(v.x, 1.);
        assert_f64_eq!(v.y, 2.);
        assert_f64_eq!(v.z, 3.);
    }

    #[test]
    fn test_format() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(format!("{:?}", v), "Vec3(1,2,3)");
        assert_eq!(format!("{}", v), "1 2 3");
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(-v, Vec3::new(-1., -2., -3.));
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);
        assert_eq!(v1 + v2, Vec3::new(5., 7., 9.));

        let mut v3 = v1;
        v3 += v2;
        assert_eq!(v3, Vec3::new(5., 7., 9.));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);
        assert_eq!(v1 - v2, Vec3::new(-3., -3., -3.));

        let mut v3 = v1;
        v3 -= v2;
        assert_eq!(v3, Vec3::new(-3., -3., -3.));
    }

    #[test]
    fn test_mul() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v * 2., Vec3::new(2., 4., 6.));
        assert_eq!(4. * v, Vec3::new(4., 8., 12.));

        let v1 = Vec3::new(1., 3., 5.);
        let v2 = Vec3::new(2., 4., 6.);
        assert_eq!(v1 * v2, Vec3::new(2., 12., 30.));
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(9., 18., 27.);
        assert_eq!(v / 3., Vec3::new(3., 6., 9.));

        let v1 = Vec3::new(3., 9., 15.);
        let v2 = Vec3::new(1., 3., 5.);
        assert_eq!(v1 / v2, Vec3::new(3., 3., 3.));
    }

    #[test]
    fn test_dot() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.dot_self(), v.dot(&v));
    }

    #[test]
    fn test_normalize() {
        let v = Vec3::new(1., 2., 3.);
        assert_f64_eq!(v.normalize().squared(), 1.);
    }

    #[test]
    fn test_index() {
        let v = Vec3::new(1., 2., 3.);
        assert_f64_eq!(v[0], 1.);
        assert_f64_eq!(v[1], 2.);
        assert_f64_eq!(v[2], 3.);
    }
}
