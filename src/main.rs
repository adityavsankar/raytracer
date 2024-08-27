use crate::{
    bvh::BVHNode,
    camera::Camera,
    material::{Dielectric, Lambertian, Metal},
    objects::ObjectList,
    sphere::Sphere,
    vec3::{Color, Point3, Vec3},
};
use std::sync::Arc;

mod bvh;
mod camera;
mod interval;
mod material;
mod objects;
mod ray;
mod sphere;
mod vec3;

fn main() -> std::io::Result<()> {
    fastrand::seed(3);

    let mut world = ObjectList::new();

    let ground_material = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.push(Arc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = fastrand::f32();

            let center = Point3::new(
                i as f32 + 0.9 * fastrand::f32(),
                0.2,
                j as f32 + 0.9 * fastrand::f32(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    0.0..0.8 => {
                        // diffuse
                        let albedo = Color::random() * Color::random();
                        let sphere_material = Box::new(Lambertian::new(albedo));
                        // let center2 = center + Vec3::new(0.0, t.gen_range(0.0..0.5), 0.0);
                        world.push(Arc::new(Sphere::stationary(
                            center,
                            // center2,
                            0.2,
                            sphere_material,
                        )));
                    }
                    0.8..0.95 => {
                        // metal
                        let albedo = Color::random_range(0.5..1.0);
                        let fuzz = fastrand::f32() * 0.5;
                        let sphere_material = Box::new(Metal::new(albedo, fuzz));
                        world.push(Arc::new(Sphere::stationary(center, 0.2, sphere_material)));
                    }
                    _ => {
                        // glass
                        let sphere_material = Box::new(Dielectric::new(1.5));
                        world.push(Arc::new(Sphere::stationary(center, 0.2, sphere_material)));
                    }
                };
            }
        }
    }

    let material1 = Box::new(Dielectric::new(1.5));
    let material2 = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.push(Arc::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    world.push(Arc::new(Sphere::stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    world.push(Arc::new(Sphere::stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut world1 = ObjectList::new();
    world1.push(Arc::new(BVHNode::new(&mut world.objects)));

    let camera = Camera::new(
        16.0 / 9.0,
        800,
        100,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );

    camera.render(&world1, "image1.ppm")?;

    Ok(())
}
