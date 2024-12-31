use vector3::Point3;
use crate::{hittable::{HitRecord, Hittable}, ray::Ray};

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - *ray.origin();
        let a = ray.direction().squared();
        let h = ray.direction().dot(&oc);
        let c = oc.squared() - self.radius.powi(2);
        let disc = h*h - a*c;
        if disc < 0. {
            return None;
        }

        let sqrt_disc = disc.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrt_disc) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrt_disc) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }
        let mut hit_record = HitRecord::default();
        hit_record.t = root;
        hit_record.point = ray.at(root);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}
