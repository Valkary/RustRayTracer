use std::time::SystemTime;

use objects::Model::Model3D;

mod Scene;
mod objects;
mod tools;

use crate::objects::{Camera::Camera, Sphere::Sphere, Triangle::Triangle};
use crate::tools::{Intersectable::Intersectable, Vector3::Vector3};

fn main() {
    let now = SystemTime::now();

    let camera = Camera {
        position: Vector3::new(0.0, 0.0, -4.0),
        fov_h: 60.0,
        fov_v: 60.0,
        width: 800,
        height: 800,
        near_plane: 0.6,
        fal_plane: 50.0,
        default_z: 15.0,
    };

    let sphere01 = Sphere::new(Vector3::new(0.0, 0.0, 10.0), 5.0, image::Rgb([255, 0, 0]));
    let sphere02 = Sphere::new(Vector3::new(3.0, -2.0, 8.0), 2.0, image::Rgb([0, 255, 0]));
    let sphere03 = Sphere::new(Vector3::new(-9.0, 3.0, 2.0), 1.5, image::Rgb([0, 255, 255]));

    let triangle01 = Triangle::new(
        Vector3::new(0.0, 0.0, 20.0),
        Vector3::new(0.0, 15.0, 20.0),
        Vector3::new(15.0, 15.0, 20.0),
        image::Rgb([255, 255, 0]),
    );
    let triangle02 = Triangle::new(
        Vector3::new(0.0, 0.0, 25.0),
        Vector3::new(15.0, 0.0, 25.0),
        Vector3::new(15.0, 15.0, 25.0),
        image::Rgb([255, 0, 0]),
    );

    let model01 = Model3D::new(
        Vector3::new(10.0, 0.0, 0.0), 
        vec![triangle01, triangle02]
    );

    let model02 = Model3D::new_from_obj_file(Vector3::zero(), "SmallTeapot.obj").unwrap();

    let objects: Vec<Box<dyn Intersectable>> = vec![
        // Box::new(sphere01),
        // Box::new(sphere02),
        // Box::new(sphere03),
        // Box::new(model01),
        Box::new(model02)
    ];

    let mut scene: Scene::Scene = Scene::Scene::new(camera);
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