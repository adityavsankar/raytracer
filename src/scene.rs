use crate::{
    bvh::BVHNode,
    camera::Camera,
    constant_medium::ConstantMedium,
    cuboid::Cuboid,
    entity::{Entity, EntityCluster},
    instance::{Rotated, Translated},
    material::{Dielectric, DiffuseLight, Isotropic, Lambertian, Material, Metal},
    quad::Quad,
    sphere::Sphere,
    texture::{Checker, Image, Solid, Texture},
    vec3::{Color, Point3, Vec3},
};
use serde::Deserialize;
use std::{convert::Into, error::Error, fs, path::Path, sync::Arc};

#[derive(Debug, Deserialize)]
struct Config {
    entity: Vec<EntityConfig>,
    camera: CameraConfig,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "variant")]
enum EntityVariant {
    Sphere(SphereConfig),
    MovingSphere(MovingSphereConfig),
    Quad(QuadConfig),
    Cuboid(CuboidConfig),
    ConstantMedium(Box<ConstantMediumConfig>),
    EntityCluster(EntityClusterConfig),
}

#[derive(Debug, Deserialize)]
struct EntityConfig {
    #[serde(flatten)]
    variant: EntityVariant,
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
struct MovingSphereConfig {
    center1: [f64; 3],
    center2: [f64; 3],
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
struct ConstantMediumConfig {
    boundary: EntityConfig,
    density: f64,
}

#[derive(Debug, Deserialize)]
struct EntityClusterConfig {
    children: Vec<EntityConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "variant")]
enum MaterialVariant {
    Lambertian(LambertianConfig),
    Metal(MetalConfig),
    Dielectric(DielectricConfig),
    DiffuseLight(DiffuseLightConfig),
    Isotropic(IsotropicConfig),
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
struct IsotropicConfig {
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
    Checker(Box<CheckerConfig>),
    Image(ImageConfig),
}

#[derive(Debug, Deserialize)]
struct SolidColorConfig {
    color: [f64; 3],
}

#[derive(Debug, Deserialize)]
struct CheckerConfig {
    odd: TextureConfig,
    even: TextureConfig,
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
                Arc::new(Solid::from(Color::from(solid_color.color)))
            }
            TextureVariant::Checker(checker) => Arc::new(Checker::new(
                checker.odd.into(),
                checker.even.into(),
                checker.scale,
            )),
            TextureVariant::Image(image) => Arc::new(Image::new(&image.image_path)),
        }
    }
}

impl From<MaterialConfig> for Arc<dyn Material> {
    fn from(value: MaterialConfig) -> Self {
        match value.material {
            MaterialVariant::Lambertian(lambertian) => {
                Arc::new(Lambertian::new(lambertian.texture.into()))
            }
            MaterialVariant::Metal(metal) => {
                Arc::new(Metal::new(Color::from(metal.albedo), metal.fuzz))
            }
            MaterialVariant::Dielectric(dielectric) => {
                Arc::new(Dielectric::new(dielectric.refractive_index))
            }
            MaterialVariant::DiffuseLight(diffuse_light) => {
                Arc::new(DiffuseLight::new(diffuse_light.texture.into()))
            }
            MaterialVariant::Isotropic(isotropic) => {
                Arc::new(Isotropic::new(isotropic.texture.into()))
            }
        }
    }
}

impl From<EntityConfig> for Arc<dyn Entity> {
    fn from(config: EntityConfig) -> Self {
        let material = config.material.into();
        let mut entity: Arc<dyn Entity> = match config.variant {
            EntityVariant::Sphere(sphere) => Arc::new(Sphere::stationary(
                Point3::from(sphere.center),
                sphere.radius,
                material,
            )),
            EntityVariant::MovingSphere(moving_sphere) => Arc::new(Sphere::moving(
                Point3::from(moving_sphere.center1),
                Point3::from(moving_sphere.center2),
                moving_sphere.radius,
                material,
            )),
            EntityVariant::Quad(quad) => Arc::new(Quad::new(
                Point3::from(quad.q),
                Vec3::from(quad.u),
                Vec3::from(quad.v),
                material,
            )),
            EntityVariant::Cuboid(cuboid) => Arc::new(Cuboid::new(
                Point3::from(cuboid.a),
                Point3::from(cuboid.b),
                material,
            )),
            EntityVariant::ConstantMedium(constant_medium) => Arc::new(ConstantMedium::new(
                constant_medium.boundary.into(),
                constant_medium.density,
                material,
            )),
            EntityVariant::EntityCluster(entity_cluster) => {
                let mut bruh = EntityCluster::new();
                for entity in entity_cluster.children {
                    bruh.push(entity.into());
                }
                Arc::new(bruh)
            }
        };

        if let Some(rotation) = config.rotation {
            entity = Arc::new(Rotated::new(entity, Vec3::from(rotation)));
        }

        if let Some(translation) = config.translation {
            entity = Arc::new(Translated::new(entity, Vec3::from(translation)));
        }

        entity
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
    let mut entities: Vec<Arc<dyn Entity>> = scene.entity.into_iter().map(Into::into).collect();
    let camera = scene.camera.into();
    let world = BVHNode::new(&mut entities);
    let name = Path::new(scene_path).file_stem().unwrap().to_str().unwrap();

    Ok((world, camera, name))
}
