use std::fmt;
use std::ops::{Add, Mul, Sub};
use num::NumCast;

#[derive(Clone, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    const ZERO: Vector3 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn zero() -> Self {
        return Vector3::ZERO.clone();
    }

    pub fn new<X: NumCast, Y: NumCast, Z: NumCast>(x: X, y: Y, z: Z) -> Self {
        Vector3 {
            x: NumCast::from(x).unwrap(),
            y: NumCast::from(y).unwrap(),
            z: NumCast::from(z).unwrap(),
        }
    }

    pub fn add(a: &Vector3, b: &Vector3) -> Self {
        return Vector3::new(a.x + b.x, a.y + b.y, a.z + b.z);
    }

    pub fn sub(a: &Vector3, b: &Vector3) -> Self {
        return Vector3::new(a.x - b.x, a.y - b.y, a.z - b.z);
    }

    pub fn dot_product(a: &Vector3, b: &Vector3) -> f32 {
        return a.x * b.x + a.y * b.y + a.z * b.z;
    }
    
    pub fn cross_product(a: &Vector3, b: &Vector3) -> Self {
        return Vector3::new(
            (a.y * b.z) - (a.z * b.y),
            (a.z * b.x) - (a.x * b.z),
            (a.x * b.y) - (a.y * b.x),
        );
    }

    pub fn scalar_multiplication<T: NumCast>(v: &Vector3, scalar: T) -> Self {
        let scalar: f32 = NumCast::from(scalar).unwrap();
        return Vector3::new(v.x * scalar, v.y * scalar, v.z * scalar);
    }

    pub fn magnitude(v: &Vector3) -> f32 {
        return Vector3::dot_product(&v, &v).sqrt();
    }

    pub fn normalize(v: &Vector3) -> Self {
        let magnitude = Vector3::magnitude(&v);

        return Vector3::new(v.x / magnitude, v.y / magnitude, v.z / magnitude);
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        return Self::add(&self, &rhs);
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        return Self::sub(&self, &rhs);
    }
}

impl<T: NumCast> Mul<T> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: T) -> Self::Output {
        Self::scalar_multiplication(&self, rhs)
    }
}

impl Mul for Vector3 {
    type Output = f32;

    fn mul(self, rhs: Vector3) -> Self::Output {
        return Self::dot_product(&self, &rhs);
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vector3: {{ x: {}, y: {}, z: {} }}",
            self.x, self.y, self.z
        )
    }
}
