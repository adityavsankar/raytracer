use crate::{
    aabb::Aabb,
    entity::{Entity, HitRecord},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f64,
    material: Arc<dyn Material>,
    bounding_box: Aabb,
}

impl Entity for Quad {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord> {
        let denominator = self.normal.dot(*ray.direction());
        if denominator.abs() < 1e-6 {
            return None;
        }
        let time = (self.d - self.normal.dot(*ray.origin())) / denominator;
        if !time_interval.contains(time) {
            return None;
        }
        let hit_point = ray.at(time);
        let hit_point_vector = hit_point - self.q;
        let alpha = self.w.dot(hit_point_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(hit_point_vector));
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }
        Some(HitRecord::new(
            hit_point,
            ray,
            self.normal,
            time,
            alpha,
            beta,
            &*self.material,
        ))
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let d1 = Aabb::new_from_points(q, q + u + v);
        let d2 = Aabb::new_from_points(q + u, q + v);
        let bounding_box = Aabb::enclose(&d1, &d2);
        let n = u.cross(v);
        let normal = n.unit();
        let d = normal.dot(q);
        let w = n / n.length_sq();
        Self {
            q,
            u,
            v,
            w,
            normal,
            d,
            material,
            bounding_box,
        }
    }
}
