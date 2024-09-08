use std::{error::Error, path::Path};

mod bvh;
mod camera;
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
    let file_path = std::env::args().nth(1).expect("No file provided");
    let file_name = Path::new(&file_path).file_stem().unwrap().to_str().unwrap();

    let (world, camera) = scene::scene(&file_path)?;
    camera.render(&world, &file_name)?;

    Ok(())
}
