use std::ops::{Add, Sub, Mul, Div};
use std::f64;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// here's everything Vec3 should implement:
// - dot and cross product
// - length, squared length, and unit vector
// - ...

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        let length = self.length();

        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn total(&self) -> f64 {
        self.x + self.y + self.z
    }

    pub fn abs(&self) -> Vec3 {
        Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn colorize(&self) -> [u8; 3] {
        let k = 1.0;

        // TODO: simplify

        // remove colors less than 0
        // this shouldn't happen, but just in case
        let mut color = Vec3 {
            x: self.x.max(0.0),
            y: self.y.max(0.0),
            z: self.z.max(0.0),
        };

        // compress colorspace
        // note, in the future, k should be average color across all channels

        color = (3.0 * color) / (2.0 * k + color);

        {
            let away = (color.y - 1.0).max(0.0) + (color.z - 1.0).max(0.0);
            color.x = (color.x.min(1.0)) + (away - (away - (1.0 - color.x).max(0.0)).max(0.0));
        }

        {
            let away = (color.x - 1.0).max(0.0) + (color.z - 1.0).max(0.0);
            color.y = (color.y.min(1.0)) + (away - (away - (1.0 - color.y).max(0.0)).max(0.0));
        }

        {
            let away = (color.x - 1.0).max(0.0) + (color.y - 1.0).max(0.0);
            color.z = (color.z.min(1.0)) + (away - (away - (1.0 - color.z).max(0.0)).max(0.0));
        }

        // gamma correction and range normalization
        color.x = color.x.sqrt() * 255.9;
        color.y = color.y.sqrt() * 255.9;
        color.z = color.z.sqrt() * 255.9;

        return [color.x as u8, color.y as u8, color.z as u8];
    }

    // -> ()
    pub fn print(&self) {
        println!("{:?}", (self.x, self.y, self.z))
    }
}

// - ...
// - Vec3 adding Vec3s and scalars
// - ...

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z,
        }
    }
}

// - ...
// - Vec3 subtracting Vec3s and scalars
// - ...

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self - other.x,
            y: self - other.y,
            z: self - other.z,
        }
    }
}

// - ...
// - Vec3 piecewise multiplication for Vec3s and scalars
// - ...

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

// - ...
// - divide vec3 by scalar
// - ...

fn clamp(number: f64) -> f64 {
    // get rid of infinities?
    return if number.abs() == (1.0 / 0.0) {f64::MAX} else {number}
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: clamp(self.x / other.x),
            y: clamp(self.y / other.y),
            z: clamp(self.z / other.z),
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: clamp(self.x / other),
            y: clamp(self.y / other),
            z: clamp(self.z / other),
        }
    }
}

// - ...
// - equality

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }
}

// and now, some tests

#[cfg(test)]
pub mod test {
    use super::Vec3;

    #[test]
    fn test_new() {
        let vec = Vec3::new(1.0, -2.0, 0.3);
        vec.print();
    }

    #[test]
    fn test_xyz() {
        let vec = Vec3::new(1.0, -2.0, 0.3);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, -2.0);
        assert_eq!(vec.z, 0.3);
    }

    #[test]
    fn test_add_vec() {
        let vec = Vec3::new(1.0, -2.0, 0.3);
        let other = Vec3::new(-2.0, -7.3, 1.2);

        let test = Vec3::new(-1.0, -9.3, 1.5);

        // Vec3 and Vec3
        assert_eq!(
            vec + other,
            test,
        );
    }

    #[test]
    fn test_add_scalar() {
        let vec = Vec3::new(1.0, -2.0, 0.3);
        let scalar = 93.8;

        let test = Vec3::new(94.8, 91.8, 94.1);

        // Vec3 and scalar
        assert_eq!(
            vec + scalar,
            test,
        );
    }

    #[test]
    fn test_sub_vec() {
        let vec = Vec3::new(1.0, -2.0, 0.3);
        let other = Vec3::new(-2.0, -7.3, 1.2);

        let test = Vec3::new(3.0, 5.3, -0.8999999999999999);

        // Vec3 and Vec3
        assert_eq!(
            vec - other,
            test,
        );
    }

    #[test]
    fn test_sub_scalar() {
        let vec = Vec3::new(1.0, -2.0, 0.3);
        let scalar = 93.8;

        let test = Vec3::new(-92.8, -95.8, -93.5);

        // Vec3 and scalar
        assert_eq!(
            vec - scalar,
            test,
        );
    }

    #[test]
    fn test_mul_vec() {
        let vec = Vec3::new(1.0, -2.0, 0.3);
        let other = Vec3::new(-2.0, -7.3, 1.2);

        let test = Vec3::new(-2.0, 14.6, 0.36);

        // Vec3 and Vec3
        assert_eq!(
            vec * other,
            test,
        );
    }

    #[test]
    fn test_mul_scalar() {
        let vec = Vec3::new(1.0, -2.0, 0.3);
        let scalar = 93.8;

        let test = Vec3::new(93.8, -187.6, 28.139999999999997);

        // Vec3 and scalar
        assert_eq!(vec * scalar, test);
    }

    #[test]
    fn test_div_vec() {
        let vec = Vec3::new(1.0, -2.0, 0.3);
        let other = Vec3::new(-2.0, -7.3, 1.2);

        let test = Vec3::new(-0.5, 0.273972602739726, 0.25);

        // Vec3 and Vec3
        assert_eq!(vec / other, test);
    }

    #[test]
    fn test_div_scalar() {
        // TODO: div by 0
        let vec = Vec3::new(1.0, -2.0, 0.3);
        let scalar = 4.0;

        let test = Vec3::new(0.25, -0.5, 0.075);

        // Vec3 and scalar
        assert_eq!(
            vec / scalar,
            test,
        );
    }

    #[test]
    fn test_dot() {
        let vec: Vec3 = Vec3::new(0.2, 0.4, 0.7);
        let other: Vec3 = Vec3::new(0.1, 0.3, 0.3);

        assert_eq!(vec.dot(&other), 0.35);
    }

    #[test]
    fn test_cross() {
        let vec: Vec3 = Vec3::new(3.0, -3.0, 1.0);
        let other: Vec3 = Vec3::new(4.0, 9.0, 2.0);

        let test: Vec3 = Vec3::new(-15.0, -2.0, 39.0);

        assert_eq!(
            vec.cross(&other),
            test,
        );
    }

    #[test]
    fn test_length() {
        let vec = Vec3::new(0.0, -3.0, 4.0);
        assert_eq!(vec.length(), 5.0);
    }

    #[test]
    fn test_length_squared() {
        let vec = Vec3::new(5.0, -3.0, 4.0);
        assert_eq!(vec.length_squared(), 50.0);
    }

    #[test]
    fn test_unit() {
        let vec = Vec3::new(0.0, -3.0, 4.0);
        let test = Vec3::new(0.0, -0.6, 0.8);

        assert_eq!(
            vec.unit(),
            test,
        );
    }

    #[test]
    fn test_tone_map() {
        let over = Vec3::new(10.0, 10.0, 10.0);

        assert_eq!(
            over.tone_map(&1.0),
            Vec3::new(0.0, 0.0, 0.0)
        )
    }
}
