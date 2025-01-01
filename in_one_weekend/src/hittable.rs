use std::rc::Rc;

use crate::{material::{DefaultMaterial, Material}, ray::Ray};
use utils::interval::Interval;
use vector3::{Point3, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Point3::default(),
            normal: Vec3::default(),
            material: Rc::new(DefaultMaterial::default()),
            t: 0.,
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = ray.direction().dot(&outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
