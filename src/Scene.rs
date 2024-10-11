use image::{Rgb, RgbImage};

use crate::{
    objects::{camera::Camera, object3d::Object3D, ray::Ray},
    tools::{
        color_tools::ColorType,
        intersectable::{Intersectable, Intersection},
        vector3::Vector3,
    },
};

pub struct Scene<'a> {
    camera: Camera,
    objects: Vec<Object3D<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            objects: Vec::new(),
        }
    }

    pub fn set_objects(&mut self, objects: Vec<Object3D<'a>>) {
        self.objects = objects;
    }

    pub fn add_object(&mut self, object: Object3D<'a>) {
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

    pub fn raytrace(&self) -> Vec<Vec<ColorType>> {
        let pos_raytrace = self.camera.calculate_ray_positions();
        let mut pixel_buffer: Vec<Vec<ColorType>> =
            vec![vec![Rgb([0, 0, 0]); self.camera.width]; self.camera.height];

        println!("w: {}, h: {}", self.camera.width, self.camera.height);
        println!(
            "pixel buffer: {}x{}",
            pixel_buffer.len(),
            pixel_buffer[0].len()
        );

        for y in 0..self.camera.height {
            for x in 0..self.camera.width {
                let curr_x = pos_raytrace[y][x].x + self.camera.position.x;
                let curr_y = pos_raytrace[y][x].y + self.camera.position.y;
                let curr_z = pos_raytrace[y][x].z + self.camera.position.z;

                let ray = Ray::new(&self.camera.position, &Vector3::new(curr_x, curr_y, curr_z));

                match self.raycast(ray) {
                    Some(inter) => {
                        pixel_buffer[y][x] = {
                            match inter.object {
                                Object3D::Sphere(sphere) => sphere.color.rgb(),
                                Object3D::Plane(plane) => plane.color.rgb(),
                                Object3D::Triangle(triangle) => triangle.color.rgb(),
                            }
                        }
                    }
                    None => pixel_buffer[y][x] = image::Rgb([0, 0, 0]),
                }
            }
        }

        return pixel_buffer;
    }

    fn raycast(&self, ray: Ray) -> Option<Intersection> {
        return self
            .objects
            .iter()
            .filter_map(|s| s.get_intersection(&ray))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap());
    }
}
