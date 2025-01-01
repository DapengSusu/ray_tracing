use std::rc::Rc;

use crate::{hittable::{HitRecord, Hittable}, material::Material, ray::Ray};
use utils::interval::Interval;
use vector3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere { center, radius: radius.max(0.), material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - *ray.origin();
        let a = ray.direction().squared();
        let h = ray.direction().dot(&oc);
        let c = oc.squared() - self.radius*self.radius;

        let disc = h*h - a*c;
        if disc < 0. {
            return None;
        }

        let sqrt_disc = disc.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrt_disc) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_disc) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut hit_record = HitRecord::default();
        hit_record.t = root;
        hit_record.point = ray.at(root);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = self.material.clone();

        Some(hit_record)
    }
}
