use image::open;
use nalgebra::Vector3;

use crate::camera::Camera;

#[derive(Clone)]
pub(crate) struct Image {
    // rgb data from image
    pub(crate) data: Vec<u8>,
    pub(crate) marked: Vec<bool>,
    pub(crate) camera: Camera,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Image {
    pub(crate) fn new_from_file(
        file_path: String,
        pos: Vector3<f32>,
        up: Vector3<f32>,
        focus: Vector3<f32>,
        height_angle: f32,
        width: usize,
        height: usize,
    ) -> Self {
        // read from file
        let image = open(file_path).unwrap().into_rgb8().into_vec();
        let marked = vec![false; image.len()/3];

        let look = focus - pos;
        let near = 0.01;
        let far = 1000.0;
        let camera = Camera::new(width, height, pos, focus, look, up, height_angle, near, far);
        Image {
            data: image,
            marked,
            camera,
            width,
            height,
        }
    }
}
