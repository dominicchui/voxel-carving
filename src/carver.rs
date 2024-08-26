use nalgebra::{Vector3, Vector4};

use crate::{
    image::Image, raytracer::trace_ray, voxel::VoxelBlock
};

/// given voxelblock and image, for each voxel, project ray to each camera and get pixel and color
/// ray trace to see if the colors are consistent
/// if not, then carve away
pub(crate) fn carve(voxel_block: &mut VoxelBlock, images: &Vec<Image>) {
    // should_carve(0.0, 0.0, 0.0, image, width, height);
    // carve in each of the 6 directions

    // positive x direction
    // only select images that are looking in the correct direction
    println!("+x");
    let images_subset = &mut vec![];
    for image in images {
        let dir = image.camera.look;
        if dir.x > 0.0 { 
            println!("{}", image.camera.pos);
            images_subset.push(image.clone());
        }
    }
    if !images_subset.is_empty() {
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
                    let (x_f, y_f, z_f) = voxel_block.index_to_coordinate(index);

                    if should_carve(voxel_block, index, x_f, y_f, z_f, images_subset) {
                        voxel_block.carve(x, y, z);
                    }
                }
            }
        }
    }

    // negative x direction
    println!("-x");
    let images_subset = &mut vec![];
    for image in images {
        let dir = image.camera.look;
        if dir.x < 0.0 { 
            println!("{}", image.camera.pos);
            images_subset.push(image.clone());
        }
    }

    if !images_subset.is_empty() {
        for x in (0..voxel_block.resolution).rev() {
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
                    let (x_f, y_f, z_f) = voxel_block.index_to_coordinate(index);

                    if should_carve(voxel_block, index, x_f, y_f, z_f, images_subset) {
                        voxel_block.carve(x, y, z);
                    }
                }
            }
        }
    }

    // positive y direction
    println!("+y");
    let images_subset = &mut vec![];
    for image in images {
        let dir = image.camera.look;
        if dir.y > 0.0 { 
            println!("{}", image.camera.pos);
            images_subset.push(image.clone());
        }
    }
    if !images_subset.is_empty() {
        for y in 0..voxel_block.resolution {
            for x in 0..voxel_block.resolution {
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
                    let (x_f, y_f, z_f) = voxel_block.index_to_coordinate(index);

                    if should_carve(voxel_block, index, x_f, y_f, z_f, images_subset) {
                        voxel_block.carve(x, y, z);
                    }
                }
            }
        }
    }

    // negative y direction
    println!("-y");
    let images_subset = &mut vec![];
    for image in images {
        let dir = image.camera.look;
        if dir.y < 0.0 { 
            println!("{}", image.camera.pos);
            images_subset.push(image.clone());
        }
    }

    if !images_subset.is_empty() {
        for y in 0..voxel_block.resolution {
            for x in 0..voxel_block.resolution {
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
                    let (x_f, y_f, z_f) = voxel_block.index_to_coordinate(index);

                    if should_carve(voxel_block, index, x_f, y_f, z_f, images_subset) {
                        voxel_block.carve(x, y, z);
                    }
                }
            }
        }
    }

    // positive z direction
    println!("+z");
    let images_subset = &mut vec![];
    for image in images {
        let dir = image.camera.look;
        if dir.z > 0.0 { 
            println!("{}", image.camera.pos);
            images_subset.push(image.clone());
        }
    }
    if !images_subset.is_empty() {
        for z in 0..voxel_block.resolution {
            for x in 0..voxel_block.resolution {
                for y in 0..voxel_block.resolution {
                    // convert coordinates to object space
                    let index = x
                        + y * voxel_block.resolution
                        + z * voxel_block.resolution * voxel_block.resolution;

                    // skip not visible voxels 
                    let voxel = &voxel_block.voxels[index];
                    if !voxel.visible {
                        continue;
                    }
                    let (x_f, y_f, z_f) = voxel_block.index_to_coordinate(index);

                    if should_carve(voxel_block, index, x_f, y_f, z_f, images_subset) {
                        voxel_block.carve(x, y, z);
                    }
                }
            }
        }
    }
    // negative z direction
    println!("-z");
    let images_subset = &mut vec![];
    for image in images {
        let dir = image.camera.look;
        if dir.z < 0.0 { 
            println!("{}", image.camera.pos);
            images_subset.push(image.clone());
        }
    }
    if !images_subset.is_empty() {
        for z in (0..voxel_block.resolution).rev() {
            for x in 0..voxel_block.resolution {
                for y in 0..voxel_block.resolution {
                    // convert coordinates to object space
                    let index = x
                        + y * voxel_block.resolution
                        + z * voxel_block.resolution * voxel_block.resolution;

                    // skip not visible voxels 
                    let voxel = &voxel_block.voxels[index];
                    if !voxel.visible {
                        continue;
                    }
                    let (x_f, y_f, z_f) = voxel_block.index_to_coordinate(index);

                    if should_carve(voxel_block, index, x_f, y_f, z_f, images_subset) {
                        voxel_block.carve(x, y, z);
                    }
                }
            }
        }
    }
}

