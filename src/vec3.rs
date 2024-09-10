use std::{
    iter::Sum,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, RangeBounds, Sub, SubAssign,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Index<u8> for Vec3 {
    type Output = f64;

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

impl From<(f64, f64, f64)> for Vec3 {
    #[inline(always)]
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

impl From<[f64; 3]> for Vec3 {
    #[inline(always)]
    fn from([x, y, z]: [f64; 3]) -> Self {
        Self { x, y, z }
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sum for Vec3 {
    #[inline(always)]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::default(), |a, b| a + b)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random() -> Self {
        Self {
            x: fastrand::f64(),
            y: fastrand::f64(),
            z: fastrand::f64(),
        }
    }

    pub fn random_range(range: impl RangeBounds<f64> + Clone) -> Self {
        Self {
            x: fastrand_contrib::f64_range(range.clone()),
            y: fastrand_contrib::f64_range(range.clone()),
            z: fastrand_contrib::f64_range(range),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::random_range(-1.0..1.0);
            if p.length_sq() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                fastrand_contrib::f64_range(-1.0..1.0),
                fastrand_contrib::f64_range(-1.0..1.0),
                0.0,
            );
            if p.length_sq() < 1.0 {
                return p;
            }
        }
    }

    #[inline(always)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[inline(always)]
    pub fn z(&self) -> f64 {
        self.z
    }

    #[inline(always)]
    pub fn length_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    #[inline(always)]
    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }

    #[inline(always)]
    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    #[inline(always)]
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline(always)]
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    #[inline(always)]
    pub fn near_zero(&self) -> bool {
        const T: f64 = 1e-6;
        self.x.abs() < T && self.y.abs() < T && self.z.abs() < T
    }

    #[inline(always)]
    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * n
    }

    #[inline(always)]
    pub fn refract(&self, n: Vec3, relative_refractive_index: f64) -> Vec3 {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = relative_refractive_index * (*self + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_sq()).abs().sqrt()) * n;

        r_out_perp + r_out_parallel
    }

    #[inline(always)]
    fn linear_to_gamma(component: f64) -> f64 {
        component.sqrt().max(0.0)
    }

    pub fn rgb8(&self) -> [u8; 3] {
        let (start, end) = (0.000, 0.999);
        let r = Self::linear_to_gamma(self.x);
        let g = Self::linear_to_gamma(self.y);
        let b = Self::linear_to_gamma(self.z);

        let r_byte = (256.0 * r.clamp(start, end)) as u8;
        let g_byte = (256.0 * g.clamp(start, end)) as u8;
        let b_byte = (256.0 * b.clamp(start, end)) as u8;

        [r_byte, g_byte, b_byte]
    }
}
