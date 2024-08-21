use carver::carve;
use image::Image;
use nalgebra::Vector3;
use voxel::VoxelBlock;

mod camera;
mod carver;
mod image;
mod voxel;

const LENGTH: usize = 10;
const RESOLUTION: usize = 10;

fn main() {
    let start: std::time::Instant = std::time::Instant::now();
    println!("Hello, world!");

    let pos = Vector3::new(3.0, 3.0, 3.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let focus = Vector3::new(0.0, 0.0, 0.0);
    let height_angle = 30.0;
    let image = Image::new_from_file("./data/input/cube/cube_0.png".to_owned(), pos, up, focus, height_angle);

    let mut voxel_block = VoxelBlock::new_with_color(LENGTH, RESOLUTION,  Some(Vector3::new(255, 255, 255)));
    carve(&mut voxel_block);
    voxel_block.save_to_file("./data/output/mesh.obj");
    

    let duration = start.elapsed();
    println!("Mesh operation time: {:?}", duration);
}
