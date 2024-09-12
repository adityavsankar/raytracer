use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    #[inline]
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    #[inline]
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    #[inline]
    pub fn time(&self) -> &f64 {
        &self.time
    }

    #[inline]
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
