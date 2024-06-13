use crate::tools::intersectable::{Intersectable, Intersection};

use super::{plane::Plane, ray::Ray, sphere::Sphere};

#[derive(Debug, Clone)]
pub enum Object3D<'a> {
    Sphere(&'a Sphere),
    Plane(&'a Plane)
}

impl<'a> Intersectable for Object3D<'a> {
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection<'a>> {
        match *self {
            Object3D::Sphere(sphere) => sphere.get_intersection(ray),
            Object3D::Plane(plane) => plane.get_intersection(ray),
        }
    }
}