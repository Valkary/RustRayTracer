use std::sync::{Arc, Mutex};
use std::thread;

use image::{Rgb, RgbImage};

use crate::{
    objects::{camera::Camera, object3d::Object3D, ray::Ray},
    tools::{
        intersectable::{Intersectable, Intersection},
        vector3::Vector3,
    },
};

pub struct Scene {
    camera: Camera,
    objects: Vec<Object3D>,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            objects: Vec::new(),
        }
    }

    pub fn set_objects(&mut self, objects: Vec<Object3D>) {
        self.objects = objects;
    }

    pub fn add_object(&mut self, object: Object3D) {
        self.objects.push(object);
    }

    pub fn generate_raytraced_image(&self) {
        let pixel_buffer = self.raytrace();
        let mut img = RgbImage::new(self.camera.width as u32, self.camera.height as u32);

        for y in 0..self.camera.height {
            for x in 0..self.camera.width {
                img.put_pixel(x as u32, y as u32, pixel_buffer[y][x]);
            }
        }

        match img.save("image.bmp") {
            Ok(_) => println!("Image correctly saved to image.bmp"),
            Err(e) => println!("{}", e.to_string()),
        }
    }

    pub fn raytrace(&self) -> Vec<Vec<Rgb<u8>>> {
        let n_threads = 16;
        let pos_raytrace = self.camera.calculate_ray_positions();
        let pixel_buffer: Vec<Vec<Rgb<u8>>> =
            vec![vec![Rgb([0, 0, 0]); self.camera.width]; self.camera.height];

        println!("w: {}, h: {}", self.camera.width, self.camera.height);
        println!(
            "pixel buffer: {}x{}",
            pixel_buffer.len(),
            pixel_buffer[0].len()
        );

        let step_x = self.camera.width / n_threads;
        let step_y = self.camera.height / n_threads;

        let pixel_buffer = Arc::new(Mutex::new(pixel_buffer));

        let mut handles = vec![];

        for y in 0..n_threads {
            for x in 0..n_threads {
                let pos_raytrace = pos_raytrace.clone();
                let pixel_buffer = Arc::clone(&pixel_buffer);
                let handle = thread::spawn(move || {
                    self.raytrace_section(
                        &pos_raytrace,
                        &pixel_buffer,
                        (x * step_x, (x + 1) * step_x),
                        (y * step_y, (y + 1) * step_y),
                    );
                });
                handles.push(handle);
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let pixel_buffer = Arc::try_unwrap(pixel_buffer).unwrap().into_inner().unwrap();
        pixel_buffer
    }

    fn raytrace_section(
        &self,
        pos_raytrace: &Vec<Vec<Vector3>>,
        pixel_buffer: &Arc<Mutex<Vec<Vec<Rgb<u8>>>>>,
        x_range: (usize, usize),
        y_range: (usize, usize),
    ) {
        for y in y_range.0..y_range.1 {
            for x in x_range.0..x_range.1 {
                if x < self.camera.width && y < self.camera.height {
                    let curr_x = pos_raytrace[y][x].x + self.camera.position.x;
                    let curr_y = pos_raytrace[y][x].y + self.camera.position.y;
                    let curr_z = pos_raytrace[y][x].z + self.camera.position.z;

                    let ray =
                        Ray::new(&self.camera.position, &Vector3::new(curr_x, curr_y, curr_z));

                    let color = match self.raycast(ray) {
                        Some(inter) => inter.color,
                        None => image::Rgb([0, 0, 0]),
                    };

                    let mut pixel_buffer = pixel_buffer.lock().unwrap();
                    pixel_buffer[y][x] = color;
                }
            }
        }
    }

    fn raycast(&self, ray: Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .fold(None, |acc: Option<Intersection>, object| {
                match object.get_intersection(&ray) {
                    Some(curr) => match &acc {
                        Some(prev) => {
                            if curr.distance < prev.distance {
                                Some(curr.clone())
                            } else {
                                acc
                            }
                        }
                        None => Some(curr.clone()),
                    },
                    None => acc,
                }
            })
    }
}
