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
use std::f32;

extern crate rand;
use rand::Rng;

fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let r = Vec3 { x: rng.next_f32(), y: rng.next_f32(), z: rng.next_f32() };
    let point = (2.0 * r) - Vec3 { x: 1.0, y: 1.0, z: 1.0 };
    if point.norm() >= 1.0 {
        random_point_in_unit_sphere()
    } else {
        point
    }
}
/*
 * [ see anitaliasing.rs, hitable.rs ]
 * 
 * Say we have a hit record with hit point p. Construct a unit sphere tangent to
 * it. If -N is the normal at p pointing inwards, the the center of the sphere
 * is p + N. Let `target` be a random point in that sphere. Construct a ray
 * originating from p, going towards `target`, and colour the world. In this
 * example, `target` could be a point that isn't hitable (so we colour it some
 * shade of blue) or it could be a point on another object.
 *
 * 50% of the light is absorbed by the opaque object (this helps in rendering
 * shadows too--if the first ray of light is directed at the crack between the
 * two spheres, the reflected ray will probably also be directed towards a
 * surface, so light is successively lost).
 *
 * // TODO: improve documentation above
 */
fn color(ray: &Ray, world: &Hitable) -> Vec3 {
    match world.hit(ray, 0.001, 10000.0) {
        Some(rec) => {
            let target = rec.p + rec.normal + random_point_in_unit_sphere();
            0.5 * color( & Ray { a: rec.p, b: target - rec.p }, world)
        }
        None => {
            let blue = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
            let white = Vec3::from_one(1.0);
            let unit_direction = ray.direction().unit();
            let t = 0.5*(unit_direction.y + 1.0);
            (1.0-t)*white + t*blue
        }
    }
}

/*
 * [ first paragraph taken from Ray Tracing in One Weekend by Peter Shirley ]
 *
 * When a real camera takes a picture, there are usually no jaggies along edges
 * because the edge pixels are a blend of some foreground and some background.
 * We can get the same effect by averaging a bunch of samples inside each pixel.
 *
 * Instead of getting the colour at only *one* point of the pixel, we take `ns`
 * samples within the pixel, and average the colour. 
 */
fn main() {

    let nx = 200; // rows
    let ny = 100; // columns
    let ns = 100;
    println!("P3"); // the colors are in ASCII
    println!("{} {}", nx, ny);
    println!("255"); // max color
    let world = HitableList {
        hlist: vec![
            Box::new(Sphere { radius: 0.5, center: Vec3 { x: 0.0, y: 0.0, z: -1.0} }),
            Box::new(Sphere { radius: 100.0, center: Vec3 { x: 0.0, y: -100.5, z: -1.0} })
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
                    acc + color(&ray, &world)
                }) * (1.0 / (ns as f32))
            }.sqrt_fields();
            let ir = (255.99*c.x) as i32;
            let ig = (255.99*c.y) as i32;
            let ib = (255.99*c.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
