use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    ops::{Index, IndexMut},
};

use nalgebra::{base, Matrix4, Translation3, Vector3, Vector4};
use ordered_float::OrderedFloat;

#[derive(Default, Clone, Debug)]
pub(crate) struct Voxel {
    pub(crate) carved: bool,
    pub(crate) visible: bool,
    pub(crate) seen: bool,
    // estimated (diffuse) color of the voxel
    pub(crate) color: Option<Vector3<u8>>,
    pub(crate) ctm: Matrix4<f32>,
    pub(crate) inverse_ctm: Matrix4<f32>,
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
    pub fn new() -> Self {
        Voxel {
            carved: false,
            visible: false,
            seen: false,
            color: None,
            ctm: Matrix4::identity(),
            inverse_ctm: Matrix4::identity()
        }
    }
}

impl VoxelBlock {
    pub fn new(length: usize, resolution: usize) -> Self {
        let mut voxels = vec![Voxel::new(); resolution * resolution * resolution];
        
        let voxel_length = length as f32 / resolution as f32;
        let baseline_shift = -(length as f32 / 2.0 - 0.5 * voxel_length);
        println!("voxel_length: {}", voxel_length);
        println!("baseline_shift: {}", baseline_shift);
        
        // make boundary as surface and set CTM
        for x in 0..resolution {
            for y in 0..resolution {
                for z in 0..resolution {
                    let index = x + y * resolution + z * resolution * resolution;
                    if x == 0
                        || x == resolution - 1
                        || y == 0
                        || y == resolution - 1
                        || z == 0
                        || z == resolution - 1
                    {
                        voxels[index].visible = true;
                    }

                    // ctm
                    let ctm = Self::calculate_ctm(x, y, z, voxel_length, baseline_shift);
                    let inverse_ctm = ctm.try_inverse().unwrap();

                    voxels[index].ctm = ctm;
                    voxels[index].inverse_ctm = inverse_ctm;
                }
            }
        }

        VoxelBlock {
            voxels,
            length,
            resolution
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

    fn calculate_ctm(x: usize, y: usize, z: usize, voxel_length: f32, baseline_shift: f32) -> Matrix4<f32> {
        let scale = Matrix4::new_scaling(voxel_length);
        let trans = Self::translation(x, y, z, voxel_length, baseline_shift);
        let mat: Matrix4<f32> = Matrix4::identity();
        mat * trans * scale
    }

    fn translation(x: usize, y: usize, z: usize, voxel_length: f32, baseline_shift: f32) -> Matrix4<f32> {
        let x_shift = x as f32 * voxel_length + baseline_shift;
        let y_shift = y as f32 * voxel_length + baseline_shift;
        let z_shift = z as f32 * voxel_length + baseline_shift;
        let t = Translation3::new(x_shift, y_shift, z_shift);
        t.to_homogeneous()
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

        let mut carved = 0;
        let mut consistent = 0;
        let mut inconclusive = 0;

        // Write faces
        // each voxel has 6 faces and 12 triangles
        for z in 0..self.resolution {
            for y in 0..self.resolution {
                for x in 0..self.resolution {
                    let voxel_index =
                        x + y * self.resolution + z * self.resolution * self.resolution;

                    // check if voxel is present, seen, and visible
                    let voxel = &self.voxels[voxel_index];
                    if voxel.carved {
                        carved += 1;
                        continue;
                    }
                    if voxel.seen {
                        if voxel.color.is_some() {
                            consistent += 1;
                        } else {
                            inconclusive += 1;
                        }
                    }
                    if !voxel.visible || voxel.carved {//|| voxel.color.is_none() {
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
        println!("Consistent {consistent}");
        println!("Carved {carved}");
        println!("Inconclusive {inconclusive}");
    }

    pub fn carve(&mut self, x: usize, y: usize, z: usize) {
        let res_squared = self.resolution * self.resolution;
        let index = x + y * self.resolution + z * res_squared;
        let voxel = &mut self.voxels[index];
        voxel.carved = true;
        voxel.visible = false;
        voxel.seen = true;
        println!("carve {index}");

        // mark 8 neighbors as visible
        let max_index = (res_squared * self.resolution) as i32;

        let left = index as i32 - 1;
        if left >= 0 && left < max_index {
            let voxel = &mut self.voxels[left as usize];
            if !voxel.carved {
                voxel.visible = true;
            }
        }
        let right = index as i32 + 1;
        if right >= 0 && right < max_index {
            let voxel = &mut self.voxels[right as usize];
            if !voxel.carved {
                voxel.visible = true;
            }
        }
        let top = index as i32 - self.resolution as i32;
        if top >= 0 && top < max_index {
            let voxel = &mut self.voxels[top as usize];
            if !voxel.carved {
                voxel.visible = true;
            }
        }
        let bottom = index as i32 + self.resolution as i32;
        if bottom >= 0 && bottom < max_index {
            let voxel = &mut self.voxels[bottom as usize];
            if !voxel.carved {
                voxel.visible = true;
            }
        }
        let forward = index as i32 - res_squared as i32;
        if forward >= 0 && forward < max_index {
            let voxel = &mut self.voxels[forward as usize];
            if !voxel.carved {
                voxel.visible = true;
            }
        }
        let back = index as i32 + res_squared as i32;
        if back >= 0 && back < max_index {
            let voxel = &mut self.voxels[back as usize];
            if !voxel.carved {
                voxel.visible = true;
            }
        }
    }
}

pub(crate) fn find_cube_intersect(voxel: &Voxel, p: Vector4<f32>, d: Vector4<f32>) -> Option<f32> {
    let error: f32 = 0.00001;
    let p = voxel.inverse_ctm * p;
    let d = voxel.inverse_ctm * d + error * d;
    let t_x_neg = (-0.5 - p.x) / d.x;
    let t_x_pos = (0.5 - p.x) / d.x;
    let t_y_neg = (-0.5 - p.y) / d.y;
    let t_y_pos = (0.5 - p.y) / d.y;
    let t_z_neg = (-0.5 - p.z) / d.z;
    let t_z_pos = (0.5 - p.z) / d.z;
    let candidate_ts = [t_x_neg, t_x_pos, t_y_neg, t_y_pos, t_z_neg, t_z_pos];
    let mut valid_ts: Vec<f32> = Vec::new();

    for t in candidate_ts.into_iter() {
        if is_valid_cube_t(p, d, t) {
            valid_ts.push(t);
        }
    }

    if valid_ts.is_empty() {
        None
    } else {
        valid_ts.sort_by(|a, b| a.total_cmp(b));
        Some(valid_ts[0])
    }
}

pub(crate) fn is_valid_cube_t(p: Vector4<f32>, d: Vector4<f32>, t: f32) -> bool {
    if t <= 0.0 {
        return false;
    }
    let intersect = p + t * d;
    let x = intersect[0];
    let y = intersect[1];
    let z = intersect[2];

    let bounds = 0.5 + 0.0001;

    // check finite boundaries
    !(x < -bounds
        || x > bounds
        || y < -bounds
        || y > bounds
        || z < -bounds
        || z > bounds)
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector4;

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

    #[test]
    fn test_ctm() {
        let voxel_block = VoxelBlock::new(2, 2);
        let voxel = &voxel_block.voxels[0];
        // convert to world space
        let ctm = voxel.ctm;
        println!("ctm {}", ctm);
        assert_eq!(ctm * Vector4::new(-0.5, -0.5, -0.5, 1.0), Vector4::new(-1.0,-1.0,-1.0, 1.0));
        assert_eq!(ctm * Vector4::new(0.0, 0.0, 0.0, 1.0), Vector4::new(-0.5,-0.5,-0.5, 1.0));

        let voxel = &voxel_block.voxels[3];
        // convert to world space
        let ctm = voxel.ctm;
        println!("ctm {}", ctm);
        assert_eq!(ctm * Vector4::new(-0.5, -0.5, -0.5, 1.0), Vector4::new(0.0,0.0,-1.0, 1.0));
        assert_eq!(ctm * Vector4::new(0.0, 0.0, 0.0, 1.0), Vector4::new(0.5,0.5,-0.5, 1.0));
    }
}
