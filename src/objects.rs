use crate::{
    aabb::Aabb,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    pub hit_point: Point3,
    pub normal: Vec3,
    pub time: f64,
    pub front: bool,
    pub material: &'a dyn Material,
    pub u: f64,
    pub v: f64,
}

pub trait Object: Send + Sync + std::fmt::Debug {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}

impl<'a> HitRecord<'a> {
    pub fn new(
        hit_point: Point3,
        ray: &Ray,
        outward_normal: Vec3,
        time: f64,
        u: f64,
        v: f64,
        material: &'a dyn Material,
    ) -> Self {
        let front = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            hit_point,
            normal,
            time,
            front,
            material,
            u,
            v,
        }
    }
}
