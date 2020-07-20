use crate::server::structs::*;
use crate::color::Color;

pub trait Packet<const X: usize> {
    fn ser(&self) -> [u8; X];
}

pub struct MapStart {
    pub map_size: u32,
}

impl Packet<5> for MapStart {
    fn ser(&self) -> [u8; 5] {
        let mut data = vec!(18);
        data.extend_from_slice(&self.map_size.to_le_bytes());

        let mut res = [0; 5];
        res.copy_from_slice(&data[..5]);

        res
    }
}

pub struct MapChuck {
    pub map_data: u8,
}

impl Packet<2> for MapChuck {
    fn ser(&self) -> [u8; 2] {
        [19, self.map_data]
    }
}

// TODO IMPLEMENT CTFState or TCState
pub struct StateData<'a> {
    pub player_id: u8,
    pub fog_color: Color,
    pub team1: &'a Team,
    pub team2: &'a Team,
    pub gamemode: u8,
}

impl Packet<52> for StateData<'_> {
    fn ser(&self) -> [u8; 52] {
        let mut data = vec!(
            15,
            self.player_id, 
        );

        data.extend_from_slice(&self.fog_color.ser());
        data.extend_from_slice(&self.team1.color.ser());
        data.extend_from_slice(&self.team2.color.ser());
        data.extend_from_slice(self.team1.name.as_bytes());
        data.extend_from_slice(self.team2.name.as_bytes());
        data.push(self.gamemode);

        data.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let mut res = [0; 52];
        res.copy_from_slice(&data[..52]);

        res
    }
}