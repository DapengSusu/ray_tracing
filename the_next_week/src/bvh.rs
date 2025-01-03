use std::rc::Rc;

use crate::{
    aabb::{self, AABB},
    hittable::{HitRecord, Hittable, InvalidHittable},
    hittable_list::HittableList,
    ray::Ray
};
use utils::interval::Interval;

pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn from_hittable_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        BVHNode::new(&mut list.objects, 0, len)
    }

    pub fn new(objects: &mut Vec<Rc<dyn Hittable>>, begin: usize, end: usize) -> Self {
        let mut bvh_node = BVHNode::default();

        // Build the bounding box of the span of source objects.
        bvh_node.bbox = aabb::EMPTY;
        for object_index in begin..end {
            bvh_node.bbox = AABB::combine(&bvh_node.bbox, objects[object_index].bounding_box());
        }

        let object_span = end - begin;
        match object_span {
            1 => {
                bvh_node.left = objects[begin].clone();
                bvh_node.right = objects[begin].clone();
            },
            2 => {
                bvh_node.left = objects[begin].clone();
                bvh_node.right = objects[begin + 1].clone();
            },
            _ => {
                let axis = bvh_node.bbox.longest_axis();
                objects[begin..end].sort_by(|a, b| {
                    let a_axis_interval = a.bounding_box().axis_interval(axis);
                    let b_axis_interval = b.bounding_box().axis_interval(axis);

                    a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
                });
                let mid = begin + object_span / 2;
                bvh_node.left = Rc::new(BVHNode::new(objects, begin, mid));
                bvh_node.right = Rc::new(BVHNode::new(objects, mid, end));
            }
        }

        bvh_node
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, mut ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, &mut ray_t) {
            return None;
        }

        let left_record = self.left.hit(ray, ray_t);
        let mut interval = ray_t;
        if let Some(ref record) = left_record {
            interval.max = record.t;
        }
        let right_record = self.right.hit(ray, interval);

        // 如果 right_record 不是 None，则返回 right_record，否则返回 left_record
        // 都为 None 时，返回 None
        right_record.or(left_record)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

impl Default for BVHNode {
    fn default() -> Self {
        Self {
            left: Rc::new(InvalidHittable),
            right: Rc::new(InvalidHittable),
            bbox: AABB::default(),
        }
    }
}
