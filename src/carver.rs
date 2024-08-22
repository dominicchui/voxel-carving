use crate::voxel::VoxelBlock;

/// given voxelblock and image, for each voxel, project ray to each camera and get pixel and color
/// ray trace to see if the colors are consistent
/// if not, then carve away
pub(crate) fn carve(voxel_block: &mut VoxelBlock) {
    // carve in each of the 6 directions

    // positive x direction
    for x in 0..voxel_block.resolution {
        for y in 0..voxel_block.resolution {
            for z in 0..voxel_block.resolution {
                if should_carve(x, y, z) {
                    voxel_block.carve(x, y, z);
                }
            }
        }
    }
}

fn should_carve(x: usize, y: usize, z: usize) -> bool {
    x + y + z < 20
}
