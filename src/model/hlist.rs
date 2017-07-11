use model::hitable::Hitable;
use model::hitable::HitRecord;
use model::ray::Ray;
use std::f32;

pub struct HitableList {
    pub hlist: Vec<Box<Hitable>>
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hlist.iter().fold(None, |acc, ref x| {
            match acc {
                None => x.hit(ray, t_min, t_max),
                Some(hit_record) => {
                    let rec = x.hit(ray, t_min, hit_record.t);
                    match rec {
                        None => acc,
                        Some(_) => rec
                    }
                }
            }
        })
    }
}

