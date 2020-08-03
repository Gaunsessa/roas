use crate::color::Color;
// use crate::server::packets::Packet;

// Height = 64, Width/Length = 512

// 512^2 2D array with collums.
// Collums can be 64 voxels high.

// #[derive(Clone, Copy, Debug)]
// pub struct Voxel {
//     pub r#type: VoxelType,
//     pub color: Option<Color>
// }

#[derive(Clone, Copy, Debug)]
pub enum Voxel {
    Air,
    Soild,
    Surface(Color)
}

// #[derive(Clone, Debug)]
pub struct VoxelColumn {
    inner: Vec<VoxelSpan>
}

// #[derive(Clone, Debug)]
pub struct VoxelSpan {
    pub length: u8,
    pub color_start: u8,
    pub color_end: u8,
    pub air_start: u8,
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8
}

pub struct Map {
    pub inner: Box<[[VoxelColumn; 512]; 512]>
}

impl Map {
    // pub fn from_file() -> Self {
        


    //     Self {

    //     }
    // }

    pub fn empty() -> Self {
        Self {
            inner: box[[[Voxel::Air; 64]; 512]; 512]
        }
    }

    // pub fn get_voxel(&self, x: usize, y: usize, z: usize) -> &Voxel {
    //     &self.inner[x][z][y]
    // }

    // pub fn set_voxel(&mut self, voxel: Voxel, x: usize, y: usize, z: usize) {
    //     self.inner[x][z][y] = voxel;
    // }
}

// impl Packet<1> for Map {
//     fn ser(&self) -> [u8; 1] {

//     }
// }