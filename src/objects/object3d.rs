use crate::tools::intersectable::{Intersectable, Intersection};

use super::{model::Model3D, ray::Ray, sphere::Sphere, triangle::Triangle};

pub enum Object3D {
    Model(Model3D),
    Sphere(Sphere),
    Triangle(Triangle)
}

impl Intersectable for Object3D {
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Object3D::Model(model) => model.get_intersection(ray),
            Object3D::Sphere(sphere) => sphere.get_intersection(ray),
            Object3D::Triangle(triangle) => triangle.get_intersection(ray),
        }
    }
}