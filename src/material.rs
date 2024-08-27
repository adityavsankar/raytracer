use crate::{
    objects::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Debug, Clone)]
pub struct Reflected {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(&self, incoming: &Ray, hit_record: &HitRecord) -> Option<Reflected>;
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, incoming: &Ray, hit_record: &HitRecord) -> Option<Reflected> {
        let scatter_dir = {
            let t = hit_record.normal + Vec3::random_unit_vector();
            if t.near_zero() {
                hit_record.normal
            } else {
                t
            }
        };
        Some(Reflected {
            attenuation: self.albedo,
            scattered: Ray::new_with_time(hit_record.hit_point, scatter_dir, incoming.time()),
        })
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, incoming: &Ray, hit_record: &HitRecord) -> Option<Reflected> {
        let reflected = incoming.direction().reflect(hit_record.normal).unit()
            + self.fuzz * Vec3::random_unit_vector();
        let scattered = Ray::new_with_time(hit_record.hit_point, reflected, incoming.time());

        if scattered.direction().dot(hit_record.normal) > 0.0 {
            Some(Reflected {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Material for Dielectric {
    fn scatter(&self, incoming: &Ray, hit_record: &HitRecord) -> Option<Reflected> {
        let ri = if hit_record.front {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = incoming.direction().unit();
        let cos_theta = (-unit_dir).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction = if ri * sin_theta > 1.0 || self.reflectance(cos_theta) > rand::random() {
            unit_dir.reflect(hit_record.normal)
        } else {
            unit_dir.refract(hit_record.normal, ri)
        };

        Some(Reflected {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered: Ray::new_with_time(hit_record.hit_point, direction, incoming.time()),
        })
    }
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    fn reflectance(&self, cosine: f32) -> f32 {
        let r0 = ((1.0 - self.refraction_index) / (1.0 + self.refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
