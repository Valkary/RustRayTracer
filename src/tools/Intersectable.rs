use image::Rgb;

use super::vector3::Vector3;
use crate::objects::ray::Ray;

#[derive(Clone, Debug)]
pub struct Intersection {
    pub distance: f32,
    pub position: Vector3,
    pub normal: Vector3,
    pub color: Rgb<u8>,
}

pub trait Intersectable {
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection>;   
}
