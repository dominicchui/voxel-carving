use carver::carve;
use image::Image;
use nalgebra::{Point3, Vector3};
use voxel::VoxelBlock;

mod camera;
mod carver;
mod image;
mod scene_generator;
mod raytracer;
mod voxel;

const LENGTH: usize = 2;
const RESOLUTION: usize = 10;

fn main() {
    let start: std::time::Instant = std::time::Instant::now();
    let images = &mut scene_generator::cone();

    let mut voxel_block =
        VoxelBlock::new(LENGTH, RESOLUTION);

    println!("camera pos {}", images[0].camera.pos);

    // for i in 0..7 {
    //     let coord = voxel_block.index_to_coordinate(i);
    //     // let point = Point3::new(coord.0, coord.1, coord.2);
    //     let proj_point = carver::project_coordinate(coord.0, coord.1, coord.2, &images[0], &voxel_block).unwrap();
    //     // let proj_point = images[0].camera.proj_matrix.project_point(&point);
    //     // let x_index = ((proj_point.0 + 1.0) / 2.0 * images[0].width as f32) as i32;
    //     // let y_index = ((proj_point.1 + 1.0) / 2.0 * images[0].height as f32) as i32;
    //     println!("index {i}, coordinates ({},{},{})", coord.0, coord.1, coord.2);
    //     println!("proj_point ({}, {})", proj_point.0, proj_point.1);
    //     let image_index = proj_point.0 + proj_point.1 * images[0].width;
    //     let r = images[0].data[image_index * 3];
    //     let g = images[0].data[image_index * 3 + 1];
    //     let b = images[0].data[image_index * 3 + 2];
    //     println!("color: ({r},{g},{b})");
    // }
    carve(&mut voxel_block, images);

    voxel_block.save_to_file("./data/output/mesh.obj");

    let duration = start.elapsed();
    println!("Mesh operation time: {:?}", duration);
}
