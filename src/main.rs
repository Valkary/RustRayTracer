use std::time::SystemTime;

use objects::object3d::Object3D;
use objects::plane::Plane;
use tools::color_tools::Color;

mod objects;
mod scene;
mod tools;

use crate::objects::{camera::Camera, sphere::Sphere};
use crate::tools::vector3::Vector3;

fn main() {
    let now = SystemTime::now();

    let camera = Camera {
        position: Vector3::new(0.0, 0.0, -4.0),
        fov_h: 60.0,
        fov_v: 60.0,
        width: 400,
        height: 400,
        near_plane: 0.6,
        fal_plane: 50.0,
        default_z: 15.0,
    };

    let sphere01 = Sphere::new(Vector3::new(0.0, 0.0, 10.0), 5.0, Color::DarkRed);
    let sphere02 = Sphere::new(Vector3::new(3.0, -2.0, 8.0), 2.0, Color::DarkGreen);
    let sphere03 = Sphere::new(Vector3::new(-6.0, 3.0, 2.0), 1.5, Color::DarkOrange);
    let plane01 = Plane::new(
        Vector3::new(0, 5, -4),
        Vector3::new(0, 1, 0),
        Color::DarkGray,
    );
    let plane02 = Plane::new(
        Vector3::new(0, 0, 10),
        Vector3::new(0, 0, -1),
        Color::LightBlue,
    );

    let objects: Vec<Object3D> = vec![
        Object3D::Sphere(&sphere01),
        Object3D::Sphere(&sphere02),
        Object3D::Sphere(&sphere03),
        Object3D::Plane(&plane01),
        Object3D::Plane(&plane02),
    ];

    let mut scene: scene::Scene = scene::Scene::new(camera);
    scene.set_objects(objects);
    scene.generate_raytraced_image();

    match now.elapsed() {
        Ok(elapsed) => {
            println!("Duration: {} millis", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
}
