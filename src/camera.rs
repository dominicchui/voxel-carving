use nalgebra::{Matrix4, Vector3};

pub(crate) struct Camera {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pos: Vector3<f32>,
    pub(crate) look: Vector3<f32>,
    pub(crate) up: Vector3<f32>,
    pub(crate) height_angle: f32,
}

impl Camera {
    pub fn new(
        width: u32,
        height: u32,
        pos: Vector3<f32>,
        look: Vector3<f32>,
        up: Vector3<f32>,
        height_angle: f32,
    ) -> Self {
        Camera {
            width,
            height,
            pos,
            look,
            up,
            height_angle,
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        let mtrans = Matrix4::new(
            1.0,
            0.0,
            0.0,
            -self.pos.x,
            0.0,
            1.0,
            0.0,
            -self.pos.y,
            0.0,
            0.0,
            1.0,
            -self.pos.z,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        let w = -self.look.clone().normalize();
        let v = (self.up - self.up.dot(&w) * w).normalize();
        let u = v.cross(&w);

        let mrot = Matrix4::new(
            u.x, u.y, u.z, 0.0, v.x, v.y, v.z, 0.0, w.x, w.y, w.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        mrot * mtrans
    }

    pub fn get_inverse_view_matrix(&self) -> Matrix4<f32> {
        self.get_view_matrix().try_inverse().unwrap()
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}
