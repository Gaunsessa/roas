use crate::server::structs::*;
use crate::server::client::*;
use crate::color::Color;

use std::convert::TryInto;
use std::collections::HashMap;

use enet_sys::ENetPeer;

pub trait ServerPacket {
    fn ser(&self) -> Vec<u8>;
}

pub trait ClientPacket {
    fn der(packet: &[u8]) -> Self;
}

#[derive(Debug)]
pub enum Packet {
    PositionData(PositionData),
    OrientationData(OrientationData),
    WorldUpdate(), // TODO Put world update
    InputData(InputData),
    WeaponInput(WeaponInput),
    HitPacket(HitPacket),
    SetHP(),
    GrenadePacket(GrenadePacket),
    SetTool(SetTool),
    SetColor(SetColor),
    ExistingPlayer(ExistingPlayer),
    ShortPlayerData(),
    MoveObject(),
    CreatePlayer(),
    BlockAction(BlockAction),
    BlockLine(BlockLine),
    StateData(), // TODO Put state data
    KillAction(),
    ChatMessage(ChatMessage),
    MapStart(),
    MapChunk(),
    PlayerLeft(PlayerLeft),
    TerritoryCapture(TerritoryCapture),
    ProgressBar(),
    IntelCapture(IntelCapture),
    IntelPickup(IntelPickup),
    IntelDrop(IntelDrop),
    Restock(Restock),
    FogColor(),
    WeaponReload(WeaponReload),
    ChangeTeam(ChangeTeam),
    ChangeWeapon(ChangeWeapon),
    MapCached(MapCached),
}

impl Packet {
    pub fn from(data: &[u8]) -> Self {
        match data[0] {
            0 => Self::PositionData(PositionData::der(data)),
            1 => Self::OrientationData(OrientationData::der(data)),
            2 => panic!("Client should not send world update."),
            3 => Self::InputData(InputData::der(data)),
            4 => Self::WeaponInput(WeaponInput::der(data)),
            5 => Self::HitPacket(HitPacket::der(data)),
            6 => Self::GrenadePacket(GrenadePacket::der(data)),
            7 => Self::SetTool(SetTool::der(data)),
            8 => Self::SetColor(SetColor::der(data)),
            9 => Self::ExistingPlayer(ExistingPlayer::der(data)),
            10 => panic!("Client should not send short player data."),
            11 => panic!("Client should not send move object."),
            12 => panic!("Client should not send create player."),
            13 => Self::BlockAction(BlockAction::der(data)),
            14 => Self::BlockLine(BlockLine::der(data)),
            15 => panic!("Client should not send state data."),
            17 => Self::ChatMessage(ChatMessage::der(data)),
            18 => panic!("Client should not send killaction or map start."),
            19 => panic!("Client should not send map chunk."),
            20 => panic!("Client should not send player left."),
            21 => panic!("Client should not send territory capture."),
            22 => panic!("Client should not send progress bar."),
            23 => panic!("Client should not send intel capture."),
            24 => panic!("Client should not send intel pickup."),
            25 => panic!("Client should not send intel drop."),
            26 => panic!("Client should not send restock"),
            27 => panic!("Client should not send fog color."),
            28 => Self::WeaponReload(WeaponReload::der(data)),
            29 => Self::ChangeTeam(ChangeTeam::der(data)),
            30 => Self::ChangeWeapon(ChangeWeapon::der(data)),
            31 => Self::MapCached(MapCached::der(data)),
            _ => panic!("INVALID PACKET ID: {}", data[0])
        }
    }
}

#[derive_server_packet(0)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct PositionData {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive_server_packet(1)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct OrientationData {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug, Clone, Copy)]
pub struct WorldUpdate {
    players: [PlayerPOS; 32]
}

impl WorldUpdate {
    pub fn new(players: &HashMap<*mut ENetPeer, Client>) -> Self {
        let mut data = [PlayerPOS { px: 0.0, py: 0.0, pz: 0.0, ox: 0.0, oy: 0.0, oz: 0.0 }; 32];

        for (_, client) in players.iter() {
            data[client.player_id as usize] = client.pos;
        }

        Self {
            players: data
        }
    }
}

impl ServerPacket for WorldUpdate {
    fn ser(&self) -> Vec<u8> {
        let mut data = vec!(2);
        let _ = self.players.iter().map(|p| data.append(&mut p.ser())).collect::<()>();

        data
    }
}

impl ServerPacket for PlayerPOS {
    fn ser(&self) -> Vec<u8> {
        let mut data = vec!();
        data.extend_from_slice(&self.px.to_le_bytes());
        data.extend_from_slice(&self.py.to_le_bytes());
        data.extend_from_slice(&self.pz.to_le_bytes());
        data.extend_from_slice(&self.ox.to_le_bytes());
        data.extend_from_slice(&self.oy.to_le_bytes());
        data.extend_from_slice(&self.oz.to_le_bytes());

        data
    }
}

