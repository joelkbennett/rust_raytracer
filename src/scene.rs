use crate::camera::Camera;
use crate::sphere::Sphere;

#[derive(Debug, PartialEq)]
pub struct Scene {
  pub width: u32,
  pub height: u32,
  pub camera: Camera,
  pub objects: Vec<Sphere>,
}