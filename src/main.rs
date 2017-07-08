/*
 * [chapter 2]
 *
 * Slightly improved hello world (used the Vec3 class)
 */

pub use self::vector::Vec3;
mod vector;

fn main() {

    let nx = 200; // rows
    let ny = 100; // columns
    println!("P3"); // the colors are in ASCII
    println!("{} {}", nx, ny);
    println!("255"); // max color
    // now, rbg values
    // pixels are written out in rows, left to right
    // rows are written out top to bottom
    // by convention, each of the red/blue/green components will range from
    // 0.0 to 1.0
    for j in (0..ny).rev() {
        for i in 0..nx {
            let v = Vec3 {
                x: (i as f32)/(nx as f32),
                y:  (j as f32)/(ny as f32),
                z:  0.2
            };
            let ir = (255.99*v.x) as i32;
            let ig = (255.99*v.y) as i32;
            let ib = (255.99*v.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
