use std::rc::Rc;
use crate::{hittable::{HitRecord, Hittable}, ray::Ray};

pub struct HittableList(Vec<Rc<dyn Hittable>>);

impl HittableList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, hittable: Rc<dyn Hittable>) {
        self.0.push(hittable);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut hit_record = HitRecord::default();
        let mut is_hit = false;
        let mut close_st = ray_tmax;

        self.0.iter().for_each(|hittable| {
            if let Some(record) = hittable.hit(ray, ray_tmin, close_st) {
                is_hit = true;
                close_st = record.t;
                hit_record = record;
            }
        });

        if is_hit {
            return Some(hit_record);
        }
        None
    }
}
