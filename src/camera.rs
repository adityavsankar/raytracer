use crate::{
    bvh::BVHNode,
    interval::Interval,
    objects::Object,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
    time::Instant,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Camera {
    // img settings
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u16,
    max_depth: u16,
    pixel_sample_scale: f64,
    // view settings
    center: Point3,
    vertical_fov: f64,
    look_from: Point3,
    look_at: Point3,
    view_up: Vec3,
    background: Color,
    // orthonormal basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    // defocus blur
    defocus_angle: f64,
    focus_distance: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    // internal
    pixel_00: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u16,
        max_depth: u16,
        vertical_fov: f64,
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        background: Color,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Self {
        let image_height = 1.max((image_width as f64 / aspect_ratio).round() as u32);

        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;

        let center = look_from;

        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (look_from - look_at).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

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

    fn ray_color(&self, ray: &Ray, world: &BVHNode, depth: u16) -> Color {
        if depth == 0 {
            return Color::default();
        }

        if let Some(hit_record) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            let emitted_color =
                hit_record
                    .material
                    .emit(hit_record.u, hit_record.v, &hit_record.hit_point);
            if let Some(reflected) = hit_record.material.scatter(ray, &hit_record) {
                let scattered_color =
                    reflected.attenuation * self.ray_color(&reflected.scattered, world, depth - 1);
                emitted_color + scattered_color
            } else {
                emitted_color
            }
        } else {
            self.background
        }
    }

    fn sample_square() -> Vec3 {
        Vec3::new(
            fastrand_contrib::f64_range(-0.5..0.5),
            fastrand_contrib::f64_range(-0.5..0.5),
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
            + (i as f64 + offset.x()) * self.pixel_delta_u
            + (j as f64 + offset.y()) * self.pixel_delta_v;
        let origin = match self.defocus_angle {
            ..=0.0 => self.center,
            _ => self.defocus_disk_sample(),
        };
        let time = fastrand::f64();
        Ray::new(origin, pixel_sample - origin, time)
    }

    const OUTPUT_DIR: &'static str = "./results";

    pub fn render(&self, world: &BVHNode, scene_name: &str) -> Result<(), Box<dyn Error>> {
        let start = Instant::now();
        let pixels = self.render_image(world);
        let end = Instant::now();
        let result_path = self.save_image(pixels, scene_name)?;

        println!("Finished");
        println!("Render Time: {:.3}s", (end - start).as_secs_f64());
        println!("Output Location: {result_path}");
        println!("Resolution: {} x {}", self.image_width, self.image_height);

        Ok(())
    }

    fn render_image(&self, world: &BVHNode) -> Vec<Color> {
        let progress_bar = ProgressBar::new(self.image_height as u64);
        let progress_style = ProgressStyle::default_bar()
            .template("Render Progress: [{bar:40.green}] {percent_precise}%\nElapsed: {elapsed} | Remaining: {eta}").unwrap()
            .progress_chars("=> ");
        progress_bar.set_style(progress_style);

        // flattens the rows of pixels back to one dimension and collects to a Vec
        (0..self.image_height)
            .into_par_iter()
            .progress_with(progress_bar)
            .flat_map(|j| {
                // this iterator returns one row of pixels (scanline)
                (0..self.image_width).into_par_iter().map(move |i| {
                    // this iterator returns one pixel by averaging samples
                    (0..self.samples_per_pixel)
                        .into_par_iter()
                        .map(|_| self.ray_color(&self.get_ray(i, j), world, self.max_depth))
                        .sum::<Color>()
                        * self.pixel_sample_scale
                })
            })
            .collect()
    }

    fn save_image(&self, pixels: Vec<Color>, name: &str) -> Result<String, Box<dyn Error>> {
        if !Path::new(Self::OUTPUT_DIR).exists() {
            create_dir_all(Self::OUTPUT_DIR)?;
        }

        let result_path = format!("{}/{}.png", Self::OUTPUT_DIR, name);
        let image_file = File::create(&result_path)?;
        let image_buf = BufWriter::new(image_file);
        let png_encoder = PngEncoder::new(image_buf);
        let raw: Vec<u8> = pixels.into_iter().flat_map(Vec3::to_rgb8).collect();

        png_encoder.write_image(
            &raw,
            self.image_width,
            self.image_height,
            ExtendedColorType::Rgb8,
        )?;

        Ok(result_path)
    }
}
