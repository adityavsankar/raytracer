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
pub struct Sphere {
    center1: Point3,
    radius: f64,
    material: Arc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
    bounding_box: Aabb,
}

impl Sphere {
    pub fn stationary(center1: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let b_box = Aabb::new_from_points(center1 - r_vec, center1 + r_vec);
        Sphere {
            center1,
            radius,
            material,
            is_moving: false,
            center_vec: Vec3::default(),
            bounding_box: b_box,
        }
    }

    pub fn moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::new_from_points(center1 - r_vec, center1 + r_vec);
        let box2 = Aabb::new_from_points(center2 - r_vec, center2 + r_vec);
        let b_box = Aabb::enclose(&box1, &box2);
        Sphere {
            center1,
            radius,
            material,
            is_moving: true,
            center_vec: center2 - center1,
            bounding_box: b_box,
        }
    }

    #[inline]
    fn sphere_center(&self, time: f64) -> Point3 {
        self.center1 + self.center_vec * time
    }

    fn get_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + std::f64::consts::PI;
        let u = phi * 0.5 * std::f64::consts::FRAC_1_PI;
        let v = theta * std::f64::consts::FRAC_1_PI;
        (u, v)
    }
}

impl Entity for Sphere {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord> {
        let center = if self.is_moving {
            self.sphere_center(*ray.time())
        } else {
            self.center1
        };
        let oc = center - *ray.origin();
        let a = ray.direction().length_sq();
        let half_b = ray.direction().dot(oc);
        let c = oc.length_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let inv_a = 1.0 / a;
        let mut root = (half_b - sqrt_d) * inv_a;

        if !time_interval.surrounds(root) {
            root = (half_b + sqrt_d) * inv_a;
            if !time_interval.surrounds(root) {
                return None;
            }
        }

        let hit_point = ray.at(root);
        let outward_normal = (hit_point - center) / self.radius;
        let (u, v) = Self::get_uv(&outward_normal);
        Some(HitRecord::new(
            hit_point,
            ray,
            outward_normal,
            root,
            u,
            v,
            &*self.material,
        ))
    }

    #[inline]
    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
