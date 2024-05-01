use image::Rgb;
use crate::tools::Intersectable::{Intersectable, Intersection};
use crate::tools::Vector3::Vector3;
use super::Ray::Ray;

pub struct Sphere {
    position: Vector3,
    radius: f64,
    color: Rgb<f32>
}

impl Sphere {
    pub fn new(position: Vector3, radius: f64, color: Rgb<f32>) -> Self {
        return Sphere {
            position,
            radius,
            color
        }
    }
}

impl Intersectable for Sphere {
    fn get_intersection(&self, ray: Ray) -> Option<Intersection> {
        let l = Vector3::sub(&self.position, &ray.origin);
        let tca = Vector3::dot_product(&l, &ray.get_direction());
        let l2 = Vector3::magnitude(&l).powi(2);
        let d2 = l2 - tca.powi(2);

        if d2 < 0.0 {
            return None;
        }

        let d = d2.sqrt();
        let diff = (self.radius.powi(2) - d.powi(2)).sqrt();
        let t0 = tca - diff;
        let t1 = tca + diff;
        
        let distance = t0.min(t1);
        let position = Vector3::add(&ray.origin, &Vector3::scalar_multiplication(&ray.get_direction(), distance));
        let normal = Vector3::normalize(&Vector3::sub(&position, &self.position));

        return Some(Intersection {
            distance,
            position,
            normal,
            color: self.color
        })
    }
}