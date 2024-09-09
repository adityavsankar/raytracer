use crate::{interval::Interval, ray::Ray, vec3::Point3};
use std::ops::Index;

#[derive(Debug, Clone, Copy, Default)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Index<u8> for AABB {
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
impl AABB {
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
            let t0 = (axis_interval.start - origin[axis]) * ad_inv;
            let t1 = (axis_interval.end - origin[axis]) * ad_inv;

            if t0 < t1 {
                if t0 > time_interval.start {
                    time_interval.start = t0;
                }
                if t1 < time_interval.end {
                    time_interval.end = t1;
                }
            } else {
                if t1 > time_interval.start {
                    time_interval.start = t1;
                }
                if t0 < time_interval.end {
                    time_interval.end = t0;
                }
            }

            if time_interval.end <= time_interval.start {
                return false;
            }
        }

        true
    }
}