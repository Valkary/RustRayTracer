use image::Rgb;

use crate::{
    objects::{Camera::Camera, Ray::Ray},
    tools::{
        Intersectable::{Intersectable, Intersection},
        Vector3::Vector3,
    },
};

pub struct Scene<'a> {
    camera: Camera,
    objects: Vec<Box<dyn Intersectable + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            objects: Vec::new(),
        }
    }

    pub fn set_objects(&mut self, objects: Vec<Box<dyn Intersectable + 'a>>) {
        self.objects = objects;
    }

    pub fn add_object(&mut self, object: Box<dyn Intersectable + 'a>) {
        self.objects.push(object);
    }

    pub fn raytrace(&self) -> Vec<Vec<Rgb<f32>>> {
        let pos_raytrace = self.camera.calculate_ray_positions();
        let mut pixel_buffer: Vec<Vec<Rgb<f32>>> = Vec::with_capacity(self.camera.height);

        for y in 0..self.camera.height {
            pixel_buffer.push(Vec::with_capacity(self.camera.width));
            for x in 0..self.camera.width {
                let curr_x = pos_raytrace[y][x].x + self.camera.position.x;
                let curr_y = pos_raytrace[y][x].y + self.camera.position.y;
                let curr_z = pos_raytrace[y][x].z + self.camera.position.z;

                let ray = Ray::new(&self.camera.position, &Vector3::new(curr_x, curr_y, curr_z));

                match self.raycast(ray) {
                    Some(inter) => pixel_buffer[y][x] = inter.color,
                    None => pixel_buffer[y][x] = image::Rgb([0.0, 0.0, 0.0]),
                }
            }
        }

        return pixel_buffer;
    }

    fn raycast(&self, ray: Ray) -> Option<Intersection> {
        return self
            .objects
            .iter()
            .fold(None, |acc: Option<Intersection>, object| {
                match object.get_intersection(&ray) {
                    Some(curr) => match &acc {
                        Some(prev) => {
                            if curr.distance < prev.distance {
                                return Some(curr.clone());
                            } else {
                                return acc;
                            }
                        }
                        None => return Some(curr.clone()),
                    },
                    None => return acc,
                }
            });
    }
}
