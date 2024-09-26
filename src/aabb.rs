use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::ops::{Add, Index};

#[derive(Debug, Clone, Copy, Default)]
pub struct Aabb(Interval, Interval, Interval);

impl Add<Vec3> for Aabb {
    type Output = Aabb;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0 + rhs.x(), self.1 + rhs.y(), self.2 + rhs.z())
    }
}

impl Index<u8> for Aabb {
    type Output = Interval;

    #[inline]
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => unreachable!(),
        }
    }
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut s = Self(x, y, z);
        s.pad_to_minimums();
        s
    }

    pub fn new_from_points(a: Point3, b: Point3) -> Self {
        let x = Interval::new(a.x().min(b.x()), a.x().max(b.x()));
        let y = Interval::new(a.y().min(b.y()), a.y().max(b.y()));
        let z = Interval::new(a.z().min(b.z()), a.z().max(b.z()));
        let mut s = Self(x, y, z);
        s.pad_to_minimums();
        s
    }

    pub fn enclose(b0: &Self, b1: &Self) -> Self {
        let x = Interval::enclose(&b0.0, &b1.0);
        let y = Interval::enclose(&b0.1, &b1.1);
        let z = Interval::enclose(&b0.2, &b1.2);
        Self(x, y, z)
    }

    pub fn grow(&mut self, other: &Self) {
        self.0.grow(&other.0);
        self.1.grow(&other.1);
        self.2.grow(&other.2);
    }

    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.0.size() < delta {
            self.1.expand(delta);
        }
        if self.0.size() < delta {
            self.1.expand(delta);
        }
        if self.0.size() < delta {
            self.1.expand(delta);
        }
    }

    pub fn hit(&self, ray: &Ray, mut time_interval: Interval) -> bool {
        let origin = ray.origin();
        let direction = ray.direction();

        for axis in 0..3 {
            let axis_interval = self[axis];
            let ad_inv = 1.0 / direction[axis];

            let (mut t0, mut t1) = (
                (axis_interval.start - origin[axis]) * ad_inv,
                (axis_interval.end - origin[axis]) * ad_inv,
            );

            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > time_interval.start {
                time_interval.start = t0;
            }

            if t1 < time_interval.end {
                time_interval.end = t1;
            }

            if time_interval.end <= time_interval.start {
                return false;
            }
        }

        true
    }

    pub fn x(&self) -> Interval {
        self.0
    }

    pub fn y(&self) -> Interval {
        self.1
    }

    pub fn z(&self) -> Interval {
        self.2
    }
}