#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct InputData {
    pub player_id: u8,
    pub key_states: u8,
}

#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct WeaponInput {
    pub player_id: u8,
    pub weapon_input: u8,
}

#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct HitPacket {
    pub player_id: u8,
    pub hit_type: u8,
}

#[derive_server_packet(5)]
#[derive(Clone, Debug)]
pub struct SetHP {
    pub hp: u8,
    pub r#type: u8,
    pub source_x: f32,
    pub source_y: f32,
    pub source_z: f32
}

#[derive_server_packet(6)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct GrenadePacket {
    pub player_id: u8,
    pub fuse_length: f32,
    pub x_pos: f32,
    pub y_pos: f32,
    pub z_pos: f32,
    pub x_vel: f32,
    pub y_vel: f32,
    pub z_vel: f32
}

#[derive_server_packet(7)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct SetTool {
    pub player_id: u8,
    pub tool: u8
}

#[derive_server_packet(8)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct SetColor {
    pub player_id: u8,
    pub blue: u8,
    pub green: u8,
    pub red: u8
}

#[derive_server_packet(9)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct ExistingPlayer {
    pub player_id: u8,
    pub team: u8,
    pub weapon: u8,
    pub held_item: u8,
    pub kills: u32,
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub name: String
}

#[derive_server_packet(10)]
#[derive(Clone, Debug)]
pub struct ShortPlayerData {
    pub player_id: u8,
    pub team: u8,
    pub weapon: u8,
}

#[derive_server_packet(11)]
#[derive(Clone, Debug)]
pub struct MoveObject {
    pub object_id: u8,
    pub team: u8,
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive_server_packet(12)]
#[derive(Clone, Debug)]
pub struct CreatePlayer {
    pub player_id: u8,
    pub weapon: u8,
    pub team: u8,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub name: String
}

#[derive_server_packet(13)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct BlockAction {
    pub player_id: u8,
    pub action_type: u8,
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive_server_packet(14)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct BlockLine {
    pub player_id: u8,
    pub start_x: u32,
    pub start_y: u32,
    pub start_z: u32,
    pub end_x: u32,
    pub end_y: u32,
    pub end_z: u32,
}

pub struct StateData<'a> {
    pub player_id: u8,
    pub fog_color: Color,
    pub team1: &'a Team,
    pub team2: &'a Team,
    pub gamemode: u8,
}

impl ServerPacket for StateData<'_> {
    fn ser(&self) -> Vec<u8> {
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

        data
    }
}

#[derive_server_packet(18)]
#[derive(Clone, Debug)]
pub struct KillAction {
    pub player_id: u8,
    pub killer_id: u8,
    pub kill_type: u8,
    pub respawn_time: u8,
}

#[derive_server_packet(17)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub player_id: u8,
    pub chat_type: u8,
    pub chat_message: String,
}

#[derive_server_packet(18)]
#[derive(Clone, Debug)]
pub struct MapStart {
    pub map_size: u32,
}

#[derive_server_packet(19)]
#[derive(Clone, Debug)]
pub struct MapChuck {
    pub map_data: u8,
}

#[derive_server_packet(20)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct PlayerLeft {
    pub player_id: u8,
}

#[derive_server_packet(21)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct TerritoryCapture {
    pub player_id: u8,
    pub entity_id: u8,
    pub winning: u8,
    pub state: u8,
}

#[derive_server_packet(22)]
#[derive(Clone, Debug)]
pub struct ProgressBar {
    pub entity_id: u8,
    pub team_id: u8,
    pub rate: u8,
    pub progress: u8,
}

#[derive_server_packet(23)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct IntelCapture {
    pub player_id: u8,
    pub winning: u8,
}

#[derive_server_packet(24)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct IntelPickup {
    pub player_id: u8,
}

#[derive_server_packet(25)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct IntelDrop {
    pub player_id: u8,
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive_server_packet(26)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct Restock {
    pub player_id: u8,
}

#[derive_server_packet(27)]
#[derive(Clone, Debug)]
pub struct FogColor {
    pub fog_color: u32,
}

#[derive_server_packet(28)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct WeaponReload {
    pub player_id: u8,
    pub clip_ammo: u8,
    pub reserve_amma: u8,
}

#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct ChangeTeam {
    pub player_id: u8,
    pub team_id: u8,
}

#[derive_server_packet(30)]
#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct ChangeWeapon {
    pub player_id: u8,
    pub weapon_id: u8,
}

#[derive_client_packet(0)]
#[derive(Clone, Debug)]
pub struct MapCached {
    pub cached: u8,
}