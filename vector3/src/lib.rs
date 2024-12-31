use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Neg, Sub, SubAssign}
};

extern crate nalgebra as na;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub mtx: na::Vector3<f64>
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 { mtx: na::Vector3::new(0., 0., 0.) }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { mtx: na::Vector3::new(x, y, z) }
    }

    pub fn from_nalgebra(vec: na::Vector3<f64>) -> Vec3 {
        Vec3 { mtx: vec }
    }

    pub fn x(&self) -> f64 {
        self.mtx.x
    }

    pub fn y(&self) -> f64 {
        self.mtx.y
    }

    pub fn z(&self) -> f64 {
        self.mtx.z
    }

    pub fn set_x(&mut self, x: f64) {
        self.mtx.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.mtx.y = y;
    }

    pub fn set_z(&mut self, z: f64) {
        self.mtx.z = z;
    }

    pub fn mul(&self, rhs: f64) -> Vec3 {
        Vec3 { mtx: self.mtx * rhs }
    }

    pub fn div(&self, rhs: f64) -> Vec3 {
        Vec3 { mtx: self.mtx / rhs }
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.mtx.dot(&rhs.mtx)
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 { mtx: self.mtx.cross(&rhs.mtx) }
    }

    pub fn normalize(&self) -> Vec3 {
        Vec3 { mtx: self.mtx.normalize() }
    }

    pub fn squared(&self) -> f64 {
        self.mtx.norm_squared()
    }

    pub fn magnitude(&self) -> f64 {
        self.mtx.magnitude()
    }

    pub fn norm(&self) -> f64 {
        self.mtx.norm()
    }

    pub fn to_nalgebra(&self) -> na::Vector3<f64> {
        self.mtx
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {})", self.x(), self.y(), self.z())
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({},{},{})", self.x(), self.y(), self.z())
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.mtx == other.mtx
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { mtx: self.mtx + rhs.mtx }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.mtx += rhs.mtx;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { mtx: self.mtx - rhs.mtx }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.mtx -= rhs.mtx;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { mtx: -self.mtx }
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
        assert_eq!(v.mtx, na::Vector3::new(0., 0., 0.));

        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.mtx, na::Vector3::new(1., 2., 3.));

        let v = Vec3::from_nalgebra(na::Vector3::new(1., 2., 3.));
        assert_eq!(v, Vec3::new(1., 2., 3.));
    }

    #[test]
    fn test_axis() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.x(), 1.);
        assert_eq!(v.y(), 2.);
        assert_eq!(v.z(), 3.);
    }

    #[test]
    fn test_debug() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(format!("{:?}", v), "Vec3(1,2,3)");
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1., 2., 3.);
        let nv = -v;
        assert_eq!(nv.mtx, na::Vector3::new(-1., -2., -3.));
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
        assert_eq!(v.mul(2.), Vec3::new(2., 4., 6.));
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.div(2.), Vec3::new(0.5, 1., 1.5));
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);
        assert_eq!(v1.dot(v2), v1.to_nalgebra().dot(&v2.to_nalgebra()));
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);
        assert_eq!(
            v1.cross(v2),
            Vec3::from_nalgebra(v1.to_nalgebra().cross(&v2.to_nalgebra()))
        );
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
        assert_eq!(v.squared(), v.to_nalgebra().norm_squared());
    }

    #[test]
    fn test_magnitude() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.magnitude(), v.to_nalgebra().magnitude());
    }

    #[test]
    fn test_norm() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.norm(), v.to_nalgebra().norm());
    }
}
