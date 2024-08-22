use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    ops::{Index, IndexMut},
};

use nalgebra::Vector3;
use ordered_float::OrderedFloat;

#[derive(Default, Clone, Debug)]
pub(crate) struct Voxel {
    pub(crate) color: Option<Vector3<u8>>,
    pub(crate) visible: bool,
}

pub(crate) struct VoxelBlock {
    pub(crate) voxels: Vec<Voxel>,
    // side length for the block to be carved
    pub(crate) length: usize,
    // how many voxels per side
    pub(crate) resolution: usize,
}

impl Index<(OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>)> for VoxelBlock {
    type Output = Voxel;
    fn index(
        &self,
        coordinates: (OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>),
    ) -> &Self::Output {
        let x = coordinates.0;
        let y = coordinates.1;
        let z = coordinates.2;
        let index = self.coordinate_to_index(*x, *y, *z);
        &self.voxels[index]
    }
}

impl IndexMut<(OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>)> for VoxelBlock {
    fn index_mut(
        &mut self,
        coordinates: (OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>),
    ) -> &mut Self::Output {
        let x = coordinates.0;
        let y = coordinates.1;
        let z = coordinates.2;
        let index = self.coordinate_to_index(*x, *y, *z);
        &mut self.voxels[index]
    }
}

impl Voxel {
    pub fn new(color: Option<Vector3<u8>>) -> Self {
        Voxel { color, visible: false }
    }
}

impl VoxelBlock {
    pub fn new(length: usize, resolution: usize) -> Self {
        Self::new_with_color(length, resolution, None)
    }

    pub fn new_with_color(length: usize, resolution: usize, color: Option<Vector3<u8>>) -> Self {
        let mut voxels = vec![Voxel::new(color); resolution * resolution * resolution];
        
        // make boundary as surface
        for x in 0..resolution {
            for y in 0..resolution {
                for z in 0..resolution {
                    if x == 0 || x == resolution - 1 ||
                        y == 0 || y == resolution - 1 ||
                        z == 0 || z == resolution - 1 {
                            let index = x + y * resolution + z * resolution * resolution;
                            voxels[index].visible = true;
                        }
                }
            }
        }

        VoxelBlock {
            voxels,
            length,
            resolution,
        }
    }

    pub fn voxel_length(&self) -> f32 {
        self.length as f32 / self.resolution as f32
    }

    pub fn coordinate_to_index(&self, x: f32, y: f32, z: f32) -> usize {
        Self::_coordinate_to_index(x, y, z, self.length, self.resolution)
    }

    fn _coordinate_to_index(x: f32, y: f32, z: f32, length: usize, resolution: usize) -> usize {
        // reshift (0,0,0) to origin
        let half = length as f32 / 2.0;
        let x_2 = x + half;
        let y_2 = y + half;
        let z_2 = z + half;

        let voxel_length = length as f32 / resolution as f32;
        let x_index = (x_2 / voxel_length) as usize;
        let y_index = (y_2 / voxel_length) as usize;
        let z_index = (z_2 / voxel_length) as usize;

        x_index + y_index * resolution + z_index * resolution * resolution
    }

    pub fn index_to_coordinate(&self, index: usize) -> (f32, f32, f32) {
        Self::_index_to_coordinate(index, self.length, self.resolution)
    }

    fn _index_to_coordinate(index: usize, length: usize, resolution: usize) -> (f32, f32, f32) {
        let voxel_length = length as f32 / resolution as f32;
        let z = (index / (resolution * resolution)) as f32 * voxel_length;
        let remainder = index % (resolution * resolution);
        let y = (remainder / resolution) as f32 * voxel_length;
        let x = (remainder % resolution) as f32 * voxel_length;

        // recenter origin as the center of the block
        let half = length as f32 / 2.0;
        (x - half, y - half, z - half)
    }

