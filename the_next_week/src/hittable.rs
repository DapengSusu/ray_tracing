use std::rc::Rc;

use crate::{aabb::AABB, material::{InvalidMaterial, Material}, ray::Ray};
use utils::interval::Interval;
use vector3::{Point3, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &AABB;
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
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = ray.direction().dot(&outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Point3::default(),
            normal: Vec3::default(),
            material: Rc::new(InvalidMaterial),
            t: 0.,
            front_face: false
        }
    }
}

#[derive(Default)]
pub struct InvalidHittable;

impl Hittable for InvalidHittable {
    fn hit(&self, _ray: &Ray, _ray_t: Interval) -> Option<HitRecord> {
        unimplemented!("InvalidHittable hit")
    }

    fn bounding_box(&self) -> &AABB {
        unimplemented!("InvalidHittable bounding_box")
    }
}
