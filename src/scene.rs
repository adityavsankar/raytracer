use crate::{
    bvh::BVHNode,
    camera::Camera,
    cuboid::Cuboid,
    instance::{Rotated, Translated},
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
#[serde(tag = "variant")]
enum ObjectVariant {
    Sphere(SphereConfig),
    Quad(QuadConfig),
    Cuboid(CuboidConfig),
}

#[derive(Debug, Deserialize)]
struct ObjectConfig {
    #[serde(flatten)]
    object: ObjectVariant,
    material: MaterialConfig,
    translation: Option<[f64; 3]>,
    rotation: Option<[f64; 3]>,
}

#[derive(Debug, Deserialize)]
struct SphereConfig {
    center: [f64; 3],
    radius: f64,
}

#[derive(Debug, Deserialize)]
struct QuadConfig {
    q: [f64; 3],
    u: [f64; 3],
    v: [f64; 3],
}

#[derive(Debug, Deserialize)]
struct CuboidConfig {
    a: [f64; 3],
    b: [f64; 3],
}

#[derive(Debug, Deserialize)]
#[serde(tag = "variant")]
enum MaterialVariant {
    Lambertian(LambertianConfig),
    Metal(MetalConfig),
    Dielectric(DielectricConfig),
    DiffuseLight(DiffuseLightConfig),
}

#[derive(Debug, Deserialize)]
struct LambertianConfig {
    texture: TextureConfig,
}

#[derive(Debug, Deserialize)]
struct MetalConfig {
    albedo: [f64; 3],
    fuzz: f64,
}

#[derive(Debug, Deserialize)]
struct DielectricConfig {
    refractive_index: f64,
}

#[derive(Debug, Deserialize)]
struct DiffuseLightConfig {
    texture: TextureConfig,
}

#[derive(Debug, Deserialize)]
struct MaterialConfig {
    #[serde(flatten)]
    material: MaterialVariant,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "variant")]
enum TextureVariant {
    SolidColor(SolidColorConfig),
    Checker(CheckerConfig),
    Image(ImageConfig),
}

#[derive(Debug, Deserialize)]
struct SolidColorConfig {
    color: [f64; 3],
}

#[derive(Debug, Deserialize)]
struct CheckerConfig {
    color1: [f64; 3],
    color2: [f64; 3],
    scale: f64,
}

#[derive(Debug, Deserialize)]
struct ImageConfig {
    image_path: String,
}

#[derive(Debug, Deserialize)]
struct TextureConfig {
    #[serde(flatten)]
    variant: TextureVariant,
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
        match value.variant {
            TextureVariant::SolidColor(solid_color) => {
                let color = Color::from(solid_color.color);
                Arc::new(Solid::from(color))
            }
            TextureVariant::Checker(checker) => {
                let color1 = Color::from(checker.color1);
                let color2 = Color::from(checker.color2);
                Arc::new(Checker::new(
                    Solid::from(color1),
                    Solid::from(color2),
                    checker.scale,
                ))
            }
            TextureVariant::Image(image) => {
                let image = Image::new(&image.image_path);
                Arc::new(image)
            }
        }
    }
}

impl From<MaterialConfig> for Arc<dyn Material> {
    fn from(value: MaterialConfig) -> Self {
        match value.material {
            MaterialVariant::Lambertian(lambertian) => {
                let texture = lambertian.texture.into();
                Arc::new(Lambertian::new(texture))
            }
            MaterialVariant::Metal(metal) => {
                let albedo = Color::from(metal.albedo);
                Arc::new(Metal::new(albedo, metal.fuzz))
            }
            MaterialVariant::Dielectric(dielectric) => {
                Arc::new(Dielectric::new(dielectric.refractive_index))
            }
            MaterialVariant::DiffuseLight(diffuse_light) => {
                let texture = diffuse_light.texture.into();
                Arc::new(DiffuseLight::new(texture))
            }
        }
    }
}

impl From<ObjectConfig> for Arc<dyn Object> {
    fn from(config: ObjectConfig) -> Self {
        let material = config.material.into();
        let mut object: Arc<dyn Object> = match config.object {
            ObjectVariant::Sphere(sphere) => Arc::new(Sphere::stationary(
                Point3::from(sphere.center),
                sphere.radius,
                material,
            )),
            ObjectVariant::Quad(quad) => Arc::new(Quad::new(
                Point3::from(quad.q),
                Vec3::from(quad.u),
                Vec3::from(quad.v),
                material,
            )),
            ObjectVariant::Cuboid(cuboid) => Arc::new(Cuboid::new(
                Point3::from(cuboid.a),
                Point3::from(cuboid.b),
                material,
            )),
        };

        if let Some(rotation) = config.rotation {
            object = Arc::new(Rotated::new(object, Vec3::from(rotation)));
        }

        if let Some(translation) = config.translation {
            object = Arc::new(Translated::new(object, Vec3::from(translation)));
        }

        object
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
            Vec3::from(value.look_from),
            Vec3::from(value.look_at),
            Vec3::from(value.view_up),
            Color::from(value.background),
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
