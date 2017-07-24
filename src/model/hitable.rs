use model::ray::Ray;
use model::hitrecord::HitRecord;

pub trait Hitable {

    /* 
     * Returns a hit record if the ray hits the item. The hit only "counts" if
     * t is in the specified range.
     */
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
