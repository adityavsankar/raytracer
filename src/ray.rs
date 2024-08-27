use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn new_with_time(origin: Point3, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
