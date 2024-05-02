use image::{Rgb, RgbImage};

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
        let pos_raytrace = self.camera.calculate_ray_positions();
        let mut pixel_buffer: Vec<Vec<Rgb<u8>>> = vec![vec![Rgb([0,0,0]); self.camera.width]; self.camera.height];

        println!("w: {}, h: {}", self.camera.width, self.camera.height);
        println!("pixel buffer: {}x{}", pixel_buffer.len(), pixel_buffer[0].len());

        for y in 0..self.camera.height {
            for x in 0..self.camera.width {
                let curr_x = pos_raytrace[y][x].x + self.camera.position.x;
                let curr_y = pos_raytrace[y][x].y + self.camera.position.y;
                let curr_z = pos_raytrace[y][x].z + self.camera.position.z;

                let ray = Ray::new(&self.camera.position, &Vector3::new(curr_x, curr_y, curr_z));

                match self.raycast(ray) {
                    Some(inter) => pixel_buffer[y][x] = inter.color,
                    None => pixel_buffer[y][x] = image::Rgb([0,0,0]),
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
