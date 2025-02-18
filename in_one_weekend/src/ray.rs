use vector3::{Point3, Vec3};

/// ## A ray in 3D space.
/// **Origin** is the starting point of the ray.
/// **Direction** is the direction of the ray.
#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}
