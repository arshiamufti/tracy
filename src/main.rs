/*
 * [chapter 4]
 *
 * Spheres!
 *
 * output: simplesphere.png
 */

pub use self::vector::Vec3;
mod vector;
pub use self::ray::Ray;
mod ray;

fn does_hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let direction = ray.direction();
    let diff = ray.origin() - center;
    let a = direction.dot(&direction);
    let b = 2.0 * direction.dot(&diff);
    let c = diff.dot(&diff) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);

    discriminant > 0.0
}
/*
 * This function accepts a ray and returns the colour of the background. The
 * colour is some gradient of blue depending on the y value of the ray's
 * direction. (see more in simple_camera.rs)
 */
fn color(ray: &Ray) -> Vec3 {
    let red = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    let blue = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
    let white = Vec3::from_one(1.0);

    let radius = 0.5;
    let center = Vec3 { x: 0.0, y: 0.0, z: -1.0 };
    if does_hit_sphere(center, radius, &ray) {
        return red;
    }

    let unit_direction = ray.direction().unit();
    let t = 0.5*(unit_direction.y + 1.0);
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
