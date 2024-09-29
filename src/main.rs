#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]

use std::error::Error;

mod aabb;
mod bvh;
mod camera;
mod constant_medium;
mod cuboid;
mod entity;
mod instance;
mod interval;
mod mat3;
mod material;
mod perlin;
mod quad;
mod ray;
mod scene;
mod sphere;
mod texture;
mod vec3;

fn main() -> Result<(), Box<dyn Error>> {
    let scene_path = std::env::args()
        .nth(1)
        .ok_or("Provide a path to the scene configuration as an argument")?;

    match scene::create(&scene_path) {
        Ok((world, camera, scene_name)) => camera.render(&world, &scene_name)?,
        Err(e) => eprintln!("{e}"),
    }

    Ok(())
}
