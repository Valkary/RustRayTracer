use crate::objects::{object3d::Object3D, ray::Ray};

#[derive(Clone, Debug)]
pub struct Intersection<'a> {
    pub distance: f32,
    pub object: Object3D<'a>
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f32, object: Object3D<'a>) -> Intersection<'a> {
        return Intersection {
            distance,
            object
        }
    }
}

pub trait Intersectable {
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection>;   
}
