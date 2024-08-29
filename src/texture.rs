use crate::vec3::{Color, Point3};
use std::sync::Arc;

pub trait Texture: Send + Sync + std::fmt::Debug {
    fn color_value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

#[derive(Debug, Clone, Default)]
pub struct SolidColor {
    color: Color,
}

impl Texture for SolidColor {
    fn color_value(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        self.color
    }
}

impl From<Color> for SolidColor {
    fn from(color: Color) -> Self {
        Self::new(color.x(), color.y(), color.z())
    }
}

impl SolidColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            color: Color::new(r, g, b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
    inv_scale: f32,
}

impl Texture for CheckerTexture {
    fn color_value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let x_int = (self.inv_scale * p.x()).floor() as i32;
        let y_int = (self.inv_scale * p.y()).floor() as i32;
        let z_int = (self.inv_scale * p.z()).floor() as i32;
        if (x_int + y_int + z_int) & 1 == 0 {
            self.even.color_value(u, v, p)
        } else {
            self.odd.color_value(u, v, p)
        }
    }
}

impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>, scale: f32) -> Self {
        Self {
            odd,
            even,
            inv_scale: 1.0 / scale,
        }
    }
}
