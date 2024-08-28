use nalgebra::{Vector3, Vector4};

use crate::{
    image::Image, voxel::VoxelBlock
};

enum Consistency {
    Consistent(Vector3<u8>),
    Inconsistent,
    Inconclusive,
    Background,
}

enum ProjectedColor {
    Color(Vector3<u8>),
    Background,
    Unknown
}

#[derive(Debug)]
enum Plane {
    X,
    Y,
    Z
}

/// given voxelblock and image, for each voxel, project ray to each camera and get pixel and color
/// ray trace to see if the colors are consistent
/// if not, then carve away
pub(crate) fn carve(voxel_block: &mut VoxelBlock, images: &mut [Image]) {
    // carve in each of the 6 directions until nothing left to be removed
    loop {
        println!("loop!");
        let mut carved_count = 0;
        let sweeps = vec![
            (Plane::X, false), (Plane::X, true),
            (Plane::Y, false), (Plane::Y, true),
            (Plane::Z, false), (Plane::Z, true)
            ];

        for (plane, reverse) in sweeps {
            println!("sweep plane {plane:?} reversed? {reverse}");
            let count = sweep_plane(&plane, reverse, images, voxel_block);
            println!("carved {count} voxels");
            carved_count += count;
        }
        if carved_count == 0 {
            break;
        }
    }
}

fn sweep_plane(plane: &Plane, reverse: bool, images: &mut [Image], voxel_block: &mut VoxelBlock) -> usize {
    let plane_bounds: Box<dyn Iterator<Item = _>> = if reverse {
        Box::new((0..voxel_block.resolution).rev())
    } else {
        Box::new(0..voxel_block.resolution)
    };

    let valid_images = &mut vec![];

    for image in images {
        match plane {
            Plane::X => {
                if (reverse && image.camera.look.x < 0.0) || (!reverse && image.camera.look.x > 0.0) {
                    println!("{}", image.camera.pos);
                    valid_images.push(image);
                }
            },
            Plane::Y => {
                if (reverse && image.camera.look.y < 0.0) || (!reverse && image.camera.look.y > 0.0) {
                    println!("{}", image.camera.pos);
                    valid_images.push(image);
                }
            },
            Plane::Z => {
                if (reverse && image.camera.look.z < 0.0) || (!reverse && image.camera.look.z > 0.0) {
                    println!("{}", image.camera.pos);
                    valid_images.push(image);
                }
            },
        }
    }

    // sweep through the slices
    let mut carved_count = 0;
    for a in plane_bounds {

        // reset marked pixels
        // for image in &mut *valid_images {
        //     image.marked = vec![false; image.data.len()/3];
        // }

        let mut carved = vec![];
        for b in 0..voxel_block.resolution {
            for c in 0..voxel_block.resolution {
                // get coordinate of voxel
                let (x,y,z) = match plane {
                    Plane::X => (a,c,b),
                    Plane::Y => (b,a,c),
                    Plane::Z => (c,b,a),
                };
                let index = x
                            + y * voxel_block.resolution
                            + z * voxel_block.resolution * voxel_block.resolution;
                // skip not visible voxels
                let voxel = &mut voxel_block.voxels[index];
                if !voxel.visible || voxel.carved {
                    continue;
                }

                match should_carve_voxel(index, valid_images, voxel_block) {
                    Consistency::Consistent(color) =>{
                        let voxel = &mut voxel_block.voxels[index];
                        voxel.color = Some(color);
                    },
                    Consistency::Inconsistent => {
                        // println!("inconsistent");
                        carved.push(index);
                    },
                    Consistency::Inconclusive => {
                        // println!("Inconclusive");
                    },
                    Consistency::Background => {
                        // println!("background");
                        carved.push(index);
                    },
                }
            }
        }
        // carve voxels
        carved_count += carved.len();
        for voxel in carved {
            voxel_block.carve(voxel);
        }
    }
    carved_count
}

fn should_carve_voxel(index: usize, images: &mut [&mut Image], voxel_block: &mut VoxelBlock) -> Consistency {
    let (x,y,z) = voxel_block.index_to_coordinate(index);
    let mut projected_colors = vec![];
    for image in images {
        if let Some(projected_index) = project_coordinate(x,y,z,image, voxel_block) {
            let r = image.data[projected_index * 3];
            let g = image.data[projected_index * 3 + 1];
            let b = image.data[projected_index * 3 + 2];
            if r == 0 && g == 0 && b ==0  {
                return Consistency::Background;
            } else {
                let color = Vector3::new(r, g, b);
                projected_colors.push(color);
            }
        }
    }
    if projected_colors.is_empty() {
        Consistency::Inconclusive
    } else if let Some(color) = colors_roughly_equal(projected_colors) { 
        Consistency::Consistent(color)
    } else {
        Consistency::Inconsistent
    }
}

pub fn project_coordinate(x: f32, y: f32, z: f32, image: &Image, voxel_block: &VoxelBlock) -> Option<usize> {
    // center the coordinate in the voxel
    let half_voxel_length = voxel_block.length as f32 / voxel_block.resolution as f32 / 2.0;
    let x_half = x + half_voxel_length;
    let y_half = y + half_voxel_length;
    let z_half = z + half_voxel_length;
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
        return None;
    }

    // ignore if pixel has been marked
    let index = (x_index + image.width as i32 * y_index) as usize;
    if image.marked[index] {
        println!("occluded!");
        return None;
    }

    Some(index)
}


/// checks whether value1 and value2 are within a defined number of values apart
fn is_roughly_equal(value1: u8, value2: u8, threshold: u8) -> bool {
    let min = if value1.checked_sub(threshold).is_none() {
        0
    } else {
        value1 - threshold
    };

    let max = if value1.checked_add(threshold).is_none() {
        255
    } else {
        value1 + threshold
    };

    value2 >= min && value2 <= max
}

fn colors_roughly_equal(colors: Vec<Vector3<u8>>) -> Option<Vector3<u8>> {
    // println!("colors {colors:?}");
    let range = 50;
    let mut r_total = 0;
    let mut g_total = 0;
    let mut b_total = 0;
    let count = colors.len();
    for color in &colors {
        r_total += color[0] as usize;
        g_total += color[1] as usize;
        b_total += color[2] as usize;
    }
    let r_avg = (r_total / count) as u8;
    let g_avg = (g_total / count) as u8;
    let b_avg = (b_total / count) as u8;
    for color in &colors {
        if !is_roughly_equal(r_avg, color[0], range) || 
            !is_roughly_equal(g_avg, color[1], range) || 
            !is_roughly_equal(b_avg, color[2], range) {
            return None;
        }
    }
    Some(Vector3::new(r_avg, g_avg, b_avg))
}


#[cfg(test)]
mod tests {
    use nalgebra::Vector3;

    use crate::carver::is_roughly_equal;

    #[test]
    fn test_is_roughly_equal() {
        assert!(is_roughly_equal(10,10, 10));
        assert!(is_roughly_equal(10,12, 10));
        assert!(is_roughly_equal(12,10, 10));
        assert!(is_roughly_equal(20,10, 10));
        assert!(is_roughly_equal(10,20, 10));
        assert!(is_roughly_equal(5,10, 10));
        assert!(is_roughly_equal(10,5, 10));
        assert!(is_roughly_equal(254,250, 10));
        assert!(is_roughly_equal(250,254, 10));
        assert!(!is_roughly_equal(22,10, 10));
        assert!(!is_roughly_equal(10,22, 10));
    }
}
