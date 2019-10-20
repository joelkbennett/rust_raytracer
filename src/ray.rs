use crate::point::Point;
use crate::color::Color;
use crate::sphere::Sphere;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub start: Point,
    pub direction: Point,
}

impl Ray {
    pub fn origin(&self) -> &Point {
        &self.start
    }

    pub fn direction(&self) -> &Point {
        &self.direction
    }

    pub fn point_at_parameter(self, t: f64) -> Point {
        (self.direction * t) + &self.start
    }

    pub fn color_at_ray(&self, sphere: &Sphere) -> Color {
        let hit = sphere.hit(&self);
        if hit > 0.0 {
            let normal = (self.point_at_parameter(hit) - &sphere.center).unit_vector();
            return Color{r: (normal.x + 1.0) as u8, g: (normal.y + 1.0) as u8, b: (normal.z + 1.0) as u8 } * 0.5;
        }

        let unit_direction = self.direction.unit_vector();
        let t = unit_direction.y + 1.0;

        let result = (Point{x: 1.0, y:1.0, z:1.0} * (1.0 - t)) + &(Point{x: 0.5, y:0.7, z: 1.0} * t);
        return Color{ r: (255.99 * result.x) as u8, g: (255.99 * result.y) as u8, b: (255.99 * result.z) as u8 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_return_origin() {
        let ray = Ray{start: Point{x:0.0, y:0.0, z:0.0}, direction: Point{x:2.0, y:3.0, z:1.0}};
        assert_eq!(*ray.origin(), Point{x:0.0, y:0.0, z:0.0});
    }

    #[test]
    fn test_should_return_direction() {
        let ray = Ray{start: Point{x:0.0, y:0.0, z:0.0}, direction: Point{x:2.0, y:3.0, z:1.0}};
        assert_eq!(*ray.direction(), Point{x:2.0, y:3.0, z:1.0});
    }

    #[test]
    fn test_point_at_parameter() {
        let ray = Ray{start: Point{x:0.0, y:0.0, z:0.0}, direction: Point{x:2.0, y:3.0, z:1.0}};
        assert_eq!(ray.point_at_parameter(4.0), Point{x: 8.0, y: 12.0, z: 4.0});
    }

    #[test]
    fn test_point_at_start() {
        let ray = Ray{start: Point{x:1.0, y:2.0, z:8.0}, direction: Point{x:2.0, y:3.0, z:1.0}};
        assert_eq!(ray.point_at_parameter(0.0), Point{x: 1.0, y: 2.0, z: 8.0});
    }
}
