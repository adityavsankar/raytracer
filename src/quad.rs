use std::sync::Arc;

use crate::{
    bvh::AxisAlignedBoundingBox,
    interval::Interval,
    material::Material,
    objects::{HitRecord, Object},
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f32,
    material: Arc<dyn Material>,
    b_box: AxisAlignedBoundingBox,
}

impl Object for Quad {
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

    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        self.b_box
    }
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let d1 = AxisAlignedBoundingBox::new_from_points(q, q + u + v);
        let d2 = AxisAlignedBoundingBox::new_from_points(q + u, q + v);
        let b_box = AxisAlignedBoundingBox::enclose(&d1, &d2);
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
            b_box,
        }
    }
}
