use crate::tools::Vector3::Vector3;

pub struct Camera {
    pub fov_h: f32,
    pub fov_v: f32,
    pub default_z: f32,
    pub width: usize,
    pub height: usize,
    pub near_plane: f32,
    pub fal_plane: f32,
    pub position: Vector3,
}

impl Camera {
    pub fn calculate_ray_positions(&self) -> Vec<Vec<Vector3>> {
        let mut positions: Vec<Vec<Vector3>> = vec![vec![Vector3::zero(); self.width]; self.height];
        let angle_max_x = self.fov_h / 2.0;
        let radius_max_x = self.default_z / angle_max_x.to_radians().cos();

        let max_x = angle_max_x.to_radians().cos() * radius_max_x;
        let min_x = -max_x;

        let angle_max_y = self.fov_v / 2.0;
        let radius_max_y = self.default_z / angle_max_y.to_radians().sin();

        let max_y = angle_max_y.to_radians().sin() * radius_max_y;
        let min_y = -max_y;

        let step_x = (max_x - min_x) / self.width as f32;
        let step_y = (max_y - min_y) / self.height as f32;

        for y in 0..positions.len() {
            for x in 0..self.width {
                let pos_x = min_x + (step_x * x as f32);
                let pos_y = min_y + (step_y * y as f32);

                positions[y][x] = Vector3::new(pos_x, pos_y, self.default_z);
            }
        }

        return positions;
    }
}
