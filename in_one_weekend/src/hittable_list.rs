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
        let mut close_st = ray_t.max;

        // 使用filter_map来收集可能的结果
        let hit_record = self.0.iter().filter_map(|hittable| {
            if let Some(record) = hittable.hit(ray, Interval::new(ray_t.min, close_st)) {
                close_st = record.t;
                Some(record)
            } else {
                None
            }
        }).last(); // 取得最后一个结果

        hit_record
    }
}
