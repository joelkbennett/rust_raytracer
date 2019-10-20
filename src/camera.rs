use crate::point::Point;

#[derive(Debug, PartialEq)]
pub struct Camera {
  pub origin: Point,
  pub lower_left: Point,
  pub horizontal: Point,
  pub vertical: Point,
}

