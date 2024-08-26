use nalgebra::{Matrix4, Vector3};

#[derive(Clone, Copy)]
pub(crate) struct Camera {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) pos: Vector3<f32>,
    pub(crate) look: Vector3<f32>,
    pub(crate) up: Vector3<f32>,
    pub(crate) height_angle: f32,
    pub(crate) view_matrix: Matrix4<f32>,
    pub(crate) inv_view_matrix: Option<Matrix4<f32>>,
    pub(crate) proj_matrix: Matrix4<f32>,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(
        width: usize,
        height: usize,
        pos: Vector3<f32>,
        look: Vector3<f32>,
        up: Vector3<f32>,
        height_angle: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let mtrans = Matrix4::new(
            1.0,
            0.0,
            0.0,
            -pos.x,
            0.0,
            1.0,
            0.0,
            -pos.y,
            0.0,
            0.0,
            1.0,
            -pos.z,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        let w = -look.clone().normalize();
        let v = (up - up.dot(&w) * w).normalize();
        let u = v.cross(&w);

        let mrot = Matrix4::new(
            u.x, u.y, u.z, 0.0, v.x, v.y, v.z, 0.0, w.x, w.y, w.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let view_matrix = mrot * mtrans;

        // projection matrix
        let width_angle =
            f32::atan((width as f32 / height as f32) * f32::tan(height_angle / 2.0)) * 2.0;
        let scaling = Matrix4::new(
            1.0 / (far * f32::tan(width_angle / 2.0)),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / (far * f32::tan(height_angle / 2.0)),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / far,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let c = -near / far;
        let mut unhinging = Matrix4::identity();
        unhinging[10] = 1.0 / (1.0 + c);
        unhinging[11] = -1.0;
        unhinging[14] = -c / (1.0 + c);
        unhinging[15] = 0.0;

        let mut remapping = Matrix4::identity();
        remapping[10] = -2.0;
        remapping[14] = -1.0;

        let proj_matrix = remapping * unhinging * scaling;

        Camera {
            width,
            height,
            pos,
            look,
            up,
            height_angle,
            view_matrix,
            inv_view_matrix: None,
            proj_matrix,
            near,
            far,
        }
    }

    // pub fn set_view_matrix(&mut self) {
    //     let mtrans = Matrix4::new(
    //         1.0,
    //         0.0,
    //         0.0,
    //         -self.pos.x,
    //         0.0,
    //         1.0,
    //         0.0,
    //         -self.pos.y,
    //         0.0,
    //         0.0,
    //         1.0,
    //         -self.pos.z,
    //         0.0,
    //         0.0,
    //         0.0,
    //         1.0,
    //     );
    //     let w = -self.look.clone().normalize();
    //     let v = (self.up - self.up.dot(&w) * w).normalize();
    //     let u = v.cross(&w);

    //     let mrot = Matrix4::new(
    //         u.x, u.y, u.z, 0.0, v.x, v.y, v.z, 0.0, w.x, w.y, w.z, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     );
    //     let mat = mrot * mtrans;

    //     // store the matrix
    //     self.view_matrix = Some(mat);
    // }

    // pub fn get_view_matrix(&mut self) -> Option<Matrix4<f32>> {
    //     self.view_matrix
    // }

    // pub fn get_inverse_view_matrix(&mut self) -> Matrix4<f32> {
    //     match self.inv_view_matrix {
    //         Some(mat) => mat,
    //         None => {
    //             let view = self.get_view_matrix();
    //             let inv_view = view.try_inverse().unwrap();
    //             // store the matrix
    //             self.inv_view_matrix = Some(inv_view);
    //             inv_view
    //         }
    //     }
    // }

    // pub fn set_proj_matrix(&mut self) {
    //     let width_angle =
    //         f32::atan(self.get_aspect_ratio() * f32::tan(self.height_angle / 2.0)) * 2.0;
    //     let scaling = Matrix4::new(
    //         1.0 / (self.far * f32::tan(width_angle / 2.0)),
    //         0.0,
    //         0.0,
    //         0.0,
    //         0.0,
    //         1.0 / (self.far * f32::tan(self.height_angle / 2.0)),
    //         0.0,
    //         0.0,
    //         0.0,
    //         0.0,
    //         1.0 / self.far,
    //         0.0,
    //         0.0,
    //         0.0,
    //         0.0,
    //         1.0,
    //     );

    //     let c = -self.near / self.far;
    //     let mut unhinging = Matrix4::identity();
    //     unhinging[10] = 1.0 / (1.0 + c);
    //     unhinging[11] = -1.0;
    //     unhinging[14] = -c / (1.0 + c);
    //     unhinging[15] = 0.0;

    //     let mut remapping = Matrix4::identity();
    //     remapping[10] = -2.0;
    //     remapping[14] = -1.0;

    //     let proj = remapping * unhinging * scaling;
    //     self.proj_matrix = Some(proj);
    // }

    // pub fn get_proj_matrix(&mut self) -> Option<Matrix4<f32>> {
    //     self.proj_matrix
    // }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}
