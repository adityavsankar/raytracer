use std::sync::Arc;

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

    pub fn raw(
        hit_point: Point3,
        normal: Vec3,
        time: f64,
        front: bool,
        u: f64,
        v: f64,
        material: &'a dyn Material,
    ) -> Self {
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

pub trait Entity: Send + Sync + std::fmt::Debug {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}

#[derive(Debug, Clone)]
pub struct EntityCluster {
    entities: Vec<Arc<dyn Entity>>,
    bounding_box: Aabb,
}

impl Entity for EntityCluster {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord> {
        let mut closest = time_interval.end;
        let mut result = None;
        for entity in &self.entities {
            if let Some(hit_record) = entity.hit(ray, Interval::new(time_interval.start, closest)) {
                closest = hit_record.time;
                result = Some(hit_record);
            }
        }
        result
    }

    #[inline]
    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}

impl EntityCluster {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            bounding_box: Aabb::default(),
        }
    }

    pub fn push(&mut self, entity: Arc<dyn Entity>) {
        self.bounding_box.grow(&entity.bounding_box());
        self.entities.push(entity);
    }
}
