use std::fmt;

use image::Rgb;
use obj::{Obj, ObjError};

use super::triangle::Triangle;
use crate::tools::{
    intersectable::{Intersectable, Intersection},
    vector3::Vector3,
};

#[derive(Debug)]
pub struct Model3D<'a> {
    pub position: Vector3,
    pub triangles: Vec<Triangle<'a>>,
    pub color: Color
}

impl<'a> Model3D<'a> {
    pub fn new_from_obj_file(position: Vector3, obj_path: &str, color: Color) -> Result<Model3D, ObjError> {
        let obj = Obj::load(obj_path)?;
        let mut triangles = vec![];

        let mut vtx_triangle = [Vector3::zero(), Vector3::zero(), Vector3::zero()];
        
        obj.data.position.windows(3).for_each(move |vtx| {
            for i in 0..3 {
                vtx_triangle[i].x = vtx[i][0];
                vtx_triangle[i].y = vtx[i][1];
                vtx_triangle[i].z = vtx[i][2];
            }

            triangles.push(Triangle::new(
                vtx_triangle[0].clone(),
                vtx_triangle[1].clone(),
                vtx_triangle[2].clone(),
                &color
            ))
        });

        return Ok(Model3D::new(position, triangles, color));
    }

    pub fn new(position: Vector3, triangles: Vec<Triangle<'a>>, color: Color) -> Self {
        let mut triangles = triangles.clone();
        Model3D::set_translated_triangles(&position, &mut triangles);

        return Model3D {
            position,
            triangles,
            color
        };
    }

    fn set_translated_triangles(position: &Vector3, triangles: &mut Vec<Triangle>) {
        for triangle in triangles {
            let mut new_vertices: [Vector3; 3] =
                [Vector3::zero(), Vector3::zero(), Vector3::zero()];

            for (v, vertex) in triangle.get_vertices().iter().enumerate() {
                new_vertices[v].x = vertex.x + position.x;
                new_vertices[v].y = vertex.y + position.y;
                new_vertices[v].z = vertex.z + position.z;
            }

            triangle.set_vertices(new_vertices);
        }
    }
}

impl<'a> Intersectable for Model3D<'a> {
    fn get_intersection(&self, ray: &super::ray::Ray) -> Option<Intersection> {
        self.triangles
            .iter()
            .fold(None, |acc: Option<Intersection>, triangle| {
                match triangle.get_intersection(&ray) {
                    Some(curr) => match &acc {
                        Some(prev) => {
                            if curr.distance < prev.distance {
                                return Some(curr.clone());
                            } else {
                                return acc;
                            }
                        }
                        None => return Some(curr.clone()),
                    },
                    None => return acc,
                }
            })
    }
}

impl<'a> fmt::Display for Model3D<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Model3D: {{\n  Triangles: [");
        for i in 0..self.triangles.len() {
            writeln!(f, "  {},", self.triangles[i]);
        }
        writeln!(f, "  ]\n}}");
        Ok(())
    }
}