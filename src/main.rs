use std::time::SystemTime;

use objects::object3d::Object3D;
use objects::model::Model3D;

mod scene;
mod objects;
mod tools;
mod Scene;

use crate::objects::{camera::Camera, sphere::Sphere, triangle::Triangle};
use crate::tools::vector3::Vector3;

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

    let objects: Vec<Object3D> = vec![
        Object3D::Sphere(sphere01),
        Object3D::Sphere(sphere02),
        Object3D::Sphere(sphere03),
        Object3D::Model(model01),
        Object3D::Model(model02)
    ];

    let mut scene: scene::Scene = scene::Scene::new(camera);
    scene.set_objects(objects);
    scene.generate_raytraced_image();

    match now.elapsed() {
        Ok(elapsed) => {
            println!("Duration: {} seconds", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
    
}