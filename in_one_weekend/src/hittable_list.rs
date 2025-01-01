use std::rc::Rc;

use crate::{hittable::{HitRecord, Hittable}, ray::Ray};
use utils::interval::Interval;

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

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut records = Vec::new();
        let mut is_hit = false;
        let mut close_st = ray_t.max;

        self.0.iter().for_each(|hittable| {
            if let Some(record) = hittable.hit(ray, Interval::new(ray_t.min, close_st)) {
                is_hit = true;
                close_st = record.t;
                records.push(record);
            }
        });

        if is_hit {
            return Some(records.pop().unwrap());
        }
        None
    }
}
