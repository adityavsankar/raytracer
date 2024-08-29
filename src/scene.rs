use crate::bvh::BVHNode;
use crate::material::Material;
use crate::{
    camera::Camera, material::*, objects::ObjectList, sphere::Sphere, texture::*, vec3::*,
};
use serde::Deserialize;
use std::sync::Arc;
use std::{error::Error, fs};

#[derive(Debug, Deserialize)]
struct Config {
    objects: Vec<ObjectConfig>,
    camera: CameraConfig,
}

#[derive(Debug, Deserialize)]
struct ObjectConfig {
    center: [f32; 3],
    radius: f32,
    material: MaterialConfig,
}

#[derive(Debug, Deserialize)]
struct MaterialConfig {
    variant: String,
    texture: Option<TextureConfig>,
    albedo: Option<[f32; 3]>,
    fuzz: Option<f32>,
    ref_idx: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct TextureConfig {
    variant: String,
    color1: Option<[f32; 3]>,
    color2: Option<[f32; 3]>,
    scale: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct CameraConfig {
    aspect_ratio: f32,
    image_width: u16,
    samples_per_pixel: u16,
    max_depth: u16,
    look_from: [f32; 3],
    look_at: [f32; 3],
    view_up: [f32; 3],
    vertical_fov: f32,
    defocus_angle: f32,
    focus_distance: f32,
}

pub fn scene(filename: &str) -> Result<(ObjectList, Camera), Box<dyn Error>> {
    let config: Config = toml::from_str(&fs::read_to_string(filename)?)?;
    let mut world = ObjectList::new();

    for obj in config.objects {
        let center = Point3::new(obj.center[0], obj.center[1], obj.center[2]);
        let material: Arc<dyn Material> = match obj.material.variant.as_str() {
            "lambertian" => {
                let t = obj.material.texture.unwrap();
                let texture: Arc<dyn Texture> = match t.variant.as_str() {
                    "solid_color" => {
                        let color = t.color1.unwrap();
                        Arc::new(SolidColor::new(color[0], color[1], color[2]))
                    }
                    "checker" => {
                        let color1 = t.color1.unwrap();
                        let color2 = t.color2.unwrap();
                        let scale = t.scale.unwrap();
                        Arc::new(CheckerTexture::new(
                            Arc::new(SolidColor::new(color1[0], color1[1], color1[2])),
                            Arc::new(SolidColor::new(color2[0], color2[1], color2[2])),
                            scale,
                        ))
                    }
                    _ => panic!("Unknown texture variant"),
                };
                Arc::new(Lambertian::new(texture))
            }
            "metal" => {
                let albedo: Color = obj.material.albedo.unwrap().into();
                let fuzz = obj.material.fuzz.unwrap();
                Arc::new(Metal::new(albedo, fuzz))
            }
            "dielectric" => {
                let ref_idx = obj.material.ref_idx.unwrap();
                Arc::new(Dielectric::new(ref_idx))
            }
            _ => panic!("Unknown material variant"),
        };
        world.push(Arc::new(Sphere::stationary(center, obj.radius, material)));
    }

    let camera = Camera::new(
        config.camera.aspect_ratio,
        config.camera.image_width,
        config.camera.samples_per_pixel,
        config.camera.max_depth,
        config.camera.vertical_fov,
        config.camera.look_from.into(),
        config.camera.look_at.into(),
        config.camera.view_up.into(),
        config.camera.defocus_angle,
        config.camera.focus_distance,
    );

    let mut world1 = ObjectList::new();
    world1.push(Arc::new(BVHNode::new(&mut world.objects)));

    Ok((world1, camera))
}
