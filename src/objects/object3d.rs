use crate::tools::intersectable::{Intersectable, Intersection};

use super::{model3D::Model3D, plane::Plane, ray::Ray, sphere::Sphere, triangle::Triangle};

#[derive(Debug, Clone)]
pub enum Object3D<'a> {
    Sphere(&'a Sphere),
    Plane(&'a Plane),
    Triangle(&'a Triangle),
}

impl<'a> Intersectable for Object3D<'a> {
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection<'a>> {
        match *self {
            Object3D::Sphere(sphere) => sphere.get_intersection(ray),
            Object3D::Plane(plane) => plane.get_intersection(ray),
            Object3D::Triangle(triangle) => triangle.get_intersection(ray),
        }
    }
}
