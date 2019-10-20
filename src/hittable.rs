use crate::ray::Ray;

pub trait Hittable {
  fn get_discriminant(&self, ray: &Ray) -> f64;
  fn hit(&self, ray: &Ray) -> f64;
}