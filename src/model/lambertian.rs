use model::ray::Ray;
use model::vector::Vec3;

pub struct Lambertian {
    albedo: Vec3
}

#[derive(Clone, Copy, Debug, PartialEq)]
impl Material for Lambertian {

    pub fn scatter(ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal /* + random point in unit sphere */;
        let scattered = Ray{ a: rec.p, b: target - rec.p };
        let attenuation = self.albedo;
        Some(attenuation, scattered)
    }
}
