use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    #[inline(always)]
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    #[inline(always)]
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    #[inline(always)]
    pub fn time(&self) -> f32 {
        self.time
    }

    #[inline(always)]
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
