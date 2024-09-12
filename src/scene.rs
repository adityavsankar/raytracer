use crate::{
    bvh::BVHNode,
    camera::Camera,
    cuboid::Cuboid,
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    objects::Object,
    quad::Quad,
    sphere::Sphere,
    texture::{Checker, Image, Solid, Texture},
    vec3::{Color, Point3, Vec3},
};
use serde::Deserialize;
use std::{convert::Into, error::Error, fs, path::Path, sync::Arc};

#[derive(Debug, Deserialize)]
struct Config {
    object: Vec<ObjectConfig>,
    camera: CameraConfig,
}

#[derive(Debug, Deserialize)]
struct ObjectConfig {
    variant: String,
    center: Option<[f64; 3]>,
    radius: Option<f64>,
    q: Option<[f64; 3]>,
    u: Option<[f64; 3]>,
    v: Option<[f64; 3]>,
    a: Option<[f64; 3]>,
    b: Option<[f64; 3]>,
    material: MaterialConfig,
}

#[derive(Debug, Deserialize)]
struct MaterialConfig {
    variant: String,
    texture: Option<TextureConfig>,
    albedo: Option<[f64; 3]>,
    fuzz: Option<f64>,
    refractive_index: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct TextureConfig {
    variant: String,
    color: Option<[f64; 3]>,
    color1: Option<[f64; 3]>,
    color2: Option<[f64; 3]>,
    scale: Option<f64>,
    image: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CameraConfig {
    aspect_ratio: f64,
    image_width: u32,
    samples_per_pixel: u16,
    max_depth: u16,
    look_from: [f64; 3],
    look_at: [f64; 3],
    view_up: [f64; 3],
    background: [f64; 3],
    vertical_fov: f64,
    defocus_angle: f64,
    focus_distance: f64,
}

impl From<TextureConfig> for Arc<dyn Texture> {
    fn from(value: TextureConfig) -> Self {
        match value.variant.as_str() {
            "solid_color" => {
                let color = Color::from(value.color.unwrap());
                Arc::new(Solid::from(color))
            }
            "checker" => {
                let color1 = Color::from(value.color1.unwrap());
                let color2 = Color::from(value.color2.unwrap());
                let scale = value.scale.unwrap();
                Arc::new(Checker::new(
                    Arc::new(Solid::from(color1)),
                    Arc::new(Solid::from(color2)),
                    scale,
                ))
            }
            "image" => {
                let image_path = value.image.unwrap();
                Arc::new(Image::new(&image_path))
            }
            _ => panic!("Unknown texture variant"),
        }
    }
}

impl From<MaterialConfig> for Arc<dyn Material> {
    fn from(value: MaterialConfig) -> Self {
        match value.variant.as_str() {
            "lambertian" | "diffuse_light" => {
                let texture = value.texture.unwrap().into();
                match value.variant.as_str() {
                    "lambertian" => Arc::new(Lambertian::new(texture)),
                    "diffuse_light" => Arc::new(DiffuseLight::new(texture)),
                    _ => unreachable!(),
                }
            }
            "metal" => {
                let albedo = Color::from(value.albedo.unwrap());
                let fuzz = value.fuzz.unwrap();
                Arc::new(Metal::new(albedo, fuzz))
            }
            "dielectric" => {
                let refractive_index = value.refractive_index.unwrap();
                Arc::new(Dielectric::new(refractive_index))
            }
            _ => panic!("Unknown material variant"),
        }
    }
}

impl From<ObjectConfig> for Arc<dyn Object> {
    fn from(value: ObjectConfig) -> Self {
        let material = value.material.into();
        match value.variant.as_str() {
            "sphere" => {
                let center = Point3::from(value.center.unwrap());
                Arc::new(Sphere::stationary(center, value.radius.unwrap(), material))
            }
            "quad" => {
                let q = Point3::from(value.q.unwrap());
                let u = Vec3::from(value.u.unwrap());
                let v = Vec3::from(value.v.unwrap());
                Arc::new(Quad::new(q, u, v, material))
            }
            "cuboid" => {
                let a = Point3::from(value.a.unwrap());
                let b = Point3::from(value.b.unwrap());
                Arc::new(Cuboid::new(a, b, material))
            }
            _ => panic!("Unknown object variant"),
        }
    }
}

impl From<CameraConfig> for Camera {
    fn from(value: CameraConfig) -> Self {
        Camera::new(
            value.aspect_ratio,
            value.image_width,
            value.samples_per_pixel,
            value.max_depth,
            value.vertical_fov,
            value.look_from.into(),
            value.look_at.into(),
            value.view_up.into(),
            value.background.into(),
            value.defocus_angle,
            value.focus_distance,
        )
    }
}

pub fn create(scene_path: &str) -> Result<(BVHNode, Camera, &str), Box<dyn Error>> {
    let scene: Config = toml::from_str(&fs::read_to_string(scene_path)?)?;
    let mut objects: Vec<Arc<dyn Object>> = scene.object.into_iter().map(Into::into).collect();
    let camera = scene.camera.into();
    let world = BVHNode::new(&mut objects);
    let name = Path::new(scene_path).file_stem().unwrap().to_str().unwrap();

    Ok((world, camera, name))
}
