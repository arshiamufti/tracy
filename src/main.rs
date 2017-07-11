/*
 * [chapter 6 ]
 *
 * Antialiasing
 *
 * output: sphere_with_antialiasing.png
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
/*
 * [ same as multiple_spheres.rs ]
 * 
 * Now, we hav a "world", which is simply an item that is Hitable (such as a
 * sphere or a list of spheres). A Hitable is anything that implements the `hit`
 * function. `hit` accepts a ray, and a range, and returns an Option<HitRecord>.
 * None is returned when the ray does not hit the item or if it does, but with
 * a value of t (t being the parameter for the ray) that lies outside the
 * specified range. Otherwise, Some(HitRecord) is returned, consisting of the
 * point of intersection, the normal at that point, and the value of t for which
 * the ray hits the item.
 *
 * In this example, we have a HitableList (which is hitable) of two spheres
 * (which are hitable). We colour the spheres if the ray of light hits them
 * (based on their normal at the point of the intersection), and render the blue
 * gradient background if not.
 *
 * see hitable.rs for more.
 */
fn color(ray: &Ray, world: &Hitable) -> Vec3 {
    match world.hit(ray, 0.0, 10000.0) {
        Some(rec) => 0.5 * Vec3 { x: rec.normal.x + 1.0, y: rec.normal.y + 1.0, z: rec.normal.z + 1.0 },
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
 * TODO: document antialiasing
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
            };
            let ir = (255.99*c.x) as i32;
            let ig = (255.99*c.y) as i32;
            let ib = (255.99*c.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
