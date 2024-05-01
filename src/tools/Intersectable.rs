use image::Rgb;

use crate::objects::Ray::Ray;
use super::Vector3::Vector3;

pub struct Intersection {
    pub distance: f64,
    pub position: Vector3,
    pub normal: Vector3,
    pub color: Rgb<f32>
}

pub trait Intersectable {
    fn get_intersection(&self, ray: Ray) -> Option<Intersection>;
    // fn get_intersection<'a, T: Intersectable>(&self, ray: Ray) -> Option<Intersection<'a, T>>;
}