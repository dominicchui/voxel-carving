use carver::carve;
use voxel::VoxelBlock;

mod camera;
mod carver;
mod image;
mod scene_generator;
mod raytracer;
mod voxel;

const LENGTH: usize = 4;
const RESOLUTION: usize = 100;

fn main() {
    let start: std::time::Instant = std::time::Instant::now();
    let images = &mut scene_generator::two_cones();

    let mut voxel_block =
        VoxelBlock::new(LENGTH, RESOLUTION);
    carve(&mut voxel_block, images);

    voxel_block.save_to_file("./data/output/mesh.obj");

    let duration = start.elapsed();
    println!("Elapsed time: {:?}", duration);
}
