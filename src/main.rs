use carver::carve;
use image::Image;
use nalgebra::Vector3;
use voxel::VoxelBlock;

mod camera;
mod carver;
mod image;
mod voxel;

const LENGTH: usize = 5;
const RESOLUTION: usize = 20;

fn main() {
    let start: std::time::Instant = std::time::Instant::now();

    // let pos = Vector3::new(3.0, 3.0, 3.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let focus = Vector3::new(0.0, 0.0, 0.0);
    let height_angle = 30.0;
    let width = 1024;
    let height = 768;
    let mut image_0 = Image::new_from_file(
        "./data/input/sphere/sphere_1.png".to_owned(),
        Vector3::new(3.0, 3.0, 3.0),
        up,
        focus,
        height_angle,
        width,
        height
    );
    let mut image_1 = Image::new_from_file(
        "./data/input/sphere/sphere_1.png".to_owned(),
        Vector3::new(-3.0, -3.0, -3.0),
        up,
        focus,
        height_angle,
        width,
        height
    );
    let mut image_2 = Image::new_from_file(
        "./data/input/sphere/sphere_1.png".to_owned(),
        Vector3::new(3.0, 3.0, -3.0),
        up,
        focus,
        height_angle,
        width,
        height
    );
    let mut image_3 = Image::new_from_file(
    "./data/input/sphere/sphere_1.png".to_owned(),
        Vector3::new(-3.0, 3.0, -3.0),
        up,
        focus,
        height_angle,
        width,
        height
    );
    let mut images = vec![image_0, image_1, image_2, image_3];

    let mut voxel_block =
        VoxelBlock::new_with_color(LENGTH, RESOLUTION, Some(Vector3::new(255, 255, 255)));
    carve(&mut voxel_block, &mut images);
    
    voxel_block.save_to_file("./data/output/mesh.obj");

    let duration = start.elapsed();
    println!("Mesh operation time: {:?}", duration);
}
