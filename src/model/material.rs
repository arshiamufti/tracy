use model::vector::Vec3;
use model::ray::Ray;

pub trait Material {

    /* 
     * Given a hit record for the material and an incident ray, returns the
     * scattered ray and attenuation (or None, if the entire ray is absorbed).
     */
    fn scatter(ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> ;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitRecord1 {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<Material>
}
