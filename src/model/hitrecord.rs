use model::vector::Vec3;
use model::material::Material;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material
}
