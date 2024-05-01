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

    // public static BufferedImage raytrace(Scene scene) {
    //     Camera mainCamera = scene.getCamera();
    //     double[] nearFarPlanes = mainCamera.getNearFarPlanes();
    //     BufferedImage image = new BufferedImage(mainCamera.getResolutionWidth(), mainCamera.getResolutionHeight(), BufferedImage.TYPE_INT_RGB);
    //     List<Object3D> objects = scene.getObjects();
    //     Vector3D[][] posRaytrace = mainCamera.calculatePositionsToRay();
    //     Vector3D pos = mainCamera.getPosition();
    //     double cameraZ = pos.getZ();

    //     for (int i = 0; i < posRaytrace.length; i++) {
    //         for (int j = 0; j < posRaytrace[i].length; j++) {
    //             double x = posRaytrace[i][j].getX() + pos.getX();
    //             double y = posRaytrace[i][j].getY() + pos.getY();
    //             double z = posRaytrace[i][j].getZ() + pos.getZ();

    //             Ray ray = new Ray(mainCamera.getPosition(), new Vector3D(x, y, z));
    //             Intersection closestIntersection = raycast(ray, objects, null,
    //                     new double[]{cameraZ + nearFarPlanes[0], cameraZ + nearFarPlanes[1]});

    //             Color pixelColor = Color.BLACK;
    //             if (closestIntersection != null) {
    //                 //pixelColor = closestIntersection.getObject().getColor();
    //                 pixelColor = Light.getLightedColor(closestIntersection, scene.getLight());
    //             }
    //             image.setRGB(i, j, pixelColor.getRGB());
    //         }
    //     }

    //     return image;
    // }

    pub fn raytrace(&self) {
        let pos_raytrace = self.camera.calculate_ray_positions();

        for y in 0..self.camera.height {
            for x in 0..self.camera.width {
                let curr_x = pos_raytrace[y][x].x + self.camera.position.x;
                let curr_y = pos_raytrace[y][x].y + self.camera.position.y;
                let curr_z = pos_raytrace[y][x].z + self.camera.position.z;

                let ray = Ray::new(&self.camera.position, &Vector3::new(curr_x, curr_y, curr_z));
                
                match self.raycast(ray) {
                    Some(inter) => todo!(),
                    None => todo!(),
                }
            }
        }
    }

    fn raycast(&self, ray: Ray) -> Option<Intersection> {
        return self.objects
            .iter()
            .fold(None, |acc: Option<Intersection>, object| {
                match object.get_intersection(&ray) {
                    Some(curr) => match &acc {
                        Some(prev) => {
                            if curr.distance < prev.distance {
                                return Some(curr.clone())
                            } else {
                                return acc;
                            }
                        }
                        None => return Some(curr.clone()),
                    },
                    None => return acc,
                }
            })
    }
}
