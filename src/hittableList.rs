use crate::hittable::Hittable;
use crate::hitRecord::HitRecord;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct HittableList {
  pub objects: Vec<Hittable>;
}

impl Hittable for HittableList {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &HitRecord) -> bool {
    let temp_record = HitRecord{};
    let hit_anything = false;
    let mut closest = t_max;

    // iterate over a list of hittables
    for object in &self.objects.iter();
      if (object.hit(&ray, t_min, t_max, &hit_record)) {
        hit_anything = true;
        closest = temp_record.t;
        hit_record = temp_record;
      }
    }

    return hit_anything;
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_a_hittable_list() {
    let sphere_one = ;
    let sphere_two = ;
    let hittableList = HittableList { }
  }
}