use crate::tools::vector3::Vector3;

#[derive(Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: &Vector3, direction: &Vector3) -> Self {
        return Ray {
            origin: origin.clone(),
            direction: Vector3::normalize(&direction),
        };
    }

    pub fn get_direction(&self) -> Vector3 {
        return self.direction.clone();
    }

    pub fn at(&self, t: f32) -> Vector3 {
        return Vector3::add(&self.origin, &Vector3::scalar_multiplication(&self.direction, t));
    }
}
