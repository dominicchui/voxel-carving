use nalgebra::{Vector3, Vector4};

use crate::{
    image::Image,
    voxel::VoxelBlock,
};

/// given voxelblock and image, for each voxel, project ray to each camera and get pixel and color
/// ray trace to see if the colors are consistent
/// if not, then carve away
pub(crate) fn carve(voxel_block: &mut VoxelBlock, images: &mut Vec<Image>) {
    // should_carve(0.0, 0.0, 0.0, image, width, height);
    // carve in each of the 6 directions

    // positive x direction
    for x in 0..voxel_block.resolution {
        for y in 0..voxel_block.resolution {
            for z in 0..voxel_block.resolution {
                // convert coordinates to object space
                let index = x
                    + y * voxel_block.resolution
                    + z * voxel_block.resolution * voxel_block.resolution;
                let (x_f, y_f, z_f) = voxel_block.index_to_coordinate(index);

                if should_carve(x_f, y_f, z_f, images) {
                    voxel_block.carve(x, y, z);
                }
            }
        }
    }

    // // negative x direction
    // for x in voxel_block.resolution..0 {
    //     for y in 0..voxel_block.resolution {
    //         for z in 0..voxel_block.resolution {
    //             // convert coordinates to object space
    //             let index = x + y * voxel_block.resolution + z * voxel_block.resolution * voxel_block.resolution;
    //             let (x_f, y_f, z_f) = voxel_block.index_to_coordinate(index);

    //             if should_carve(x_f, y_f, z_f, image, width, height) {
    //                 voxel_block.carve(x, y, z);
    //             }
    //         }
    //     }
    // }

    // positive y direction

    // negative y direction

    // positive z direction

    // negative z direction
}

fn should_carve(x: f32, y: f32, z: f32, images: &mut Vec<Image>) -> bool {
    for image in images {
        if let Some(consistent) = is_consistent_with_image(x, y, z, image) {
            if !consistent {
                return true;
            }
        }
    }
    false
}

fn is_consistent_with_image(x: f32, y: f32, z: f32, image: &mut Image) -> Option<bool> {
    get_projected_pixel_color(x, y, z, image).map(|(r, g, b)| r != 0 || g != 0 || b != 0)
}

fn get_projected_pixel_color(x: f32, y: f32, z: f32, image: &mut Image) -> Option<(u8, u8, u8)> {
    // convert from world space to projected/clip space
    let proj_coord = image.camera.get_proj_matrix()
        * image.camera.get_view_matrix()
        * Vector4::new(x, y, z, 1.0);

    // normalize
    let normed_coord = Vector3::new(
        proj_coord[0] / proj_coord[3],
        proj_coord[1] / proj_coord[3],
        proj_coord[2] / proj_coord[3],
    );

    // clip space goes from (-1,-1,0) to (1,1,1)
    // discard Z, and transform into image coordinates
    let x_index = ((normed_coord[0] + 1.0) / 2.0 * image.width as f32) as i32;
    let y_index = ((normed_coord[1] + 1.0) / 2.0 * image.height as f32) as i32;

    // check bounds
    if x_index < 0 || x_index >= image.width as i32 || y_index < 0 || y_index >= image.height as i32
    {
        return None;
    }
    let x_index = x_index as usize;
    let y_index = y_index as usize;
    let image_index = x_index + y_index * image.width;

    println!("original: ({},{},{})", x, y, z);
    // println!("view: ({},{},{})", view_coord[0], view_coord[1], view_coord[2]);
    // println!("projected: ({},{},{})", proj_coord[0], proj_coord[1], proj_coord[2]);
    // println!("normed: ({},{},{})", normed_coord[0], normed_coord[1], normed_coord[2]);
    println!("index: ({},{})", x_index, y_index);

    let r = image.data[image_index * 3];
    let g = image.data[image_index * 3 + 1];
    let b = image.data[image_index * 3 + 2];
    println!("color: ({},{},{})", r, g, b);
    Some((r, g, b))
}
