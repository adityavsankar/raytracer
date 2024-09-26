use crate::{
    aabb::Aabb,
    entity::{Entity, HitRecord},
    interval::Interval,
    ray::Ray,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct BVHNode {
    bounding_box: Aabb,
    left: Arc<dyn Entity>,
    right: Arc<dyn Entity>,
}

impl Entity for BVHNode {
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

    #[inline]
    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}

impl BVHNode {
    pub fn new(entities: &mut [Arc<dyn Entity>]) -> Self {
        let axis = fastrand::u8(0..=2);
        let entity_span = entities.len();
        let (left, right) = match entity_span {
            1 => (entities[0].clone(), entities[0].clone()),
            2 => (entities[0].clone(), entities[1].clone()),
            _ => {
                entities.sort_by(|a, b| {
                    let x = a.bounding_box()[axis].start;
                    let y = b.bounding_box()[axis].start;
                    x.partial_cmp(&y).unwrap()
                });
                let mid = entity_span / 2;
                let left = Arc::new(BVHNode::new(&mut entities[..mid])) as Arc<dyn Entity>;
                let right = Arc::new(BVHNode::new(&mut entities[mid..])) as Arc<dyn Entity>;
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
