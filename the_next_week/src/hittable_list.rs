use std::rc::Rc;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, ray::Ray};
use utils::interval::Interval;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB
}

impl HittableList {
    pub fn from_hittable(object: Rc<dyn Hittable>) -> Self {
        let mut hittable_list = HittableList::default();
        hittable_list.add(object);

        hittable_list
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = AABB::combine(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut close_st = ray_t.max;

        // 使用filter_map来收集可能的结果
        let hit_record = self.objects.iter().filter_map(|hittable| {
            if let Some(record) = hittable.hit(ray, Interval::new(ray_t.min, close_st)) {
                close_st = record.t;
                Some(record)
            } else {
                None
            }
        }).last(); // 取得最后一个结果

        hit_record
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
