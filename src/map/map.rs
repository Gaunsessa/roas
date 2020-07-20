use crate::color::Color;
// use crate::server::packets::Packet;

#[derive(Clone, Copy, Debug)]
pub struct Voxel {
    pub r#type: VoxelType,
    pub color: Option<Color>
}

#[derive(Clone, Copy, Debug)]
pub enum VoxelType {
    Air,
    Soild,
    Surface
}

pub struct Map {
    pub inner: Box<[[[Voxel; 64]; 512]; 512]>
}

impl Map {
    pub fn from_file() -> Self {
        


        Self {

        }
    }

    pub fn empty() -> Self {
        Self {
            inner: box[[[Voxel {
                r#type: VoxelType::Air,
                color: None
            }; 64]; 512]; 512]
        }
    }

    pub fn get_voxel(&self, x: usize, y: usize, z: usize) -> &Voxel {
        &self.inner[x][z][y]
    }

    pub fn set_voxel(&mut self, voxel: Voxel, x: usize, y: usize, z: usize) {
        self.inner[x][z][y] = voxel;
    }
}

// impl Packet<1> for Map {
//     fn ser(&self) -> [u8; 1] {

//     }
// }