use crate::server::packets::ServerPacket;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

impl ServerPacket for Color {
    fn ser(&self) -> Vec<u8> {
        vec![self.b, self.g, self.r]
    }
}
