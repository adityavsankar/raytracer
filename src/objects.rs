use crate::{
    bvh::AxisAlignedBoundingBox,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

pub struct HitRecord<'a> {
    pub hit_point: Point3,
    pub normal: Vec3,
    pub time: f32,
    pub front: bool,
    pub material: &'a dyn Material,
}

pub trait Object: Sync + Send + std::fmt::Debug {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AxisAlignedBoundingBox;
}

impl<'a> HitRecord<'a> {
    pub fn new(hit_point: Point3, normal: Vec3, time: f32, material: &'a dyn Material) -> Self {
        Self {
            hit_point,
            normal,
            time,
            front: true,
            material,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

#[derive(Debug)]
pub struct ObjectList {
    pub objects: Vec<Arc<dyn Object>>,
    b_box: AxisAlignedBoundingBox,
}

impl Object for ObjectList {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord> {
        let mut closest = time_interval.end;
        let mut h = None;
        for object in self.objects.iter() {
            if let Some(hit_record) = object.hit(ray, Interval::new(time_interval.start, closest)) {
                closest = closest.min(hit_record.time);
                h = Some(hit_record);
            }
        }
        h
    }

    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        self.b_box
    }
}

impl ObjectList {
    pub fn new() -> Self {
        let objects = Vec::new();
        let b_box = AxisAlignedBoundingBox::default();
        Self { objects, b_box }
    }

    pub fn push(&mut self, object: Arc<dyn Object>) {
        self.b_box.grow(&object.bounding_box());
        self.objects.push(object);
    }
}
