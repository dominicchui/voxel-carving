use nalgebra::{Vector3, Vector4};

use crate::{
    image::Image, raytracer::{generate_ray, generate_ray_direct, trace_ray}, voxel::{Voxel, VoxelBlock}
};

enum Consistency {
    Consistent(Vector3<u8>),
    Inconsistent,
    Inconclusive,
    Background,
}

enum ProjectedColor {
    Color(Vector3<u8>, usize),
    Background,
    Unknown
}

/// given voxelblock and image, for each voxel, project ray to each camera and get pixel and color
/// ray trace to see if the colors are consistent
/// if not, then carve away
pub(crate) fn carve(voxel_block: &mut VoxelBlock, images: &Vec<Image>) {
    // should_carve(0.0, 0.0, 0.0, image, width, height);
    // carve in each of the 6 directions until nothing left to be removed
    let mut carving = true;

    while carving {
        println!("loop!");
        // reset at top of each loop
        carving = false;

        // positive x direction
        // only select images that are looking in the correct direction
        println!("+x");
        let images_subset = &mut vec![];
        for image in images {
            let dir = image.camera.look;
            if dir.x >= 0.0 { 
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
                            carving = true;
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
            if dir.x <= 0.0 { 
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
                            carving = true;
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
                            carving = true;
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
            for y in (0..voxel_block.resolution).rev() {
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
                            carving = true;
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
            if dir.z >= 0.0 { 
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
                            carving = true;
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
                            carving = true;
                            voxel_block.carve(x, y, z);
                        }
                    }
                }
            }
        }
    }
}

fn should_carve(voxel_block: &mut VoxelBlock, index: usize, x: f32, y: f32, z: f32, images: &mut Vec<Image>) -> bool {
    // println!("index {index}, coords ({x},{y},{z})");
    match get_consistent_color(x, y, z, images, voxel_block) {
        Consistency::Consistent(color) => {
            println!("consistent {index} with color ({},{},{})", color[0], color[1], color[2]);
            // set voxel
            let voxel = &mut voxel_block.voxels[index];
            voxel.color = Some(color);
            // voxel.seen = true;
            false
        },
        Consistency::Inconsistent => {
            println!("inconsistent {index}");
            true
        },
        Consistency::Inconclusive => {
            println!("inconclusive {index}");
            // let voxel = &mut voxel_block.voxels[index];
            // voxel.seen = true;
            false
        },
        Consistency::Background => {
            println!("background {index}");
            true
        },
    }
}

fn get_consistent_color(x: f32, y: f32, z: f32, images: &mut [Image], voxel_block: &VoxelBlock) -> Consistency {
    // for each image, project voxel to get projected pixel color
    // if all images roughly agree on color, then it is consistent
    let mut estimated_color: Option<Vector3<u8>> = None;
    let mut image_indices = vec![];
    for (i,image) in images.iter().enumerate() {
        match get_projected_pixel_color(x, y, z, image, voxel_block) {
            ProjectedColor::Color(proj_color, image_index) => {
                // println!("({r},{g},{b})");
                image_indices.push((i,image_index));
                
                // check if colors are similar
                if let Some(est_color) = estimated_color {
                    if is_roughly_equal(est_color[0], proj_color[0]) &&
                        is_roughly_equal(est_color[1], proj_color[1]) && 
                        is_roughly_equal(est_color[2], proj_color[2]) {
                        estimated_color = Some(average(est_color, proj_color));
                    } else {
                        // colors are inconsistent
                        println!("not equal: {}, {}", est_color,proj_color);
                        return Consistency::Inconsistent;
                    }
                } else {
                    estimated_color = Some(proj_color);
                }
            },
            // if no projected color, then not visible so don't do anything
            ProjectedColor::Unknown => {},
            ProjectedColor::Background => {return Consistency::Background;},
        }
    }
    if let Some(color) = estimated_color {
        // mark projected pixels
        for (image, index) in image_indices {
            images[image].marked[index] = true;
        }
        Consistency::Consistent(color)
    } else {
        Consistency::Inconclusive
    }
}

pub fn project_coordinate(x: f32, y: f32, z: f32, image: &Image, voxel_block: &VoxelBlock) -> Option<(usize,usize)> {
    // center the coordinate in the voxel
    let half_voxel_length = voxel_block.length as f32 / voxel_block.resolution as f32 / 2.0;
    let x_half = x + half_voxel_length;
    let y_half = y + half_voxel_length;
    let z_half = z + half_voxel_length;
    // println!("shifted ({x_half},{y_half},{z_half})");
    // convert from world space to projected/clip space
    let proj_coord = image.camera.proj_matrix
        * image.camera.view_matrix
        * Vector4::new(x_half, y_half, z_half, 1.0);

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
        None
    } else {
        Some((x_index as usize, y_index as usize))
    }
}

fn get_projected_pixel_color(x: f32, y: f32, z: f32, image: &Image, voxel_block: &VoxelBlock) -> ProjectedColor {
    if let Some((x_index, y_index)) = project_coordinate(x, y, z, image, voxel_block) {
        let image_index = x_index + y_index * image.width;
        // only use if projected pixel is not marked
        if image.marked[image_index] {
            println!("pixel already marked");
            return ProjectedColor::Unknown;
        }
        // println!("projected to {x_index}, {y_index}");

        // check if voxel is occluded
        // let ray = generate_ray(x_index, y_index, 1.0, &image.camera);
        // let half_voxel_length = voxel_block.voxel_length() / 2.0;
        // let center = Vector3::new(x + half_voxel_length, y + half_voxel_length, z + half_voxel_length);
        // let ray = generate_ray_direct(center.x, center.y, center.z, &image.camera);
        // let dir = ray.d;
        // let pos = ray.p;
        // let expected_index = voxel_block.coordinate_to_index(x, y, z);
        // if let Some(intersect) = trace_ray(&ray, voxel_block, expected_index) {
        //     if intersect != expected_index {
        //         // occluded
        //         // println!("ray dir: {}", dir);
        //         // println!("ray pos: {}", pos);
        //         println!("({x_index},{y_index}): expected {expected_index}, actual {intersect}");
        //         return ProjectedColor::Unknown
        //     }
        // } else {
        //     // background detected (shouldn't happen)
        //     println!("({x_index},{y_index}) with ray pos {} and dir {} going to background unexpectedly", ray.p, ray.d);
        //     panic!();
        // }

        let r = image.data[image_index * 3];
        let g = image.data[image_index * 3 + 1];
        let b = image.data[image_index * 3 + 2];
        // println!("color: ({},{},{})", r, g, b);
        if r == 0 && g == 0 && b ==0  {
            ProjectedColor::Background
        } else {
            ProjectedColor::Color(Vector3::new(r, g, b), image_index)
        }
    } else {
        // out of view for this image
        ProjectedColor::Unknown
    }
}


/// checks whether value1 and value2 are within a defined number of values apart
fn is_roughly_equal(value1: u8, value2: u8) -> bool {
    // return true;
    let range = 150;

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
