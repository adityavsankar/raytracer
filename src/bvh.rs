use crate::{
    aabb::Aabb,
    interval::Interval,
    objects::{HitRecord, Object},
    ray::Ray,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct BVHNode {
    bounding_box: Aabb,
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
    fn bounding_box(&self) -> Aabb {
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

        let bounding_box = Aabb::enclose(&left.bounding_box(), &right.bounding_box());

        Self {
            bounding_box,
            left,
            right,
        }
    }
}
