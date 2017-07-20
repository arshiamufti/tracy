use model::ray::Ray;
use model::vector::Vec3;

pub struct Metal {
    albedo: Vec3
}

#[derive(Clone, Copy, Debug, PartialEq)]
impl Material for Metal {

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        // TODO: IMPLEMENT
    }
    pub fn scatter(ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        // TODO: IMPLEMENT
    }
}