    pub fn save_to_file(self, file_path: &str) {
        let f = File::create(file_path);
        let mut file = f.expect("Unable to open or create file");

        // Write vertices
        for z in 0..=self.resolution {
            for y in 0..=self.resolution {
                for x in 0..=self.resolution {
                    let vertex: String = format!("v {} {} {}\n", x, y, z);
                    file.write_all(vertex.as_bytes())
                        .expect("Unable to write to file");
                }
            }
        }

        file.write_all(b"\n").expect("Unable to write to file");

        // Write faces
        // each voxel has 6 faces and 12 triangles
        for z in 0..self.resolution {
            for y in 0..self.resolution {
                for x in 0..self.resolution {
                    let voxel_index =
                        x + y * self.resolution + z * self.resolution * self.resolution;

                    // check if voxel is present and visible
                    let voxel = &self.voxels[voxel_index];
                    if !voxel.visible || voxel.color.is_none() {
                        continue;
                    }

                    // Vertices
                    // convert from voxel index to vertex index
                    // OBJ files index starting at 1
                    let vertex_index = x
                        + y * (1 + self.resolution)
                        + z * (1 + self.resolution) * (1 + self.resolution)
                        + 1;
                    
                    // how shifting in each direction changes the vertex index
                    let x_shift = 1;
                    let y_shift = 1 + self.resolution;
                    let z_shift = (1 + self.resolution) * (1 + self.resolution);

                    let front_top_left: usize = vertex_index;
                    let front_top_right: usize = vertex_index + x_shift;
                    let front_bottom_left: usize = vertex_index + y_shift;
                    let front_bottom_right: usize = vertex_index + x_shift + y_shift;
                    let back_top_left: usize = vertex_index + z_shift;
                    let back_top_right: usize = vertex_index + x_shift + z_shift;
                    let back_bottom_left: usize = vertex_index + y_shift + z_shift;
                    let back_bottom_right: usize = vertex_index + x_shift + y_shift + z_shift;

                    // Faces
                    // front face
                    let f1 = (front_top_left, front_bottom_left, front_top_right);
                    let f2 = (front_bottom_left, front_bottom_right, front_top_right);
                    // right face
                    let f3 = (front_top_right, front_bottom_right, back_top_right);
                    let f4 = (front_bottom_right, back_bottom_right, back_top_right);
                    // left face
                    let f5 = (back_top_left, back_bottom_left, front_top_left);
                    let f6 = (back_bottom_left, front_bottom_left, front_top_left);
                    // back face
                    let f7 = (back_top_right, back_bottom_right, back_bottom_left);
                    let f8 = (back_top_right, back_bottom_left, back_top_left);
                    // top face
                    let f9 = (back_top_left, front_top_left, back_top_right);
                    let f10 = (front_top_left, front_top_right, back_top_right);
                    // bottom face
                    let f11 = (front_bottom_left, back_bottom_left, front_bottom_right);
                    let f12 = (back_bottom_left, back_bottom_right, front_bottom_right);

                    let faces = [f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12];
                    for f in faces {
                        let face: String = format!("f {} {} {}\n", f.0, f.1, f.2);
                        file.write_all(face.as_bytes())
                            .expect("Unable to write to file");
                    }
                }
            }
        }
    }

    pub fn carve(&mut self, x: usize, y: usize, z: usize) {
        let res_squared = self.resolution * self.resolution;
        let index = x + y * self.resolution + z * res_squared;
        let voxel = &mut self.voxels[index];
        voxel.color = None;
        voxel.visible = false;

        // mark 8 neighbors as visible
        let max_index = (res_squared * self.resolution) as i32;

        let left = index as i32 - 1;
        if left >= 0 && left < max_index {
            let voxel = &mut self.voxels[left as usize];
            voxel.visible = true;
        }
        let right = index as i32 + 1;
        if right >= 0 && right < max_index {
            let voxel = &mut self.voxels[right as usize];
            voxel.visible = true;
        }
        let top = index as i32 - self.resolution as i32;
        if top >= 0 && top < max_index {
            let voxel = &mut self.voxels[top as usize];
            voxel.visible = true;
        }
        let bottom = index as i32 + self.resolution as i32;
        if bottom >= 0 && bottom < max_index {
            let voxel = &mut self.voxels[bottom as usize];
            voxel.visible = true;
        }
        let forward = index as i32 - res_squared as i32;
        if forward >= 0 && forward < max_index {
            let voxel = &mut self.voxels[forward as usize];
            voxel.visible = true;
        }
        let back = index as i32 + res_squared as i32;
        if back >= 0 && back < max_index {
            let voxel = &mut self.voxels[back as usize];
            voxel.visible = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VoxelBlock;

    #[test]
    fn test_indexing() {
        let voxel_block = VoxelBlock::new(10, 10);
        assert_eq!(voxel_block.coordinate_to_index(-5.0, -5.0, -5.0), 0);
        assert_eq!(voxel_block.coordinate_to_index(-4.0, -5.0, -5.0), 1);
        assert_eq!(voxel_block.coordinate_to_index(-5.0, -4.0, -5.0), 10);
        assert_eq!(voxel_block.coordinate_to_index(-5.0, -5.0, -4.0), 100);
        assert_eq!(voxel_block.index_to_coordinate(0), (-5.0, -5.0, -5.0));
        assert_eq!(voxel_block.index_to_coordinate(1), (-4.0, -5.0, -5.0));
        assert_eq!(voxel_block.index_to_coordinate(11), (-4.0, -4.0, -5.0));
        assert_eq!(voxel_block.index_to_coordinate(111), (-4.0, -4.0, -4.0));
    }
}
