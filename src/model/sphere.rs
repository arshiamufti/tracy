use model::hitable::Hitable;
use model::hitable::HitRecord;
use model::ray::Ray;
use model::vector::Vec3;
use std::f32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub radius: f32,
    pub center: Vec3
}

impl Hitable for Sphere {

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {

        fn in_range(v: f32, t_min: f32, t_max: f32) -> bool {
            v < t_max && v > t_min
        }

        let direction = ray.direction();
        let diff = ray.origin() - self.center;
        let a = direction.dot(&direction);
        let b = 2.0 * direction.dot(&diff);
        let c = diff.dot(&diff) - (self.radius*self.radius);
        let discriminant = (b*b) - (4.0*a*c);

        match discriminant {
            pos if pos <= 0.0 => None,
            _ => {
                let root1 = (-b - discriminant.sqrt())/(2.0*a);
                let root2 = (-b + discriminant.sqrt())/(2.0*a);
                let maybe_t =
                    if in_range(root1, t_min, t_max) { Some(root1) }
                    else if in_range (root2, t_min, t_max) { Some(root2) }
                    else { None };
                match maybe_t {
                    None => None,
                    Some(t) => Some(HitRecord { // TODO: figure out how to map over options in Rust
                        t: t,
                        p: ray.point_at(t),
                        normal: (ray.point_at(t) - self.center).unit() // I hate myself for doing this
                    })
                }
            }
        }
    }
}

