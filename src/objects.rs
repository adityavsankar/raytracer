use crate::{
    bvh::AxisAlignedBoundingBox,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    pub hit_point: Point3,
    pub normal: Vec3,
    pub time: f32,
    pub front: bool,
    pub material: &'a dyn Material,
    pub u: f32,
    pub v: f32,
}

pub trait Object: Send + Sync + std::fmt::Debug {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AxisAlignedBoundingBox;
}

impl<'a> HitRecord<'a> {
    pub fn new(
        hit_point: Point3,
        normal: Vec3,
        time: f32,
        u: f32,
        v: f32,
        material: &'a dyn Material,
    ) -> Self {
        Self {
            hit_point,
            normal,
            time,
            front: true,
            material,
            u,
            v,
        }
    }

    #[inline(always)]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
