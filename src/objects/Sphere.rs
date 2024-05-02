use std::fmt;

use super::Ray::Ray;
use crate::tools::Intersectable::{Intersectable, Intersection};
use crate::tools::Vector3::Vector3;
use image::Rgb;

pub struct Sphere {
    position: Vector3,
    radius: f64,
    color: Rgb<u8>,
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sphere: {{\n  position: {},\n  radius: {},\n  color: {:?}\n}}",
            self.position, self.radius, self.color
        )
    }
}

impl Sphere {
    pub fn new(position: Vector3, radius: f64, color: Rgb<u8>) -> Self {
        return Sphere {
            position,
            radius,
            color,
        };
    }

    pub fn get_position(&self) -> Vector3 {
        return self.position.clone();
    }
}

impl Intersectable for Sphere {
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection> {
        let l = Vector3::sub(&self.position, &ray.origin);
        let tca = Vector3::dot_product(&l, &ray.get_direction());
        let l2 = Vector3::magnitude(&l).powi(2);
        let d2 = l2 - tca.powi(2);

        if d2 < 0.0 {
            return None;
        }

        let d = d2.sqrt();

        if d.is_nan() || d > self.radius {
            return None;
        }

        let diff = (self.radius.powi(2) - d.powi(2)).sqrt();
        let t0 = tca - diff;
        let t1 = tca + diff;

        let distance = t0.min(t1);
        let position = Vector3::add(
            &ray.origin,
            &Vector3::scalar_multiplication(&ray.get_direction(), distance),
        );
        let normal = Vector3::normalize(&Vector3::sub(&position, &self.position));

        return Some(Intersection {
            distance,
            position,
            normal,
            color: self.color,
        });
    }
}
