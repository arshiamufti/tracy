/*
 * [chapter 3]
 *
 * Rays, a simple camera, and a background
 *
 * output: blue.png
 */

mod model;
use model::vector::Vec3 as Vec3;
use model::ray::Ray as Ray;

/*
 * This function accepts a ray and returns the colour of the background. The
 * colour is some gradient of blue depending on the y value of the ray's
 * direction. First, the direction vector is converted to a unit direction so
 * that -1.0 < y < 1.0. Then y is scaled to a t such that 0.0 < t < 1.0. When
 * t = 1.0, we want blue, when t = 0.0, we want white. When t is in between, we
 * want a "blend".
 *
 * (fun fact: the fancy word for a blend is a linear interpolation or 'lerp',
 * and it's of the form (1-t)*start_value + t*end_value), where t is in [0, 1]
 */
fn color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.b.unit();
    let t = 0.5*(unit_direction.y + 1.0);
    let white = Vec3::from_one(1.0);
    let blue = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
    println!("unit_direction={}, t={}", unit_direction.y, t);
    (1.0-t)*white + t*blue
}

fn main() {

    let nx = 200; // rows
    let ny = 100; // columns
    println!("P3"); // the colors are in ASCII
    println!("{} {}", nx, ny);
    println!("255"); // max color
    let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = Vec3 { x: 4.0, y: 0.0, z: 0.0 };
    let vertical = Vec3 { x: 0.0, y: 2.0, z: 0.0 };
    let lower_left = Vec3 { x: -2.0, y: -1.0, z: -1.0 };
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f32)/(nx as f32);
            let v =  (j as f32)/(ny as f32);
            let ray = Ray { a: origin, b: lower_left + u*horizontal + v*vertical };
            let c = color(&ray);
            let ir = (255.99*c.x) as i32;
            let ig = (255.99*c.y) as i32;
            let ib = (255.99*c.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
