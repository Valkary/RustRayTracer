use crate::tools::{color_tools::Color, vector3::Vector3};

pub struct Light {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}
