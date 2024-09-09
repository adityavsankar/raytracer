use crate::{
    bvh::BVHNode,
    interval::Interval,
    objects::Object,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder};
use rayon::prelude::*;
use std::{error::Error, fs::File, io::BufWriter, time::Instant};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Camera {
    // img settings
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u16,
    max_depth: u16,
    pixel_sample_scale: f32,
    // view settings
    center: Point3,
    vertical_fov: f32,
    look_from: Point3,
    look_at: Point3,
    view_up: Vec3,
    background: Color,
    // orthonormal basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    // defocus blur
    defocus_angle: f32,
    focus_distance: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    // internal
    pixel_00: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        samples_per_pixel: u16,
        max_depth: u16,
        vertical_fov: f32,
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        background: Color,
        defocus_angle: f32,
        focus_distance: f32,
    ) -> Self {
        let image_height = 1.max((image_width as f32 / aspect_ratio) as u32);

        let pixel_sample_scale = 1.0 / samples_per_pixel as f32;

        let center = look_from;

        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let w = (look_from - look_at).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            center - (focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_distance * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * -v;

        Self {
            // img settings
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            pixel_sample_scale,
            // view settings
            center,
            vertical_fov,
            look_from,
            look_at,
            view_up,
            background,
            // orthonormal basis vectors
            u,
            v,
            w,
            // defocus blur
            defocus_angle,
            focus_distance,
            defocus_disk_u,
            defocus_disk_v,
            // internal
            pixel_00,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(&self, mut ray: Ray, world: &BVHNode, max_depth: u16) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        let mut attenuation = Color::new(1.0, 1.0, 1.0);

        for _ in 0..max_depth {
            if let Some(hit) = world.hit(&ray, Interval::new(0.001, f32::INFINITY)) {
                let emitted = hit.material.emit(hit.u, hit.v, &hit.hit_point);
                if let Some(refl) = hit.material.scatter(&ray, &hit) {
                    color = color + attenuation * emitted;
                    attenuation = attenuation * refl.attenuation;
                    ray = refl.scattered;
                } else {
                    color = color + attenuation * emitted;
                    break;
                }
            } else {
                color = color + attenuation * self.background;
                break;
            }
        }

        color
    }

    fn sample_square() -> Vec3 {
        Vec3::new(
            fastrand_contrib::f32_range(-0.5..0.5),
            fastrand_contrib::f32_range(-0.5..0.5),
            0.0,
        )
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Point3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel_00
            + (i as f32 + offset.x()) * self.pixel_delta_u
            + (j as f32 + offset.y()) * self.pixel_delta_v;
        let origin = match self.defocus_angle {
            ..=0.0 => self.center,
            _ => self.defocus_disk_sample(),
        };
        let time = fastrand::f32();
        Ray::new(origin, pixel_sample - origin, time)
    }

    pub fn render(&self, world: &BVHNode, file_name: &str) -> Result<(), Box<dyn Error>> {
        let start = Instant::now();
        let pixels: Vec<[u8; 3]> = ((0..self.image_height).into_par_iter().flat_map(|j| {
            (0..self.image_width).into_par_iter().map(move |i| {
                ((0..self.samples_per_pixel)
                    .into_par_iter()
                    .map(|_| self.ray_color(self.get_ray(i, j), &world, self.max_depth))
                    .sum::<Color>()
                    * self.pixel_sample_scale)
                    .rgb8()
            })
        }))
        .collect();
        let end = Instant::now();

        let result_path = format!("./results/{file_name}.png");
        let image_file = File::create(&result_path)?;
        let image_buf = BufWriter::new(image_file);
        let png_encoder = PngEncoder::new(image_buf);

        png_encoder.write_image(
            pixels.as_flattened(),
            self.image_width,
            self.image_height,
            ExtendedColorType::Rgb8,
        )?;

        println!("Done");
        println!("Render Time: {:.3}s", (end - start).as_secs_f64());
        println!("Output Location: {}", result_path);
        println!("Resolution: {} x {}", self.image_width, self.image_height);

        Ok(())
    }
}
