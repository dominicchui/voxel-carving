use image::open;
use nalgebra::Vector3;

use crate::camera::Camera;

pub(crate) struct Image {
    // rgb data from image
    pub(crate) data: Vec<u8>,
    pub(crate) camera: Camera,
}

impl Image {
    pub(crate) fn new_from_file(file_path: String, pos: Vector3<f32>, up: Vector3<f32>, focus: Vector3<f32>, height_angle: f32,) -> Self {
        // read from file
        let image = open(file_path).unwrap().into_rgb8().into_vec();
        
        let width = 500;
        let height = 300;
        let look = focus - pos;
        let camera = Camera::new(width, height, pos, look, up, height_angle);
        Image {
            data: image,
            camera,
        }
    }
}