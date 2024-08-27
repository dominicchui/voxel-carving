use nalgebra::{Vector3, Vector4};

use crate::{camera::Camera, voxel::{find_cube_intersect, Voxel, VoxelBlock}, RESOLUTION};

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

pub(crate) fn generate_ray(
    i: usize,
    j: usize,
    k: f32,
    camera: &Camera
) -> Ray {
    let dir = get_ray_dir_for_pixel(i, j, k, camera);
    // convert ray to world space
    let p = camera.pos.push(1.0);
    let d = camera.inv_view_matrix * dir;
    Ray { p, d }
}

/// For pixel[i,j], this computes and returns the direction from the camera to that pixel in camera space
fn get_ray_dir_for_pixel(
    i: usize,
    j: usize,
    k: f32,
    camera: &Camera,
) -> Vector4<f32> {
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
) -> Option<usize> {
    find_closest_intersection(ray, voxel_block)
}

/// Returns the t value for the closest intersection of the ray from position p and direction d to the objects in the scene
/// p and d are in world space
pub(crate) fn find_closest_intersection(
    ray: &Ray,
    voxel_block: &VoxelBlock,
) -> Option<usize> {
    // for each shape, find the intersection t's
    let mut t_values: Vec<f32> = vec![f32::MAX; voxel_block.voxels.len()];
    
    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            for z in 0..RESOLUTION {
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
                    // let intersect_pos = ray.p + ray.d * intersect;
                    // println!("intersect with voxel {index} at t {intersect}, pos {intersect_pos}");
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
        Some(min_index as usize)
    }
}



// pub(crate) fn compute_phong_lighting(
//     normal: Vector4<f32>,
//     lights: Vec<SceneLightData>,
//     kd: f32,
//     c_diffuse: Vector3<f32>
// ) -> Vector3<f32> {
//     // Normalize directions
//     let normal = normal.normalize();

//     let mut illumination: Vector3<f32> = Vector3::zeros();
    
//     // compute diffuse for each light
//     for light in lights {
//         illumination += compute_contribution_of_light(normal, &light, kd, c_diffuse);
//     }

//     illumination
// }

// /// Returns the color contribution of a particular light to the color of the given intersection point
// fn compute_contribution_of_light(
//     // position: Vector4<f32>,
//     normal: Vector4<f32>,
//     // incident_dir: Vector4<f32>,
//     // shape: &Shape,
//     light: &SceneLightData,
//     kd: f32,
//     // shapes: &Vec<Shape>,
//     c_diffuse: Vector3<f32>) -> Vector3<f32> {
//     let (att, k, intersect_to_light_d) = compute_light_properties(light);
//     let norm_dot_dir = f32::max(0.0, normal.dot(&intersect_to_light_d));

//     // diffuse term
//     let diffuse = kd * c_diffuse * norm_dot_dir;

//     // todo figure out clamp
//     // diff_spec_sum = nalgebra::clamp(diff_spec_sum, 0.0, 1.0);
//     att * (diffuse.component_mul(&light.color)) * k
// }


// fn compute_light_properties(
//     // position: &Vector4<f32>,
//     light: &SceneLightData,
// ) -> (f32, f32, Vector4<f32>) {
//     // intensity modifier:
//     // 1 for point and directional lights
//     // depends on falloff function for spot lights
//     let mut k: f32 = 1.0;
//     let mut intersect_to_light_d: Vector4<f32>;
//     // let distance;

//     // only point and spot lights attenuate
//     let mut att = 1.0;
//     match light.light_type {
//         LightType::PointLight => {
//             // distance = glm::distance(&position, &light.pos.unwrap());
//             // att = f32::min(
//             //     1.0,
//             //     1.0 / (light.att_function.unwrap()[0]
//             //         + distance * light.att_function.unwrap()[1]
//             //         + distance * distance * light.att_function.unwrap()[2]),
//             // );
//             // intersect_to_light_d = light.pos.unwrap() - position;
//             todo!();
//         }
//         LightType::DirectionalLight => intersect_to_light_d = -light.dir.unwrap(),
//         LightType::SpotLight => {
//             todo!();
//             // distance = glm::distance(&position, &light.pos.unwrap());
//             // att = f32::min(
//             //     1.0,
//             //     1.0 / (light.att_function.unwrap()[0]
//             //         + distance * light.att_function.unwrap()[1]
//             //         + distance * distance * light.att_function.unwrap()[2]),
//             // );
//             // intersect_to_light_d = light.pos.unwrap() - position;
//             // k = compute_angular_falloff(
//             //     light.dir.unwrap(),
//             //     intersect_to_light_d,
//             //     light.angle.unwrap() - light.penumbra.unwrap(),
//             //     light.angle.unwrap(),
//             // );
//         }
//     }
//     intersect_to_light_d = (&intersect_to_light_d).normalize();
//     (att, k, intersect_to_light_d)
// }


// #[cfg(test)]
// mod tests {
//     use nalgebra::{Vector3, Vector4};

//     use super::{compute_contribution_of_light, SceneLightData};

//     #[test]
//     fn test_compute_contribution_of_light() {
//         let light = SceneLightData {
//             light_type: super::LightType::DirectionalLight,
//             color: Vector3::new(1.0, 1.0, 1.0),
//             dir: Some(Vector4::new(-3.0, -2.0, -1.0, 0.0)),
//             pos: None
//         };
//         let kd = 0.5;
//         let c_diffuse = Vector3::new(1.0, 0.0, 0.0);
//         let normal = Vector4::new(1.0, 0.0, 0.0, 0.0);
//         let color = compute_contribution_of_light(normal, &light, kd, c_diffuse);
//         println!("color1: {}", color);
        
//         let normal = Vector4::new(0.0, 1.0, 0.0, 0.0);
//         let color = compute_contribution_of_light(normal, &light, kd, c_diffuse);
//         println!("color2: {}", color);
//     }

// }