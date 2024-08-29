// use crate::{
//     bvh::BVHNode,
//     camera::Camera,
//     material::{Dielectric, Lambertian, Metal},
//     objects::ObjectList,
//     sphere::Sphere,
//     texture::{CheckerTexture, SolidColor},
//     vec3::{Color, Point3, Vec3},
// };
// use std::sync::Arc;
use std::error::Error;
use std::path::Path;

mod bvh;
mod camera;
mod interval;
mod material;
mod objects;
mod ray;
mod scene;
mod sphere;
mod texture;
mod vec3;

fn main() -> Result<(), Box<dyn Error>> {
    // let ground_material = Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(
    //     Arc::new(SolidColor::new(0.2, 0.3, 0.1)),
    //     Arc::new(SolidColor::new(0.9, 0.9, 0.9)),
    //     0.32,
    // ))));

    // world.push(Arc::new(Sphere::stationary(
    //     Point3::new(0.0, -1000.0, 0.0),
    //     1000.0,
    //     ground_material,
    // )));

    // for i in -11..11 {
    //     for j in -11..11 {
    //         let choose_mat = fastrand::f32();

    //         let center = Point3::new(
    //             i as f32 + 0.9 * fastrand::f32(),
    //             0.2,
    //             j as f32 + 0.9 * fastrand::f32(),
    //         );

    //         if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
    //             match choose_mat {
    //                 0.0..0.8 => {
    //                     let albedo = Color::random() * Color::random();
    //                     let sphere_material =
    //                         Arc::new(Lambertian::new(Arc::new(SolidColor::from(albedo))));
    //                     // let center2 = center + Vec3::new(0.0, t.gen_range(0.0..0.5), 0.0);
    //                     world.push(Arc::new(Sphere::stationary(
    //                         center,
    //                         // center2,
    //                         0.2,
    //                         sphere_material,
    //                     )));
    //                 }
    //                 0.8..0.95 => {
    //                     let albedo = Color::random_range(0.5..1.0);
    //                     let fuzz = fastrand::f32() * 0.5;
    //                     let sphere_material = Arc::new(Metal::new(albedo, fuzz));
    //                     world.push(Arc::new(Sphere::stationary(center, 0.2, sphere_material)));
    //                 }
    //                 _ => {
    //                     let sphere_material = Arc::new(Dielectric::new(1.5));
    //                     world.push(Arc::new(Sphere::stationary(center, 0.2, sphere_material)));
    //                 }
    //             };
    //         }
    //     }
    // }

    // let material1 = Arc::new(Dielectric::new(1.5));
    // let material2 = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.4, 0.2, 0.1))));
    // let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    // world.push(Arc::new(Sphere::stationary(
    //     Point3::new(0.0, 1.0, 0.0),
    //     1.0,
    //     material1,
    // )));
    // world.push(Arc::new(Sphere::stationary(
    //     Point3::new(-4.0, 1.0, 0.0),
    //     1.0,
    //     material2,
    // )));
    // world.push(Arc::new(Sphere::stationary(
    //     Point3::new(4.0, 1.0, 0.0),
    //     1.0,
    //     material3,
    // )));

    let file_path = std::env::args().nth(1).expect("No file provided");
    let file_name = Path::new(&file_path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| format!("{}.ppm", stem))
        .unwrap();

    let (world, camera) = scene::scene("./scenes/two_checker_spheres.toml")?;
    camera.render(&world, &file_name)?;

    Ok(())
}
