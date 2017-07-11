/*
 * [chapter 4]
 *
 * Spheres!
 *
 * output: simplesphere.png
 */

mod model;
use model::vector::Vec3 as Vec3;
use model::ray::Ray as Ray;

/*
 * We have a sphere, defined by the coordinates of its center and its radius,
 * call it S(C, R). And we have a ray, represented by an origin point (A) and a
 * direction vector (B) and with the equation p(t) = A + t*B (see ray.rs).
 * To display the sphere in the image, we need to figure out if a ray of light
 * coming from the camera/eye hits the sphere or not, and if it does, colour it
 * a certain colour.
 *
 * A ray hits the sphere if it satisfies the equation of the sphere. The
 * equation of a sphere centered at C = (cx, cy, cz) with radius R is (x-cx)^2 +
 * (y-cy)^2 + (z-cz)^2 = R^2 which is the same as C.Q = R*R, where Q =
 * (x, y, z).
 *
 * So, a ray p(t) hits the sphere S(C, R) if p(t) satisfies the equation of the
 * sphere. So we want to solve
 *
 * (p(t) - C).(p(t)-C) = R^2 = 0
 *
 * which is a quadratic equation in t.
 *
 * Luckily, high school prepared me for this challenge.
 *
 */
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
 * colour is either:
 * - some gradient of blue depending on the y value of the ray's
 * direction. (see more in simple_camera.rs)
 * - the colour red if the ray hits the defined sphere
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
