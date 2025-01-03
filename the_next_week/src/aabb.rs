use utils::interval::{self, Interval};
use vector3::Point3;
use crate::ray::Ray;

pub const EMPTY: AABB = AABB { x: interval::EMPTY, y: interval::EMPTY, z: interval::EMPTY };
pub const UNIVERSE: AABB = AABB { x: interval::UNIVERSE, y: interval::UNIVERSE, z: interval::UNIVERSE };

/// Axis-Aligned Bounding Boxes (AABBs)
/// 轴对齐边界框
#[derive(Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.
        Self {
            x: if a.x <= b.x { Interval::new(a.x, b.x) } else { Interval::new(b.x, a.x) },
            y: if a.y <= b.y { Interval::new(a.y, b.y) } else { Interval::new(b.y, a.y) },
            z: if a.z <= b.z { Interval::new(a.z, b.z) } else { Interval::new(b.z, a.z) }
        }
    }

    pub fn combine(box0: &AABB, box1: &AABB) -> Self {
        Self {
            x: Interval::from_intervals(box0.x, box1.x),
            y: Interval::from_intervals(box0.y, box1.y),
            z: Interval::from_intervals(box0.z, box1.z)
        }
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> bool {
        let ray_orig = ray.origin();
        let ray_dir = ray.direction();

        for axis in 0..3_usize {
            let ax = self.axis_interval(axis);
            let adinv = ray_dir[axis].recip();

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                ray_t.min = t0.max(ray_t.min);
                ray_t.max = t1.min(ray_t.max);
            } else {
                ray_t.min = t1.max(ray_t.min);
                ray_t.max = t0.min(ray_t.max);
            }

            if ray_t.min >= ray_t.max {
                return false;
            }
        }

        true
    }

    /// 返回包围盒最长轴的索引
    /// x: 0
    /// y: 1
    /// z: 2
    pub fn longest_axis(&self) -> usize {
        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();

        let max_axis = [x_size, y_size, z_size].iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;

        max_axis
    }
}
