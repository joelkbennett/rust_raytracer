use crate::point::Point;
use crate::ray::Ray;
use crate::hittable::Hittable;

#[derive(Debug, PartialEq)]
pub struct Sphere {
  pub center: Point,
  pub radius: f64,
}

impl Hittable for Sphere {
  /// Returns the discriminant; for testing
  fn get_discriminant(&self, ray: &Ray) -> f64 {
    let ray_center = ray.start - &self.center;
    let a = ray.direction.dot(&ray.direction);
    let b = ray_center.dot(&ray.direction) * 2.0;
    let c = ray_center.dot(&ray_center) - (self.radius * self.radius);
    b * b - 4.0 * a * c
  }

  fn hit(&self, ray: &Ray) -> f64 {
    let ray_center = ray.start - &self.center;
    let a = ray.direction.dot(&ray.direction);
    let b = ray_center.dot(&ray.direction) * 2.0;
    let c = ray_center.dot(&ray_center) - (self.radius * self.radius);
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
      -1.0
    } else {
      (-b - discriminant.sqrt()) / (2.0 * a)
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_ray_hit() {
    let sphere = Sphere{ center: Point{ x: 0.0, y: 0.0, z: 0.0 }, radius: 2.0 };
    let ray = Ray{ start: Point{ x: 0.0, y: 0.0, z: -4.0 }, direction: Point{ x: 0.0, y: 0.0, z: 4.0 }};
    let hit = sphere.hit(&ray);
    assert_eq!(hit, 0.5);
  }

  #[test]
  fn test_ray_miss() {
    let sphere = Sphere{ center: Point{ x: 0.0, y: 0.0, z: 0.0 }, radius: 2.0 };
    let ray = Ray{ start: Point{ x: 0.0, y: 0.0, z: -4.0 }, direction: Point{ x: 4.0, y: 0.0, z: 0.0 }};
    let hit = sphere.hit(&ray);
    assert_eq!(hit, -1.0);
  }

  #[test]
  fn test_miss_discriminant() {
    let sphere = Sphere{ center: Point{ x: 0.0, y: 0.0, z: 0.0 }, radius: 2.0 };
    let ray = Ray{ start: Point{ x: 0.0, y: 0.0, z: -4.0 }, direction: Point{ x: 4.0, y: 0.0, z: 0.0 }};
    let hit = sphere.get_discriminant(&ray);
    assert!(hit < 0.0);
  }

  #[test]
  fn test_single_hit_discriminant() {
    let sphere = Sphere{ center: Point{ x: 0.0, y: 0.0, z: 0.0 }, radius: 2.0 };
    let ray = Ray{ start: Point{ x: 0.0, y: 2.0, z: 10.0 }, direction: Point{ x: 0.0, y: 0.0, z: -10.0 }};
    let hit = sphere.get_discriminant(&ray);
    assert!(hit == 0.0);
  }

  #[test]
  fn test_double_hit_discriminant() {
    let sphere = Sphere{ center: Point{ x: 0.0, y: 0.0, z: 0.0 }, radius: 2.0 };
    let ray = Ray{ start: Point{ x: 0.0, y: 0.0, z: -4.0 }, direction: Point{ x: 0.0, y: 0.0, z: 4.0 }};
    let hit = sphere.get_discriminant(&ray);
    assert!(hit > 1.0);
  }

  // #[test]
  // fn test_single_discriminant_value() {
  //   let sphere = Sphere{ center: Point{ x: 0.0, y: 0.0, z: 0.0 }, radius: 2.0 };
  //   let ray = Ray{ start: Point{ x: 0.0, y: 0.0, z: 10.0 }, direction: Point{ x: 0.0, y: 0.0, z: 10.0 }};
  //   let hit = sphere.get_discriminant(&ray);
  //   assert_eq!(hit, 10.0);
  // }
}