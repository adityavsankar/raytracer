use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::ops::{Add, Index};

#[derive(Debug, Clone, Copy, Default)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Add<Vec3> for Aabb {
    type Output = Aabb;

    #[inline(always)]
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl Index<u8> for Aabb {
    type Output = Interval;

    #[inline(always)]
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code, reason = "Allow for multiple constructors")]
impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut s = Self { x, y, z };
        s.pad_to_minimums();
        s
    }

    pub fn new_from_points(a: Point3, b: Point3) -> Self {
        let x = Interval::new(a.x().min(b.x()), a.x().max(b.x()));
        let y = Interval::new(a.y().min(b.y()), a.y().max(b.y()));
        let z = Interval::new(a.z().min(b.z()), a.z().max(b.z()));
        let mut s = Self { x, y, z };
        s.pad_to_minimums();
        s
    }

    pub fn enclose(b0: &Self, b1: &Self) -> Self {
        let x = Interval::enclose(&b0.x, &b1.x);
        let y = Interval::enclose(&b0.y, &b1.y);
        let z = Interval::enclose(&b0.z, &b1.z);
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn grow(&mut self, other: &Self) {
        self.x.grow(&other.x);
        self.y.grow(&other.y);
        self.z.grow(&other.z);
    }

    #[inline(always)]
    pub fn longest_axis(&self) -> u8 {
        let mut max = 0.0;
        let mut max_axis = 0;
        for axis in 0..3 {
            let size = self[axis].size();
            if size > max {
                max = size;
                max_axis = axis;
            }
        }
        max_axis
    }

    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z.expand(delta);
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
}
