use image::Rgb;

use crate::tools::{Intersectable::{Intersectable, Intersection}, Vector3::Vector3};

static EPSILON: f64 = 0.0000000000001;

pub struct Triangle {
    vertices: [Vector3; 3],
    normals: [Vector3; 3],
    color: Rgb<u8>
}

impl Triangle {
    pub fn new(v0: Vector3, v1: Vector3, v2: Vector3, color: Rgb<u8>) -> Self {
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
}

// public Intersection getIntersection(Ray ray) {
//     Intersection intersection = new Intersection(null, -1, null, null);

//     Vector3D[] vert = getVertices();
//     Vector3D v2v0 = Vector3D.substract(vert[2], vert[0]);
//     Vector3D v1v0 = Vector3D.substract(vert[1], vert[0]);
//     Vector3D vectorP = Vector3D.crossProduct(ray.getDirection(), v1v0);
//     double det = Vector3D.dotProduct(v2v0, vectorP);
//     double invDet = 1.0 / det;
//     Vector3D vectorT = Vector3D.substract(ray.getOrigin(), vert[0]);
//     double u = invDet * Vector3D.dotProduct(vectorT, vectorP);

//     if (!(u < 0 || u > 1)) {
//         Vector3D vectorQ = Vector3D.crossProduct(vectorT, v2v0);
//         double v = invDet * Vector3D.dotProduct(ray.getDirection(), vectorQ);
//         if (!(v < 0 || (u + v) > (1.0 + EPSILON))) {
//             double t = invDet * Vector3D.dotProduct(vectorQ, v1v0);
//             intersection.setDistance(t);
//         }
//     }

//     return intersection;
// }

impl Intersectable for Triangle {
    fn get_intersection(&self, ray: &super::Ray::Ray) -> Option<crate::tools::Intersectable::Intersection> {
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
            position: vector_p,
            normal: self.get_normal(),
            color: self.color,
        })
    }
}