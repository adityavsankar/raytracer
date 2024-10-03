use crate::{
    aabb::Aabb,
    entity::{Entity, EntityCluster, HitRecord},
    interval::Interval,
    material::Material,
    quad::Quad,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Cuboid {
    faces: EntityCluster,
}

impl Cuboid {
    pub fn new(a: Point3, b: Point3, material: Arc<dyn Material>) -> Self {
        let mut faces = EntityCluster::new();
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        faces.push(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            material.clone(),
        ))); // front

        faces.push(Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), max.z()),
            -dz,
            dy,
            material.clone(),
        ))); // right

        faces.push(Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), min.z()),
            -dx,
            dy,
            material.clone(),
        ))); // back

        faces.push(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dz,
            dy,
            material.clone(),
        ))); // left

        faces.push(Arc::new(Quad::new(
            Point3::new(min.x(), max.y(), max.z()),
            dx,
            -dz,
            material.clone(),
        ))); // top

        faces.push(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            material,
        ))); // bottom

        Self { faces }
    }
}

impl Entity for Cuboid {
    fn hit(&self, ray: &Ray, time_interval: Interval) -> Option<HitRecord> {
        self.faces.hit(ray, time_interval)
    }

    #[inline]
    fn bounding_box(&self) -> Aabb {
        self.faces.bounding_box()
    }
}
