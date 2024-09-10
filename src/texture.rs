use crate::vec3::{Color, Point3};
use std::sync::Arc;

pub trait Texture: Send + Sync + std::fmt::Debug {
    fn color_value(&self, u: f64, v: f64, hit_point: &Point3) -> Color;
}

#[derive(Debug, Clone, Default)]
pub struct SolidColor {
    color: Color,
}

impl Texture for SolidColor {
    fn color_value(&self, _u: f64, _v: f64, _hit_point: &Point3) -> Color {
        self.color
    }
}

impl From<Color> for SolidColor {
    fn from(color: Color) -> Self {
        Self::new(color.x(), color.y(), color.z())
    }
}

impl From<[f64; 3]> for SolidColor {
    fn from(color: [f64; 3]) -> Self {
        Self::new(color[0], color[1], color[2])
    }
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: Color::new(r, g, b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
    inv_scale: f64,
}

impl Texture for CheckerTexture {
    fn color_value(&self, u: f64, v: f64, hit_point: &Point3) -> Color {
        let x_int = (self.inv_scale * hit_point.x()).floor() as i32;
        let y_int = (self.inv_scale * hit_point.y()).floor() as i32;
        let z_int = (self.inv_scale * hit_point.z()).floor() as i32;
        if (x_int + y_int + z_int) & 1 == 0 {
            self.even.color_value(u, v, hit_point)
        } else {
            self.odd.color_value(u, v, hit_point)
        }
    }
}

impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>, scale: f64) -> Self {
        Self {
            odd,
            even,
            inv_scale: 1.0 / scale,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageTexture {
    data: Vec<u8>,
    width: u16,
    height: u16,
    bytes_per_pixel: u16,
}

impl Texture for ImageTexture {
    fn color_value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        let i = (u * self.width as f64) as usize;
        let j = ((1.0 - v) * self.height as f64) as usize;
        self.get_pixel(i, j)
    }
}

impl ImageTexture {
    pub fn new(image_file: &str) -> Self {
        let img = image::open(image_file)
            .expect("Failed to open image")
            .to_rgb8();
        let (width, height) = img.dimensions();
        let data = img.into_raw();
        let bytes_per_pixel = 3;
        Self {
            data,
            width: width as u16,
            height: height as u16,
            bytes_per_pixel,
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> Color {
        let index = x * self.bytes_per_pixel as usize
            + y * self.width as usize * self.bytes_per_pixel as usize;
        let pixel = &self.data[index..index + 3];
        Color::new(
            pixel[0] as f64 / 255.0,
            pixel[1] as f64 / 255.0,
            pixel[2] as f64 / 255.0,
        )
    }
}
