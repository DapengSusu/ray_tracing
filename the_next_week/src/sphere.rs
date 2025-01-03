use std::rc::Rc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray
};
use utils::interval::Interval;
use vector3::{Point3, Vec3};

pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    /// 创建静止球体
    pub fn new_stationary_sphere(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        let rvec = Vec3::isotropic(radius);
        Self {
            center: Ray::new(center, Vec3::zero(), 0.),
            radius: radius.max(0.),
            material,
            bbox: AABB::from_points(center - rvec, center + rvec),
        }
    }

    /// 创建运动球体
    pub fn new_moving_sphere(origin: Point3, end: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        let center = Ray::new(origin, end - origin, 0.);
        let rvec = Vec3::isotropic(radius);
        let box0 = AABB::from_points(center.at(0.) - rvec, center.at(0.) + rvec);
        let box1 = AABB::from_points(center.at(1.) - rvec, center.at(1.) + rvec);
        Self {
            center,
            radius: radius.max(0.),
            material,
            bbox: AABB::combine(&box0, &box1)
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time());
        let oc = current_center - *ray.origin();
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

        let mut hit_record = HitRecord {
            point: ray.at(root),
            material: self.material.clone(),
            t: root,
            ..Default::default()
        };
        let outward_normal = (hit_record.point - current_center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
