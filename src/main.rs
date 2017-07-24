/*
 * [chapter 7 ]
 *
 * Diffuse Materials - render realistic looking spheres!
 *
 * output: diffuse.png
 */

mod model;
use model::vector::Vec3 as Vec3;
use model::sphere::Sphere as Sphere;
use model::ray::Ray as Ray;
use model::hlist::HitableList as HitableList;
use model::hitable::Hitable as Hitable;
use model::camera::Camera as Camera;
use model::material::Material as Material;
use model::material::Lambertian as Lambertian;
use model::material::Metal as Metal;
use std::f32;

extern crate rand;
use rand::Rng;

/*
 * TODO: document this
 */
fn color(ray: &Ray, world: &Hitable, depth: u32) -> Vec3 {
    match world.hit(ray, 0.001, 10000.0) {
        Some(rec) => match (depth, rec.material.scatter(ray, &rec)) {
            (d, Some((att, sc))) if d < 50 => att * color(&sc, world, depth + 1),
            _ => Vec3::zero()
        },
        None => {
            let blue = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
            let white = Vec3::from_one(1.0);
            let unit_direction = ray.direction().unit();
            let t = 0.5*(unit_direction.y + 1.0);
            (1.0-t)*white + t*blue
        }
    }
}

fn main() {

    let nx = 200; // rows
    let ny = 100; // columns
    let ns = 100;
    println!("P3"); // the colors are in ASCII
    println!("{} {}", nx, ny);
    println!("255"); // max color
    let world = HitableList {
        hlist: vec![
            Box::new(Sphere {
                radius: 0.5,
                center: Vec3 { x: 0.0, y: 0.0, z: -1.0},
                material: Material::Lambertian(
                    Lambertian {
                        albedo: Vec3 {x: 0.8, y: 0.3, z: 0.3}
                    })
                }),

            Box::new(Sphere {
                radius: 100.0,
                center: Vec3 { x: 0.0, y: -100.5, z: -1.0},
                material: Material::Lambertian(
                    Lambertian {
                        albedo: Vec3 { x: 0.8, y: 0.3, z: 0.3 }
                    })
                }),
            Box::new(Sphere {
                radius: 0.5,
                center: Vec3 { x: 1.0, y: 0.0, z: -1.0},
                material: Material::Metal(
                    Metal {
                        albedo: Vec3 { x: 0.8, y: 0.6, z: 0.2 }
                    })
                }),
            Box::new(Sphere {
                radius: 0.5,
                center: Vec3 { x: -1.0, y: 0.0, z: -1.0},
                material: Material::Metal(
                    Metal {
                        albedo: Vec3 { x: 0.8, y: 0.8, z: 0.8 }
                    })
                })
        ]
    };
    let camera = Camera::default();
    let mut rng = rand::thread_rng();
    rng.next_f32();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let c = {
                (0..ns).fold (Vec3{x: 0.0, y: 0.0, z: 0.0}, |acc, _| {
                    let u = (i as f32 + rng.next_f32())/(nx as f32);
                    let v = (j as f32 + rng.next_f32())/(ny as f32);
                    let ray = camera.get_ray(u, v);
                    acc + color(&ray, &world, 0)
                }) * (1.0 / (ns as f32))
            }.sqrt_fields();
            let ir = (255.99*c.x) as i32;
            let ig = (255.99*c.y) as i32;
            let ib = (255.99*c.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
