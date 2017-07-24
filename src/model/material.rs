use model::vector::Vec3;
use model::ray::Ray;
use model::hitrecord::HitRecord;
extern crate rand;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lambertian {
    pub albedo: Vec3
}

impl Lambertian {

    fn random_point_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        let r = Vec3 { x: rng.next_f32(), y: rng.next_f32(), z: rng.next_f32() };
        let point = (2.0 * r) - Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        if point.norm() >= 1.0 {
            Lambertian::random_point_in_unit_sphere()
        } else {
            point
        }
    }

    #[allow(unused_variables)]
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + Lambertian::random_point_in_unit_sphere();
        let scattered = Ray{ a: rec.p, b: target - rec.p };
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metal {
    pub albedo: Vec3
}

impl Metal {

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - (2.0*v.dot(n)*(*n))
    }
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Metal::reflect(&ray_in.direction().unit(), &rec.normal);
        let scattered = Ray { a: rec.p, b: reflected };
        let attenuation = self.albedo;
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal)
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match *self {
            Material::Lambertian(ref l) => l.scatter(ray_in, rec),
            Material::Metal(ref m) => m.scatter(ray_in, rec)
        }
    }
}


