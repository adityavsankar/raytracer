use core::f32;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::io::prelude::*;
use std::{fs::File, io::BufWriter};

use crate::interval::Interval;
use crate::{
    objects::{Object, ObjectList},
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Camera {
    // img settings
    aspect_ratio: f32,
    img_width: u16,
    img_height: u16,
    samples_per_pixel: u16,
    max_depth: u16,
    pixel_sample_scale: f32,
    // view settings
    center: Point3,
    vertical_fov: f32,
    look_from: Point3,
    look_at: Point3,
    view_up: Vec3,
    // orthonormal basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    // defocus blur
    defocus_angle: f32,
    focus_dist: f32,
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
        img_width: u16,
        samples_per_pixel: u16,
        max_depth: u16,
        vertical_fov: f32,
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        defocus_angle: f32,
        focus_dist: f32,
    ) -> Self {
        let img_height = 1.max((img_width as f32 / aspect_ratio) as u16);

        let pixel_sample_scale = 1.0 / samples_per_pixel as f32;

        let center = look_from;

        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (img_width as f32 / img_height as f32);

        let w = (look_from - look_at).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / img_width as f32;
        let pixel_delta_v = viewport_v / img_height as f32;

        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * -v;

        Self {
            // img settings
            aspect_ratio,
            img_width,
            img_height,
            samples_per_pixel,
            max_depth,
            pixel_sample_scale,
            // view settings
            center,
            vertical_fov,
            look_from,
            look_at,
            view_up,
            // orthonormal basis vectors
            u,
            v,
            w,
            // defocus blur
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
            // internal
            pixel_00,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(mut ray: Ray, world: &ObjectList, max_depth: u16) -> Color {
        let mut color = Color::new(1.0, 1.0, 1.0);
        for _ in 0..max_depth {
            if let Some(hit_record) = world.hit(&ray, Interval::new(0.001, f32::INFINITY)) {
                if let Some(refl) = hit_record.mat.scatter(&ray, &hit_record) {
                    color *= refl.attenuation;
                    ray = refl.scattered;
                } else {
                    return Color::default();
                }
            } else {
                let unit_dir = ray.direction().unit();
                let a = 0.5 * (unit_dir.y() + 1.0);
                return color
                    * ((1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0));
            }
        }

        Color::new(0.0, 0.0, 0.0)
    }

    fn sample_square() -> Vec3 {
        let mut t = rand::thread_rng();
        Vec3::new(t.gen_range(-0.5..0.5), t.gen_range(-0.5..0.5), 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Point3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn get_ray(&self, i: u16, j: u16) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel_00
            + (i as f32 + offset.x()) * self.pixel_delta_u
            + (j as f32 + offset.y()) * self.pixel_delta_v;
        let origin = match self.defocus_angle {
            ..=0.0 => self.center,
            _ => self.defocus_disk_sample(),
        };
        let time = thread_rng().gen();
        Ray::new_with_time(origin, pixel_sample - origin, time)
    }

    pub fn render(&self, world: &ObjectList, file_name: &str) -> std::io::Result<()> {
        let image = File::create(file_name)?;
        let est_file_size = (self.img_width as usize * self.img_height as usize + 1) * 11;
        let mut image_buf = BufWriter::with_capacity(est_file_size, image);
        image_buf.write(format!("P3\n{} {}\n255\n", self.img_width, self.img_height).as_bytes())?;

        let mut pixels = Vec::with_capacity(self.img_height as usize * self.img_width as usize);

        pixels.par_extend(
            (0..self.img_height)
                .into_par_iter()
                .map(|j| {
                    let mut row = Vec::with_capacity(self.img_width as usize);
                    for i in 0..self.img_width {
                        let pixel_color: Color = (0..self.samples_per_pixel)
                            .map(|_| {
                                let ray = self.get_ray(i, j);
                                Self::ray_color(ray, &world, self.max_depth)
                            })
                            .sum();

                        row.push(pixel_color * self.pixel_sample_scale);
                    }
                    row
                })
                .flat_map(|row| row),
        );

        for pixel in pixels {
            image_buf.write(&pixel.p3_format())?;
        }

        image_buf.flush()?;

        println!("Done");
        println!("Output: {}", file_name);
        println!("Resolution: {} x {}", self.img_width, self.img_height);
        println!("Est. File Size: {} KB", est_file_size / (1 << 10));

        Ok(())
    }
}
