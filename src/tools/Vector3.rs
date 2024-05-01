use std;
use std::fmt;

#[derive(Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    const ZERO: Vector3 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn zero() -> Vector3 {
        return Vector3::ZERO.clone();
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Vector3 { x, y, z };
    }

    pub fn add(a: &Vector3, b: &Vector3) -> Vector3 {
        return Vector3::new(a.x + b.x, a.y + b.y, a.z + b.z);
    }

    pub fn sub(a: &Vector3, b: &Vector3) -> Vector3 {
        return Vector3::new(a.x - b.x, a.y - b.y, a.z - b.z);
    }

    pub fn dot_product(a: &Vector3, b: &Vector3) -> f64 {
        return a.x * b.x + a.y * b.y + a.z * b.z;
    }

    pub fn cross_product(a: &Vector3, b: &Vector3) -> Vector3 {
        return Vector3::new(
            a.y * b.z - b.z * b.y,
            a.z * b.x - b.x - b.z,
            a.x * b.y - b.y * b.x,
        );
    }

    pub fn scalar_multiplication(v: &Vector3, scalar: f64) -> Vector3 {
        return Vector3::new(v.x * scalar, v.y * scalar, v.z * scalar);
    }

    pub fn magnitude(v: &Vector3) -> f64 {
        return Vector3::dot_product(&v, &v).sqrt();
    }

    pub fn normalize(v: &Vector3) -> Vector3 {
        let magnitude = Vector3::magnitude(&v);

        return Vector3::new(v.x / magnitude, v.y / magnitude, v.z / magnitude);
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
