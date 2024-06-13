use std::fmt;

use super::object3d::Object3D;
use super::ray::Ray;
use crate::tools::color_tools::ColorType;
use crate::tools::intersectable::{Intersectable, Intersection};
use crate::tools::vector3::Vector3;

#[derive(Debug)]
pub struct Sphere {
    pub position: Vector3,
    pub radius: f32,
    pub color: ColorType,
}

impl Sphere {
    pub fn new(position: Vector3, radius: f32, color: ColorType) -> Self {
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

        return Some(Intersection {
            distance,
            object: Object3D::Sphere(self)
        });
    }
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