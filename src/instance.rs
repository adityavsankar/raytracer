use core::f64;
use std::sync::Arc;

use crate::{
    aabb::Aabb,
    entity::{Entity, HitRecord},
    interval::Interval,
    mat3::Mat3,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct Translated {
    entity: Arc<dyn Entity>,
    offset: Vec3,
    bounding_box: Aabb,
}

impl Translated {
    pub fn new(entity: Arc<dyn Entity>, offset: Vec3) -> Self {
        let bounding_box = entity.bounding_box() + offset;
        Self {
            entity,
            offset,
            bounding_box,
        }
    }
}

impl Entity for Translated {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord> {
        let offset_ray = Ray::new(*ray.origin() - self.offset, *ray.direction(), *ray.time());
        if let Some(mut hit_record) = self.entity.hit(&offset_ray, time_interval) {
            hit_record.hit_point += self.offset;
            Some(hit_record)
        } else {
            None
        }
    }

    #[inline]
    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}

#[derive(Debug, Clone)]
pub struct Rotated {
    entity: Arc<dyn Entity>,
    bounding_box: Aabb,
    rotation_matrix: Mat3,
    inverse_rotation_matrix: Mat3,
}

impl Rotated {
    pub fn new(entity: Arc<dyn Entity>, rotation: Vec3) -> Self {
        let rotation_matrix = Mat3::rotation_x(rotation.x())
            * Mat3::rotation_y(rotation.y())
            * Mat3::rotation_z(rotation.z());
        let inverse_rotation_matrix = rotation_matrix.transpose();

        let b_box = entity.bounding_box();
        let mut a = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut b = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let (i, j, k) = (i as f64, j as f64, k as f64);

                    let x = i * b_box.x().end + (1.0 - i) * b_box.x().start;
                    let y = j * b_box.y().end + (1.0 - j) * b_box.y().start;
                    let z = k * b_box.z().end + (1.0 - k) * b_box.z().start;

                    let rotated_point = rotation_matrix * Point3::new(x, y, z);

                    a = Point3::new(
                        a.x().min(rotated_point.x()),
                        a.y().min(rotated_point.y()),
                        a.z().min(rotated_point.z()),
                    );

                    b = Point3::new(
                        b.x().max(rotated_point.x()),
                        b.y().max(rotated_point.y()),
                        b.z().max(rotated_point.z()),
                    );
                }
            }
        }

        let bounding_box = Aabb::new_from_points(a, b);

        Self {
            entity,
            bounding_box,
            rotation_matrix,
            inverse_rotation_matrix,
        }
    }
}

impl Entity for Rotated {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord> {
        let origin = self.inverse_rotation_matrix * *ray.origin();
        let direction = self.inverse_rotation_matrix * *ray.direction();
        let rotated_ray = Ray::new(origin, direction, *ray.time());

        if let Some(mut hit_record) = self.entity.hit(&rotated_ray, time_interval) {
            hit_record.hit_point = self.rotation_matrix * hit_record.hit_point;
            hit_record.normal = self.rotation_matrix * hit_record.normal;
            Some(hit_record)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