fn should_carve(voxel_block: &mut VoxelBlock, index: usize, x: f32, y: f32, z: f32, images: &Vec<Image>) -> bool {
    if let Some(color) = get_consistent_color(x, y, z, images) {
        // set voxel
        let voxel = &mut voxel_block.voxels[index];
        voxel.color = Some(color);
        if color!=Vector3::zeros() {
            // println!("color: {}", color);
            false
        } else {
            // background pixel is black
            true
        }
    } else {
        true
    }
}

fn get_consistent_color(x: f32, y: f32, z: f32, images: &Vec<Image>) -> Option<Vector3<u8>> {
    // for each image, project voxel to get projected pixel color
    // if all images roughly agree on color, then it is consistent
    let mut estimated_color: Option<Vector3<u8>> = None;
    // let mut estimated_normal: Option<Vector3<f32>> = Some(Vector3::zeros());
    for image in images {
        if let Some((r,g,b)) = get_projected_pixel_color(x, y, z, image) {
            let proj_color = Vector3::new(r,g,b);
            // if proj_color == Vector3::zeros() { return None } //todo remove
            
            // check if colors are similar
            // todo support black as a reasonable color
            if let Some(est_color) = estimated_color {
                if is_roughly_equal(est_color[0], r) &&
                    is_roughly_equal(est_color[1], g) && 
                    is_roughly_equal(est_color[2], b) {
                    estimated_color = Some(average(est_color, proj_color));
                    // estimated_normal = (estimated_normal + ray_traced_normal) / 2.0;
                } else {
                    // colors are inconsistent
                    // println!("not equal: {}, {}", est_color,proj_color);
                    return None;
                }
            } else {
                estimated_color = Some(proj_color);
            }
        }
        // if no projected color, then not visible so ignore        
    }
    estimated_color
}

fn get_projected_pixel_color(x: f32, y: f32, z: f32, image: &Image) -> Option<(u8, u8, u8)> {
    // convert from world space to projected/clip space
    let proj_coord = image.camera.proj_matrix
        * image.camera.view_matrix
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

    // println!("original: ({},{},{})", x, y, z);
    // println!("view: ({},{},{})", view_coord[0], view_coord[1], view_coord[2]);
    // println!("projected: ({},{},{})", proj_coord[0], proj_coord[1], proj_coord[2]);
    // println!("normed: ({},{},{})", normed_coord[0], normed_coord[1], normed_coord[2]);
    // println!("index: ({},{})", x_index, y_index);

    let r = image.data[image_index * 3];
    let g = image.data[image_index * 3 + 1];
    let b = image.data[image_index * 3 + 2];
    // println!("color: ({},{},{})", r, g, b);
    Some((r, g, b))
}


/// checks whether value1 and value2 are within a defined number of values apart
fn is_roughly_equal(value1: u8, value2: u8) -> bool {
    // return true;
    let range = 50;

    let min = if value1.checked_sub(range).is_none() {
        0
    } else {
        value1 - range
    };

    let max = if value1.checked_add(range).is_none() {
        255
    } else {
        value1 + range
    };

    value2 >= min && value2 <= max
}

fn average(value1: Vector3<u8>, value2: Vector3<u8>) -> Vector3<u8> {
    let v0 = (value1[0] as u32 + value2[0] as u32) / 2;
    let v1 = (value1[1] as u32 + value2[1] as u32) / 2;
    let v2 = (value1[2] as u32 + value2[2] as u32) / 2;
    Vector3::new(v0 as u8, v1 as u8, v2 as u8)
}


#[cfg(test)]
mod tests {
    use nalgebra::Vector3;

    use crate::carver::{average, is_roughly_equal};


    #[test]
    fn test_is_roughly_equal() {
        assert!(is_roughly_equal(10,10));
        assert!(is_roughly_equal(10,12));
        assert!(is_roughly_equal(12,10));
        assert!(is_roughly_equal(20,10));
        assert!(is_roughly_equal(10,20));
        assert!(is_roughly_equal(5,10));
        assert!(is_roughly_equal(10,5));
        assert!(is_roughly_equal(254,250));
        assert!(is_roughly_equal(250,254));
        assert!(!is_roughly_equal(22,10));
        assert!(!is_roughly_equal(10,22));
    }

    #[test]
    fn test_average() {
        assert_eq!(average(Vector3::new(10,10,10), Vector3::new(20,20,20)), Vector3::new(15,15,15));
        assert_eq!(average(Vector3::new(11,11,11), Vector3::new(20,20,20)), Vector3::new(15,15,15));
        assert_eq!(average(Vector3::new(10,20,40), Vector3::new(20,40,80)), Vector3::new(15,30,60));
    }
}
