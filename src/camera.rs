use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};

#[derive(Clone, Copy)]
pub(crate) struct Camera {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) pos: Vector3<f32>,
    pub(crate) look: Vector3<f32>,
    pub(crate) up: Vector3<f32>,
    pub(crate) height_angle: f32,
    pub(crate) view_matrix: Matrix4<f32>,
    pub(crate) inv_view_matrix: Matrix4<f32>,
    pub(crate) proj_matrix: Matrix4<f32>,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(
        width: usize,
        height: usize,
        pos: Vector3<f32>,
        target: Vector3<f32>,
        look: Vector3<f32>,
        up: Vector3<f32>,
        height_angle: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let eye = Point3::from(pos);
        let target = Point3::from(target);
        let view_matrix   = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
        let inv_view_matrix = view_matrix.inverse();

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
        println!("proj_matrix {proj_matrix}");

        Camera {
            width,
            height,
            pos,
            look,
            up,
            height_angle,
            view_matrix: view_matrix.to_homogeneous(),
            inv_view_matrix : inv_view_matrix.to_homogeneous(),
            proj_matrix,
            near,
            far,
        }
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}
