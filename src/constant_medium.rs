use core::f64;
use std::sync::Arc;

use crate::{
    aabb::Aabb,
    entity::{Entity, HitRecord},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

#[derive(Debug, Clone)]
pub struct ConstantMedium {
    boundary: Arc<dyn Entity>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Entity>, density: f64, phase_function: Arc<dyn Material>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }
}

impl Entity for ConstantMedium {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord> {
        let mut t1 = self
            .boundary
            .hit(ray, Interval::new(f64::NEG_INFINITY, f64::INFINITY))?
            .time;
        let mut t2 = self
            .boundary
            .hit(ray, Interval::new(t1 + 0.0001, f64::INFINITY))?
            .time;

        if t1 < time_interval.start {
            t1 = time_interval.start;
        }

        if t2 > time_interval.end {
            t2 = time_interval.end;
        }

        if t1 >= t2 {
            return None;
        }

        if t1 < 0.0 {
            t1 = 0.0;
        }

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (t2 - t1) * ray_length;
        let hit_distance = self.neg_inv_density * fastrand::f64().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let time = t1 + hit_distance / ray_length;

        Some(HitRecord::raw(
            ray.at(time),
            Vec3::new(1.0, 0.0, 0.0),
            time,
            true,
            0.0,
            0.0,
            &*self.phase_function,
        ))
    }

    #[inline]
    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
