use std::fmt;

use image::Rgb;

use crate::tools::{intersectable::{Intersectable, Intersection}, vector3::Vector3};

use super::object3d::Object3D;

static EPSILON: f32 = 0.0000000000001;

#[derive(Clone, Debug)]
pub struct Triangle<'a> {
    pub vertices: [Vector3; 3],
    pub normals: [Vector3; 3],
    pub model: &'a Object3D
}

impl<'a> Triangle<'a> {
    pub fn new<'b>(v0: Vector3, v1: Vector3, v2: Vector3, color: &'b Color ) -> Triangle<'b> {
        return Triangle {
            vertices: [v0, v1, v2],
            normals: [Vector3::zero(), Vector3::zero(), Vector3::zero()],
            color
        }
    }

    pub fn get_normal(&self) -> Vector3 {
        let v = Vector3::sub(&self.vertices[1], &self.vertices[0]);
        let w = Vector3::sub(&self.vertices[0], &self.vertices[2]);

        return Vector3::normalize(&Vector3::cross_product(&v, &w));
    }

    pub fn get_vertices(&self) -> [Vector3; 3] {
        return self.vertices.clone();
    }

    pub fn set_vertices(&mut self, vertices: [Vector3; 3]) {
        self.vertices = vertices;
    }
}

impl<'a> Intersectable for Triangle<'a> {
    fn get_intersection(&self, ray: &super::ray::Ray) -> Option<Intersection> {
        let v2v0 = Vector3::sub(&self.vertices[2], &self.vertices[0]);
        let v1v0 = Vector3::sub(&self.vertices[1], &self.vertices[0]);
        let vector_p = Vector3::cross_product(&ray.get_direction(), &v1v0);
        let inv_det = 1.0 / Vector3::dot_product(&v2v0, &vector_p);
        let vector_t = Vector3::sub(&ray.origin, &self.vertices[0]);
        let u = inv_det * Vector3::dot_product(&vector_t, &vector_p);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let vector_q = Vector3::cross_product(&vector_t, &v2v0);
        let v = inv_det * Vector3::dot_product(&ray.get_direction(), &vector_q);

        if v < 0.0 || (u + v) > (1.0 + EPSILON) {
            return None;
        }

        let t = inv_det * Vector3::dot_product(&vector_q, &v1v0);

        return Some(Intersection {
            distance: t,
            object: &Object3D::Triangle(*self)
        });
    }
}

impl<'a> fmt::Display for Triangle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Triangle: {{\n  v0: {},\n  v1: {},\n  v2: {}\n}}",
            self.vertices[0], self.vertices[1], self.vertices[2]
        )
    }
}