use crate::tools::{color_tools::ColorType, vector3::Vector3};

pub struct Light {
    pub direction: Vector3,
    pub color: ColorType,
    pub intensity: f32,
}