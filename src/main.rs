mod Scene;
mod objects;
mod tools;

use crate::objects::{Camera::Camera, Sphere::Sphere};
use crate::tools::{Intersectable::Intersectable, Vector3::Vector3};

fn main() {
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

    let sphere = Sphere::new(
        Vector3::new(0.0, 0.0, 30.0),
        10.0,
        image::Rgb([0.0, 255.0, 0.0]),
    );

    println!("{}", sphere);

    let objects: Vec<Box<dyn Intersectable>> = vec![Box::new(sphere)];

    let mut scene: Scene::Scene = Scene::Scene::new(camera);
    scene.set_objects(objects);

    // let pixels = scene.raytrace();
    scene.generate_raytraced_image();
    // scene.generate_raytraced_image();
}
