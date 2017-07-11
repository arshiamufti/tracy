/*
 * [chapter 5]
 *
 * Surface Normals
 *
 * output: sphere.png
 */

mod model;
use model::vector::Vec3 as Vec3;
use model::ray::Ray as Ray;
use std::f32;

/*
 * If the ray does hit the sphere, return the smallest solution to the equation
 */
fn does_hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let direction = ray.direction();
    let diff = ray.origin() - center;
    let a = direction.dot(&direction);
    let b = 2.0 * direction.dot(&diff);
    let c = diff.dot(&diff) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt())/(2.0*a)
    }
}

/*
 * see documentation in simple_sphere.rs
 *
 * If the ray does hit the sphere, get the normal to the sphere at a point of
 * intersection. Scale each component so that it is between 0.0 and 1.0. Get
 * rbg values by mapping x to r, y to g, and z to b.
 */
fn color(ray: &Ray) -> Vec3 {
    let blue = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
    let white = Vec3::from_one(1.0);

    let radius = 0.5;
    let center = Vec3 { x: 0.0, y: 0.0, z: -1.0 };

    let t = does_hit_sphere(center, radius, &ray);
    if t > 0.0 {
        let normal = (ray.point_at(t) - center).unit();
        // each component of the normal is between -1.0 and 1.0. Scale it so
        // that it is between 0.0 and 1.0
        return 0.5 * Vec3 { x: normal.x + 1.0, y: normal.y + 1.0, z: normal.z + 1.0 };
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
