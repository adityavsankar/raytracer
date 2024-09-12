#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]

use std::{env, error::Error};

mod aabb;
mod bvh;
mod camera;
mod cuboid;
mod interval;
mod material;
mod objects;
mod quad;
mod ray;
mod scene;
mod sphere;
mod texture;
mod vec3;

fn main() -> Result<(), Box<dyn Error>> {
    let scene_path = env::args()
        .nth(1)
        .expect("Provide the path to the scene configuration file as an argument");

    let (world, camera, scene_name) = scene::create(&scene_path)?;
    camera.render(&world, scene_name)?;

    Ok(())
}
