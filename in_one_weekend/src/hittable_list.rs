use std::rc::Rc;

use crate::{hittable::{HitRecord, Hittable}, ray::Ray};
use utils::interval::Interval;

#[derive(Default)]
pub struct HittableList(Vec<Rc<dyn Hittable>>);

impl HittableList {
    pub fn add(&mut self, hittable: Rc<dyn Hittable>) {
        self.0.push(hittable);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_record = HitRecord::default();
        let mut is_hit = false;
        let mut close_st = ray_t.max;

        self.0.iter().for_each(|hittable| {
            if let Some(record) = hittable.hit(ray, Interval::new(ray_t.min, close_st)) {
                is_hit = true;
                close_st = record.t;
                hit_record = record;
            }
        });

        if is_hit {
            Some(hit_record)
        } else {
            None
        }
    }
}
