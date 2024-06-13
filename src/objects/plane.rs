use crate::tools::{
    color_tools::ColorType,
    intersectable::{Intersectable, Intersection},
    vector3::Vector3,
};

use super::{object3d::Object3D, ray::Ray};

#[derive(Debug)]
pub struct Plane {
    pub origin: Vector3,
    pub normal: Vector3,
    pub color: ColorType,
}

impl Plane {
    pub fn new(origin: Vector3, normal: Vector3, color: ColorType) -> Self {
        return Plane {
            origin,
            normal,
            color,
        };
    }
}

impl Intersectable for Plane {
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection> {
        let denom = Vector3::dot_product(&self.normal, &ray.direction);
        if denom.abs() < 1e-6 {
            return None;
        }

        let t =
            Vector3::dot_product(&Vector3::sub(&self.origin, &ray.origin), &self.normal) / denom;

        if t < 0.0 {
            return None;
        }

        Some(Intersection {
            distance: t,
            object: Object3D::Plane(self),
        })
    }
}
