use std::{
    iter::Sum,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, RangeBounds, Sub, SubAssign,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec3(f64, f64, f64);
pub type Point3 = Vec3;
pub type Color = Vec3;

impl Index<u8> for Vec3 {
    type Output = f64;

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

impl From<(f64, f64, f64)> for Vec3 {
    #[inline]
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self(x, y, z)
    }
}

impl From<[f64; 3]> for Vec3 {
    #[inline]
    fn from([x, y, z]: [f64; 3]) -> Self {
        Self(x, y, z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sum for Vec3 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::default(), |a, b| a + b)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.0, -self.1, -self.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self {
        Self::new(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn random() -> Self {
        Self(fastrand::f64(), fastrand::f64(), fastrand::f64())
    }

    pub fn random_range(range: impl RangeBounds<f64> + Clone) -> Self {
        Self(
            fastrand_contrib::f64_range(range.clone()),
            fastrand_contrib::f64_range(range.clone()),
            fastrand_contrib::f64_range(range),
        )
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

    #[inline]
    pub fn x(&self) -> f64 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.1
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.2
    }

    #[inline]
    pub fn length_sq(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }

    #[inline]
    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    #[inline]
    pub fn dot(&self, other: Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    #[inline]
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        const T: f64 = 1e-6;
        self.0.abs() < T && self.1.abs() < T && self.2.abs() < T
    }

    #[inline]
    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * n
    }

    #[inline]
    pub fn refract(&self, n: Vec3, relative_refractive_index: f64) -> Vec3 {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = relative_refractive_index * (*self + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_sq()).abs().sqrt()) * n;

        r_out_perp + r_out_parallel
    }

    #[inline]
    fn linear_to_gamma(component: f64) -> f64 {
        component.sqrt().max(0.0)
    }

    pub fn to_rgb8(self) -> [u8; 3] {
        const START: f64 = 0.000;
        const END: f64 = 0.999;

        let r = Self::linear_to_gamma(self.0);
        let g = Self::linear_to_gamma(self.1);
        let b = Self::linear_to_gamma(self.2);

        let r_byte = (256.0 * r.clamp(START, END)) as u8;
        let g_byte = (256.0 * g.clamp(START, END)) as u8;
        let b_byte = (256.0 * b.clamp(START, END)) as u8;

        [r_byte, g_byte, b_byte]
    }
}
