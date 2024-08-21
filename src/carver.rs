use crate::voxel::{Voxel, VoxelBlock};

/// given voxelblock and image, for each voxel, project ray to each camera and get pixel and color
/// ray trace to see if the colors are consistent
/// if not, then carve away
pub(crate) fn carve(voxel_block: &mut VoxelBlock) {
    // carve in each of the 6 directions

    // positive x direction
    for x in 0..voxel_block.length {
        for y in 0..voxel_block.length {
            for z in 0..voxel_block.length {
                let resolution = voxel_block.resolution;
                let index = x + y * resolution + z * resolution * resolution;
                let voxel = &mut voxel_block.voxels[index];
                if should_carve(x, y, z) {
                    voxel.carve();
                }
            }
        }
    }
}

fn should_carve(x: usize, y: usize, z: usize) -> bool {
    x + y + z < 20
}