use model::vector::Vec3;
use model::ray::Ray;

pub trait Hitable {

    /* 
     * Returns a hit record if the ray hits the item. The hit only "counts" if
     * t is in the specified range.
     */
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}
