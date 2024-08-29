use crate::{
    bvh::AxisAlignedBoundingBox,
    interval::Interval,
    material::Material,
    objects::{HitRecord, Object},
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Sphere {
    center1: Point3,
    radius: f32,
    mat: Arc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
    b_box: AxisAlignedBoundingBox,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord> {
        let center = if self.is_moving {
            self.sphere_center(ray.time())
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
        let (u, v) = self.get_uv(&outward_normal);
        let mut h = HitRecord::new(hit_point, outward_normal, root, u, v, &*self.mat);
        h.set_face_normal(ray, outward_normal);

        Some(h)
    }

    #[inline(always)]
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        self.b_box
    }
}

impl Sphere {
    pub fn stationary(center1: Point3, radius: f32, mat: Arc<dyn Material>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let b_box = AxisAlignedBoundingBox::new_from_points(center1 - r_vec, center1 + r_vec);
        Sphere {
            center1,
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::default(),
            b_box,
        }
    }

    pub fn moving(center1: Point3, center2: Point3, radius: f32, mat: Arc<dyn Material>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let box1 = AxisAlignedBoundingBox::new_from_points(center1 - r_vec, center1 + r_vec);
        let box2 = AxisAlignedBoundingBox::new_from_points(center2 - r_vec, center2 + r_vec);
        let b_box = AxisAlignedBoundingBox::enclose(&box1, &box2);
        Sphere {
            center1,
            radius,
            mat,
            is_moving: true,
            center_vec: center2 - center1,
            b_box,
        }
    }

    #[inline(always)]
    fn sphere_center(&self, time: f32) -> Point3 {
        self.center1 + self.center_vec * time
    }

    fn get_uv(&self, p: &Point3) -> (f32, f32) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + std::f32::consts::PI;
        let u = phi * 0.5 * std::f32::consts::FRAC_1_PI;
        let v = theta * std::f32::consts::FRAC_1_PI;
        (u, v)
    }
}
