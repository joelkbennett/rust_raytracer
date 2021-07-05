use crate::point::Point;
use crate::ray::Ray;

pub struct HitRecord {
  pub p: Point,
  pub normal: Point,
  pub t: f64,
  pub is_front_face: bool,
}

impl HitRecord {
  pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Point) {
    self.is_front_face = ray.direction.dot(&outward_normal) < 0.0;
    self.normal = if self.is_front_face {
      outward_normal
    } else {
      outward_normal.flip()
    }
  }
}