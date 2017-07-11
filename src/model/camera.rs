use model::ray::Ray;
use model::vector::Vector;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            lower_left_corner: Vec3 { x: -2.0, y: -1.0, z: -1.0 },
            horizontal: Vec3 { x: 4.0, y: 0.0, z: 0.0 },
            vertical: Vec3 { x: 0.0, y: 2.0, z: 0.0 },
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            a: self.origin,
            b: self.lower_left_corner + u*self.horizontal + v*self.vertical
        }
    }
}
