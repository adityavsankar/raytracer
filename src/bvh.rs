use crate::{
    interval::Interval,
    objects::{HitRecord, Object},
    ray::Ray,
    vec3::Point3,
};
use std::{ops::Index, sync::Arc};

#[derive(Debug, Clone, Copy, Default)]
pub struct AxisAlignedBoundingBox {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Index<u8> for AxisAlignedBoundingBox {
    type Output = Interval;

    #[inline(always)]
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code, reason = "Allow for multiple constructors")]
impl AxisAlignedBoundingBox {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn new_from_points(a: Point3, b: Point3) -> Self {
        let x = if a.x() <= b.x() {
            Interval::new(a.x(), b.x())
        } else {
            Interval::new(b.x(), a.x())
        };
        let y = if a.y() <= b.y() {
            Interval::new(a.y(), b.y())
        } else {
            Interval::new(b.y(), a.y())
        };
        let z = if a.z() <= b.z() {
            Interval::new(a.z(), b.z())
        } else {
            Interval::new(b.z(), a.z())
        };
        Self { x, y, z }
    }

    pub fn enclose(b0: &Self, b1: &Self) -> Self {
        let x = Interval::enclose(&b0.x, &b1.x);
        let y = Interval::enclose(&b0.y, &b1.y);
        let z = Interval::enclose(&b0.z, &b1.z);
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn grow(&mut self, other: &Self) {
        self.x.grow(&other.x);
        self.y.grow(&other.y);
        self.z.grow(&other.z);
    }

    #[inline(always)]
    pub fn longest_axis(&self) -> u8 {
        let mut max = 0.0;
        let mut max_axis = 0;
        for axis in 0..3 {
            let size = self[axis].size();
            if size > max {
                max = size;
                max_axis = axis;
            }
        }
        max_axis
    }

    pub fn hit(&self, ray: &Ray, mut time_interval: Interval) -> bool {
        let origin = ray.origin();
        let direction = ray.direction();

        for axis in 0..3 {
            let axis_interval = self[axis];
            let ad_inv = 1.0 / direction[axis];
            let t0 = (axis_interval.start - origin[axis]) * ad_inv;
            let t1 = (axis_interval.end - origin[axis]) * ad_inv;

            if t0 < t1 {
                if t0 > time_interval.start {
                    time_interval.start = t0;
                }
                if t1 < time_interval.end {
                    time_interval.end = t1;
                }
            } else {
                if t1 > time_interval.start {
                    time_interval.start = t1;
                }
                if t0 < time_interval.end {
                    time_interval.end = t0;
                }
            }

            if time_interval.end <= time_interval.start {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
pub struct BVHNode {
    bounding_box: AxisAlignedBoundingBox,
    left: Arc<dyn Object>,
    right: Arc<dyn Object>,
}

impl Object for BVHNode {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, time_interval) {
            return None;
        }
        let hit_left = self.left.hit(ray, time_interval);
        let hit_right = self.right.hit(ray, time_interval);

        match (hit_left, hit_right) {
            (Some(hl), Some(hr)) => Some(if hl.time < hr.time { hl } else { hr }),
            (Some(hl), None) => Some(hl),
            (None, Some(hr)) => Some(hr),
            (None, None) => None,
        }
    }

    #[inline(always)]
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bounding_box
    }
}

impl BVHNode {
    pub fn new(objects: &mut [Arc<dyn Object>]) -> Self {
        let axis = fastrand::u8(0..=2);
        let object_span = objects.len();
        let (left, right) = match object_span {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => (objects[0].clone(), objects[1].clone()),
            _ => {
                objects.sort_by(|a, b| {
                    let x = a.bounding_box()[axis].start;
                    let y = b.bounding_box()[axis].start;
                    x.partial_cmp(&y).unwrap()
                });
                let mid = object_span / 2;
                let left = Arc::new(BVHNode::new(&mut objects[..mid])) as Arc<dyn Object>;
                let right = Arc::new(BVHNode::new(&mut objects[mid..])) as Arc<dyn Object>;
                (left, right)
            }
        };

        let bounding_box =
            AxisAlignedBoundingBox::enclose(&left.bounding_box(), &right.bounding_box());

        Self {
            bounding_box,
            left,
            right,
        }
    }
}
