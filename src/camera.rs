use nalgebra::{Matrix4, Vector3};

pub(crate) struct Camera {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) pos: Vector3<f32>,
    pub(crate) look: Vector3<f32>,
    pub(crate) up: Vector3<f32>,
    pub(crate) height_angle: f32,
    pub(crate) view_matrix: Option<Matrix4<f32>>,
    pub(crate) inv_view_matrix: Option<Matrix4<f32>>,
    pub(crate) proj_matrix: Option<Matrix4<f32>>,
    near: f32,
    far: f32
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
        Camera {
            width,
            height,
            pos,
            look,
            up,
            height_angle,
            view_matrix: None,
            inv_view_matrix: None,
            proj_matrix: None,
            near,
            far
        }
    }

    pub fn get_view_matrix(&mut self) -> Matrix4<f32> {
        match self.view_matrix {
            Some(mat) => mat,
            None => {
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
                let mat = mrot * mtrans;

                // store the matrix
                self.view_matrix = Some(mat);
                mat
            }
        }
        
    }

    pub fn get_inverse_view_matrix(&mut self) -> Matrix4<f32> {
        match self.inv_view_matrix {
            Some(mat) => mat,
            None => {
                let view = self.get_view_matrix();
                let inv_view = view.try_inverse().unwrap();
                // store the matrix
                self.inv_view_matrix = Some(inv_view);
                inv_view
            }
        }
    }

    pub fn get_proj_matrix(&mut self) -> Matrix4<f32> {
        match self.proj_matrix {
            Some(mat) => mat,
            None => {
                // let theta = self.fov_y * 0.5;
                // let inv_range = 1.0 / (self.far - self.near);
                // let inv_tan = 1.0 / f32::tan(theta);
                // let mut proj = Matrix4::identity();
                // proj[0] = inv_tan / self.aspect;
                // proj[5] = inv_tan;
                // proj[10] = -(self.near + self.far) * inv_range;
                // proj[11] = -1.0;
                // proj[14] = -2.0 * self.near * self.far * inv_range;
                // proj[15] = 0.0;

                // // store the matrix
                // self.proj_matrix = Some(proj);
                // proj

                // float heightAngle = getHeightAngle();
                // float widthAngle = atan(getAspectRatio() * tan(heightAngle/2))*2;
                // glm::mat4 mScaling = glm::mat4(1/(settings.farPlane * tan(widthAngle/2)), 0, 0, 0,
                //                             0, 1/(settings.farPlane * tan(heightAngle/2)), 0, 0,
                //                             0, 0, 1/settings.farPlane, 0,
                //                             0, 0, 0, 1);

                // float c = -settings.nearPlane / settings.farPlane;
                // glm::mat4 mUnhinging = glm::mat4(1, 0, 0, 0,
                //                                 0, 1, 0, 0,
                //                                 0, 0, 1/(1+c), -1,
                //                                 0, 0, -c/(1+c), 0);
                // glm::mat4 mRemapping = glm::mat4(1, 0, 0, 0,
                //                             0, 1, 0, 0,
                //                             0, 0, -2, 0,
                //                             0, 0, -1, 1);
                // return mRemapping * mUnhinging * mScaling;
                let width_angle = f32::atan(self.get_aspect_ratio() * f32::tan(self.height_angle / 2.0)) * 2.0;
                let scaling = Matrix4::new(1.0 / (self.far * f32::tan(width_angle / 2.0)), 0.0, 0.0, 0.0, 
                    0.0, 1.0 / (self.far * f32::tan(self.height_angle / 2.0)), 0.0, 0.0, 
                    0.0, 0.0, 1.0 / self.far, 0.0, 
                    0.0, 0.0, 0.0, 1.0);
                
                let c = -self.near / self.far;
                let mut unhinging = Matrix4::identity();
                unhinging[10] = 1.0 / (1.0 + c);
                unhinging[11] = -1.0;
                unhinging[14] = -c / (1.0 + c);
                unhinging[15] = 0.0;

                let mut remapping = Matrix4::identity();
                remapping[10] = -2.0;
                remapping[14] = -1.0;

                let proj = remapping * unhinging * scaling;
                self.proj_matrix = Some(proj);
                proj
            }
        }
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}
