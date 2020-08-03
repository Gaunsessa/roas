use crate::server::packets::ServerPacket;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            r,
            g,
            b,
        }
    }
}

impl ServerPacket<3> for Color {
    fn ser(&self) -> [u8; 3] {
        let data = vec!(
            self.b,
            self.g,
            self.r,
        );

        let mut res = [0; 3];
        res.copy_from_slice(&data[..3]);

        res
    }
}