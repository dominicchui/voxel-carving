use nalgebra::{Vector3, Vector4};

use crate::{
    camera::Camera,
    voxel::{find_cube_intersect, Voxel, VoxelBlock},
    RESOLUTION,
};

pub(crate) enum LightType {
    PointLight,
    DirectionalLight,
    SpotLight,
}

pub(crate) struct SceneLightData {
    pub(crate) light_type: LightType,
    pub(crate) color: Vector3<f32>,
    // pub(crate) att_function: Vector3<f32>,
    pub(crate) dir: Option<Vector4<f32>>,
    pub(crate) pos: Option<Vector4<f32>>,
}

pub(crate) struct Ray {
    pub(crate) p: Vector4<f32>,
    pub(crate) d: Vector4<f32>,
}

pub(crate) fn generate_ray(i: usize, j: usize, k: f32, camera: &Camera) -> Ray {
    let dir = get_ray_dir_for_pixel(i, j, k, camera);
    // convert ray to world space
    let p = camera.pos.push(1.0);
    let d = (camera.inv_view_matrix * dir).normalize();
    Ray { p, d }
}

pub(crate) fn generate_ray_direct(x: f32, y: f32, z: f32, camera: &Camera) -> Ray {
    // let p = Vector4::new(x,y,z,1.0);
    let p = camera.pos.push(1.0);
    let d = (Vector4::new(x, y, z, 1.0) - p).normalize();
    Ray { p, d }
}

/// For pixel[i,j], this computes and returns the direction from the camera to that pixel in camera space
fn get_ray_dir_for_pixel(i: usize, j: usize, k: f32, camera: &Camera) -> Vector4<f32> {
    let x = (i as f32 + 0.5) / camera.width as f32 - 0.5;
    let y = (j as f32 + 0.5) / camera.height as f32 - 0.5;
    let aspect_ratio = camera.get_aspect_ratio();
    let theta_h = camera.height_angle;
    let big_v = 2.0 * k * (theta_h / 2.0).tan();
    let big_u = big_v * aspect_ratio;
    let uvk = Vector4::new(big_u * x, -big_v * y, -k, 1.0);

    // a bit unnecessary but demonstrative as the the direction is in camera space
    let eye = Vector4::new(0.0, 0.0, 0.0, 1.0);
    uvk - eye
}

/// For a given ray position and direction, trace it through the scene to closest intersect to determine color
/// Returns the first voxel hit (by index?)
pub(crate) fn trace_ray(
    ray: &Ray,
    voxel_block: &VoxelBlock,
    expected_index: usize,
) -> Option<usize> {
    find_closest_intersection(ray, voxel_block, expected_index)
}

/// Returns the t value for the closest intersection of the ray from position p and direction d to the objects in the scene
/// p and d are in world space
pub(crate) fn find_closest_intersection(
    ray: &Ray,
    voxel_block: &VoxelBlock,
    expected_index: usize,
) -> Option<usize> {
    // for each shape, find the intersection t's
    let mut t_values: Vec<f32> = vec![f32::MAX; voxel_block.voxels.len()];

    for x in 0..voxel_block.resolution {
        for y in 0..voxel_block.resolution {
            for z in 0..voxel_block.resolution {
                // convert coordinates to object space
                let index = x
                    + y * voxel_block.resolution
                    + z * voxel_block.resolution * voxel_block.resolution;

                // skip not visible voxels
                let voxel = &voxel_block.voxels[index];
                if !voxel.visible {
                    continue;
                }

                // find intersect
                if let Some(intersect) = find_cube_intersect(voxel, ray.p, ray.d) {
                    t_values[index] = intersect;
                    let intersect_pos = ray.p + ray.d * intersect;
                    if expected_index == 6 {
                        println!("d {}", ray.d);
                        println!(
                            "intersect with voxel {index} at t {intersect}, pos {intersect_pos}"
                        );
                    }
                }
            }
        }
    }

    // determine object with smallest t
    let mut min_t = f32::MAX;
    let mut min_index: i32 = -1;
    for i in 0..t_values.len() {
        if t_values[i] > 0.0 && t_values[i] < min_t {
            min_t = t_values[i];
            min_index = i as i32;
        }
    }
    if min_index == -1 {
        None
    } else {
        let expected_t = t_values[expected_index];
        if expected_t <= min_t + 0.001 {
            // println!("good enough");
            Some(expected_index)
        } else {
            Some(min_index as usize)
        }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::{Perspective3, Point3, Vector3, Vector4};

    use crate::{
        raytracer::generate_ray_direct,
        scene_generator,
        voxel::{find_cube_intersect, VoxelBlock},
    };

    use super::{generate_ray, trace_ray};

    #[test]
    fn test_trace_ray() {
        let images = &mut scene_generator::cone();
        let mut voxel_block = VoxelBlock::new(2, 2);
        // voxel_block.carve(1,1,1);
        // voxel_block.carve(0,1,1);

        let ray = generate_ray(569, 417, 1.0, &images[0].camera);
        let dir = ray.d;
        let pos = ray.p;
        println!("ray pos {pos}, ray dir {dir}");

        let ray_direct = generate_ray_direct(-0.5, -0.5, 0.5, &images[0].camera);
        let dir = ray_direct.d;
        let pos = ray_direct.p;
        println!("ray_direct pos {pos}, ray dir {dir}");

        let voxel = &voxel_block.voxels[4];
        // println!("center {}", voxel.ctm * Vector4::new(0.0,0.0,0.0, 1.0));
        // println!("corner {}", voxel.ctm * Vector4::new(-0.5, -0.5, -0.5, 1.0));
        // println!("camera {}", voxel.ctm * Vector4::new(3.5, 3.5, 2.5, 1.0));
        assert_eq!(
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            voxel.inverse_ctm * Vector4::new(-0.5, -0.5, 0.5, 1.0)
        );
        assert_eq!(
            Vector4::new(3.0, 3.0, 3.0, 1.0),
            voxel.inverse_ctm * Vector4::new(2.5, 2.5, 3.5, 1.0)
        );
        let intersect = find_cube_intersect(voxel, pos, dir);
        assert!(intersect.is_some());

        let intersected_voxel = trace_ray(&ray_direct, &voxel_block, 4);
        assert!(intersected_voxel.is_some());
        assert_eq!(intersected_voxel.unwrap(), 4);

        // the issue:
        // the voxel projects onto the image at (i,j)
        // when a ray is traced from (i,j) into the scene, it does not intersect with the voxel ?!
    }
}
