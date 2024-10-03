use crate::vec3::Vec3;
use std::ops::{Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    elements: [[f64; 3]; 3],
}

impl Mat3 {
    pub fn new(elements: [[f64; 3]; 3]) -> Self {
        Self { elements }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn transpose(&self) -> Self {
        let mut result = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                result[i][j] = self.elements[j][i];
            }
        }
        Self::new(result)
    }

    pub fn rotation_x(angle: f64) -> Self {
        let angle = angle.to_radians();
        let (sin, cos) = angle.sin_cos();
        Self::new([[1.0, 0.0, 0.0], [0.0, cos, -sin], [0.0, sin, cos]])
    }

    pub fn rotation_y(angle: f64) -> Self {
        let angle = angle.to_radians();
        let (sin, cos) = angle.sin_cos();
        Self::new([[cos, 0.0, sin], [0.0, 1.0, 0.0], [-sin, 0.0, cos]])
    }

    pub fn rotation_z(angle: f64) -> Self {
        let angle = angle.to_radians();
        let (sin, cos) = angle.sin_cos();
        Self::new([[cos, -sin, 0.0], [sin, cos, 0.0], [0.0, 0.0, 1.0]])
    }
}

impl Mul for Mat3 {
    type Output = Self;

    #[allow(clippy::needless_range_loop)]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result[i][j] += self.elements[i][k] * rhs.elements[k][j];
                }
            }
        }
        Self::new(result)
    }
}

impl MulAssign for Mat3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.elements[0][0] * rhs.x()
                + self.elements[0][1] * rhs.y()
                + self.elements[0][2] * rhs.z(),
            self.elements[1][0] * rhs.x()
                + self.elements[1][1] * rhs.y()
                + self.elements[1][2] * rhs.z(),
            self.elements[2][0] * rhs.x()
                + self.elements[2][1] * rhs.y()
                + self.elements[2][2] * rhs.z(),
        )
    }
}
