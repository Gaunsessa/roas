use crate::server::structs::*;
use crate::color::Color;

use std::convert::TryInto;

pub trait ServerPacket<const X: usize> {
    fn ser(&self) -> [u8; X];
}

pub trait ClientPacket {
    fn der(packet: &[u8]) -> Self;
}

pub struct MapStart {
    pub map_size: u32,
}

impl ServerPacket<5> for MapStart {
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

impl ServerPacket<2> for MapChuck {
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

impl ServerPacket<104> for StateData<'_> {
    fn ser(&self) -> [u8; 104] {
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

        data.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let mut res = [0; 104];
        res.copy_from_slice(&data[..104]);

        res
    }
}

pub struct PositionData {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl ServerPacket<12> for PositionData {
    fn ser(&self) -> [u8; 12] {
        let mut data = vec!(0);

        data.extend_from_slice(&self.x.to_le_bytes());
        data.extend_from_slice(&self.y.to_le_bytes());
        data.extend_from_slice(&self.z.to_le_bytes());

        let mut res = [0; 12];
        res.copy_from_slice(&data[..12]);

        res
    }
}

#[derive(Debug)]
pub struct ExistingPlayer {
    player_id: u8,
    team: u8,
    weapon: u8,
    held_item: u8,
    kills: u32,
    blue: u8,
    green: u8,
    red: u8,
    name: String
}

impl ClientPacket for ExistingPlayer {
    fn der(packet: &[u8]) -> Self {
        Self {
            player_id: packet[1],
            team: packet[2],
            weapon: packet[3],
            held_item: packet[4],
            kills: u32::from_le_bytes(packet[5..9].try_into().unwrap()),
            blue: packet[9],
            green: packet[10],
            red: packet[11],
            name: packet[12..].iter().filter_map(|&x| if x != 0 { Some(x as char) } else { None }).collect::<String>()
        }
    }
}

pub struct CreatePlayer<'a> {
    player_id: u8,
    weapon: u8,
    team: u8,
    x: f32,
    y: f32,
    z: f32,
    name: &'a str
}

impl ServerPacket<16> for CreatePlayer<'_> {
    fn ser(&self) -> [u8; 16] {
        let mut data = vec!(
            12,
            self.player_id,
            self.weapon,
            self.team
        );

        data.extend_from_slice(&self.x.to_le_bytes());
        data.extend_from_slice(&self.y.to_le_bytes());
        data.extend_from_slice(&self.z.to_le_bytes());

        let mut res = [0; 16];
        res.copy_from_slice(&data[..16]);

        res
    }
}