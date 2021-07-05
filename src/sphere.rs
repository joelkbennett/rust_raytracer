use crate::point::Point;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hitRecord::HitRecord;

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
    let half_b = ray_center.dot(&ray.direction);
    let c = ray_center.dot(&ray_center) - (self.radius * self.radius);
    half_b * half_b - a * c
  }

  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &HitRecord) -> bool {
    let ray_center = ray.start - &self.center;
    let a = ray.direction.dot(&ray.direction);
    let half_b = ray_center.dot(&ray.direction);
    let c = ray_center.dot(&ray_center) - (self.radius * self.radius);
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
      return false;
    }

    let discriminant_squared = discriminant.sqrt();
    // the nearest root
    let mut root = (-half_b - discriminant_squared) / a;
    if root < t_min || t_max < root {
      root = (-half_b + discriminant_squared) / a;

      if root < t_min || t_max < root {
        return false;
      }
    }

    hit_record.t = root;
    hit_record.p = ray.point_at_parameter(hit_record.t);
    hit_record.normal = (hit_record.p - &self.center) / self.radius;

    return true;
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