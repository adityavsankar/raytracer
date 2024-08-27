use bvh::BVHNode;
use camera::Camera;
use material::{Dielectric, Lambertian, Metal};
use objects::ObjectList;
use rand::{thread_rng, Rng};
use sphere::Sphere;
use std::sync::Arc;
use vec3::{Color, Point3, Vec3};

mod bvh;
mod camera;
mod interval;
mod material;
mod objects;
mod ray;
mod sphere;
mod vec3;

fn main() -> std::io::Result<()> {
    let mut t = thread_rng();

    let mut world = ObjectList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.push(Arc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat: f32 = t.gen();

            let center = Point3::new(
                i as f32 + 0.9 * t.gen::<f32>(),
                0.2,
                j as f32 + 0.9 * t.gen::<f32>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    0.0..0.8 => {
                        // diffuse
                        let albedo = Color::random() * Color::random();
                        let sphere_material = Arc::new(Lambertian::new(albedo));
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
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = t.gen_range(0.0..0.5);
                        let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                        world.push(Arc::new(Sphere::stationary(center, 0.2, sphere_material)));
                    }
                    _ => {
                        // glass
                        let sphere_material = Arc::new(Dielectric::new(1.5));
                        world.push(Arc::new(Sphere::stationary(center, 0.2, sphere_material)));
                    }
                };
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

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
        16.0 / 10.0,
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

    camera.render(&world1, "image.ppm")?;

    Ok(())
}
