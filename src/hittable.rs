use crate::ray::Ray;
use crate::hitRecord::HitRecord;

pub trait Hittable {
  fn get_discriminant(&self, ray: &Ray) -> f64;
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &HitRecord) -> bool;
}