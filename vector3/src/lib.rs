use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign}
};

extern crate nalgebra as na;

#[derive(Default, Clone, Copy)]
pub struct Vec3(na::Vector3<f64>);

pub type Point3 = Vec3;

impl Vec3 {
    pub fn zero() -> Self {
        Self(na::Vector3::new(0., 0., 0.))
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(na::Vector3::new(x, y, z))
    }

    pub fn from_x(x: f64) -> Self {
        Self::new(x, 0., 0.)
    }

    pub fn from_y(y: f64) -> Self {
        Self::new(0., y, 0.)
    }

    pub fn from_z(z: f64) -> Self {
        Self::new(0., 0., z)
    }

    pub fn from_nalgebra(vec: na::Vector3<f64>) -> Self {
        Self(vec)
    }

    pub fn x(&self) -> f64 {
        self.0.x
    }

    pub fn y(&self) -> f64 {
        self.0.y
    }

    pub fn z(&self) -> f64 {
        self.0.z
    }

    pub fn set_x(&mut self, x: f64) {
        self.0.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.0.y = y;
    }

    pub fn set_z(&mut self, z: f64) {
        self.0.z = z;
    }

    pub fn sub(self, rhs: &Self) -> Self {
        Self(self.0 - rhs.0)
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0.dot(&rhs.0)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self(self.0.cross(&rhs.0))
    }

    pub fn scale(&self, rhs: f64) -> Self {
        Self(self.0.scale(rhs))
    }

    pub fn unscale(&self, rhs: f64) -> Self {
        Self(self.0.unscale(rhs))
    }

    pub fn scale_to_vec3(&self, rhs: &Self) -> Self {
        Self(self.0.scale(1. / rhs.norm()))
    }

    pub fn unscale_from_vec3(&self, rhs: &Self) -> Self {
        Self(self.0.unscale(1. / rhs.norm()))
    }

    pub fn unit(&self) -> Self {
        self.scale_to_vec3(self)
    }

    pub fn normalize(&self) -> Self {
        Self(self.0.normalize())
    }

    pub fn squared(&self) -> f64 {
        self.0.norm_squared()
    }

    pub fn magnitude(&self) -> f64 {
        self.0.magnitude()
    }

    pub fn norm(&self) -> f64 {
        self.0.norm()
    }

    pub fn to_nalgebra(&self) -> na::Vector3<f64> {
        self.0
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({},{},{})", self.x(), self.y(), self.z())
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        utils::check_f64_eq!(self.x(), other.x()) &&
        utils::check_f64_eq!(self.y(), other.y()) &&
        utils::check_f64_eq!(self.z(), other.z())
    }
}

impl Eq for Vec3 {}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self * rhs.x(),
            self * rhs.y(),
            self * rhs.z()
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_nalgebra() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.to_nalgebra(), na::Vector3::new(1., 2., 3.));
    }

    #[test]
    fn test_new() {
        let v = Vec3::zero();
        assert_eq!(v.0, na::Vector3::new(0., 0., 0.));

        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.0, na::Vector3::new(1., 2., 3.));

        let v = Vec3::from_nalgebra(na::Vector3::new(1., 2., 3.));
        assert_eq!(v, Vec3::new(1., 2., 3.));
    }

    #[test]
    fn test_axis() {
        let v = Vec3::new(1., 2., 3.);
        utils::assert_f64_eq!(v.x(), 1.);
        utils::assert_f64_eq!(v.y(), 2.);
        utils::assert_f64_eq!(v.z(), 3.);
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
        let nv = -v;
        assert_eq!(nv.0, na::Vector3::new(-1., -2., -3.));
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
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(9., 18., 27.);
        assert_eq!(v / 3., Vec3::new(3., 6., 9.));
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);
        assert_eq!(v1.dot(&v2), v1.to_nalgebra().dot(&v2.to_nalgebra()));
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);
        assert_eq!(
            v1.cross(&v2),
            Vec3::from_nalgebra(v1.to_nalgebra().cross(&v2.to_nalgebra()))
        );
    }

    #[test]
    fn test_scale() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);
        assert_eq!(
            v1.scale_to_vec3(&v2).to_nalgebra(),
            v1.to_nalgebra().scale(1. / &v2.to_nalgebra().magnitude())
        );

        let v = Vec3::new(4., 3., 2.);
        let v1 = v.scale_to_vec3(&v);
        assert_eq!(v, v1.unscale_from_vec3(&v));
        let v2 = v.scale(2.);
        assert_eq!(v, v2.unscale(2.));
    }

    #[test]
    fn test_normalize() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(
            v.normalize(),
            Vec3::from_nalgebra(v.to_nalgebra().normalize())
        );
    }

    #[test]
    fn test_squared() {
        let v = Vec3::new(1., 2., 3.);
        utils::assert_f64_eq!(v.squared(), v.to_nalgebra().norm_squared());
    }

    #[test]
    fn test_magnitude() {
        let v = Vec3::new(1., 2., 3.);
        utils::assert_f64_eq!(v.magnitude(), v.to_nalgebra().magnitude());
    }

    #[test]
    fn test_norm() {
        let v = Vec3::new(1., 2., 3.);
        utils::assert_f64_eq!(v.norm(), v.to_nalgebra().norm());
    }
}
