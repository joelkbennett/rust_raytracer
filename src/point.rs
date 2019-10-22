use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn length(&self) -> f64 {
        (self.dot(&self)).sqrt() as f64
    }

    pub fn unit_vector(&self) -> Point {
        *self / self.length()
    }

    pub fn dot(&self, other: &Point) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    #[allow(dead_code)]
    pub fn cross(&self, other: &Point) -> Point {
        Point{
            x: self.y * other.z - self.z * other.y,
            y: self.x * other.z - self.z * other.x,
            z: self.x * other.y - self.y * other.x
        }
    }
}

impl ops::Add<&Point> for Point {
    type Output = Point;
    fn add(self, other: &Point) -> Point {
        Point{ x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl ops::Sub<&Point> for Point {
    type Output = Point;
    fn sub(self, other: &Point) -> Point {
        Point{ x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl ops::Mul<&Point> for Point {
    type Output = Point;
    fn mul(self, other: &Point) -> Point {
        Point{ x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl ops::Div<&Point> for Point {
    type Output = Point;
    fn div(self, other: &Point) -> Point {
        Point{ x: self.x / other.x, y: self.y / other.y, z: self.z / other.z }
    }
}

impl ops::Mul<f64> for Point {
    type Output = Point;
    fn mul(self, other: f64) -> Point {
        Point{ x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl ops::Div<f64> for Point {
    type Output = Point;
    fn div(self, other: f64) -> Point {
        Point{ x: self.x / other, y: self.y / other, z: self.z / other }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_points() {
        let point_one = Point{ x: 2.0, y: 4.0, z: 0.0 };
        let point_two = Point{ x: 1.0, y: 2.0, z: 3.0 };
        let result = point_one + &point_two;
        assert_eq!(result, Point{ x: 3.0, y: 6.0, z: 3.0});
    }

    #[test]
    fn test_adding_negative_points() {
        let point_one = Point{ x: 2.0, y: 4.0, z: 6.0 };
        let point_two = Point{ x: -4.0, y: -2.0, z: -6.0 };
        let result = point_one + &point_two;
        assert_eq!(result, Point{ x: -2.0, y: 2.0, z: 0.0});
    }

    #[test]
    fn test_subtracting_points() {
        let point_one = Point{ x: 2.0, y: 4.0, z: 0.0 };
        let point_two = Point{ x: 1.0, y: 2.0, z: 3.0 };
        let result = point_one - &point_two;
        assert_eq!(result, Point{ x: 1.0, y: 2.0, z: -3.0});
    }

    #[test]
    fn test_subtracting_negative_points() {
        let point_one = Point{ x: 2.0, y: 4.0, z: 6.0 };
        let point_two = Point{ x: -4.0, y: -2.0, z: -6.0 };
        let result = point_one - &point_two;
        assert_eq!(result, Point{ x: 6.0, y: 6.0, z: 12.0});
    }

    #[test]
    fn test_multiplying_points() {
        let point_one = Point{ x: 2.0, y: 4.0, z: 0.0 };
        let point_two = Point{ x: 1.0, y: 2.0, z: 3.0 };
        let result = point_one * &point_two;
        assert_eq!(result, Point{ x: 2.0, y: 8.0, z: 0.0});
    }

    #[test]
    fn test_multiplying_point_by_float() {
        let point = Point{ x: 2.0, y: 4.0, z: 0.0 };
        let result = point * 4.5;
        assert_eq!(result, Point{ x: 9.0, y: 18.0, z: 0.0});
    }

    #[test]
    fn test_dividing_points() {
        let point_one = Point{ x: 2.0, y: 4.0, z: 12.0 };
        let point_two = Point{ x: 1.0, y: 2.0, z: 3.0 };
        let result = point_one / &point_two;
        assert_eq!(result, Point{ x: 2.0, y: 2.0, z: 4.0});
    }

    #[test]
    fn test_dividing_point_by_float() {
        let point = Point{ x: 2.0, y: 4.0, z: 0.0 };
        let result = point / 4.0;
        assert_eq!(result, Point{ x: 0.5, y: 1.0, z: 0.0});
    }

    #[test]
    fn test_dot_product() {
        let point_one = Point{ x: 2.0, y: 4.0, z: 12.0 };
        let point_two = Point{ x: 1.0, y: 2.0, z: 3.0 };
        let result = point_one.dot(&point_two);
        assert_eq!(result, 46.0);
    }

    #[test]
    fn test_cross_product() {
        let point_one = Point{ x: 2.0, y: 6.0, z: 4.0 };
        let point_two = Point{ x: 1.0, y: 2.0, z: 3.0 };
        let result = point_one.cross(&point_two);
        assert_eq!(result, Point{x: 10.0, y: 2.0, z: -2.0});
    }
}
