use crate::server::packets::*;
use crate::server::structs::*;
use crate::server::client::*;
use crate::color::Color;

use enet_sys as enet;
use enet::{
    ENetHost,
    ENetPeer,
    _ENetEventType_ENET_EVENT_TYPE_CONNECT as ENET_CONNECT,
    _ENetEventType_ENET_EVENT_TYPE_DISCONNECT as ENET_DISCONNECT,
    _ENetEventType_ENET_EVENT_TYPE_RECEIVE as ENET_RECEIVE,
    _ENetEventType_ENET_EVENT_TYPE_NONE as ENET_NONE,
};

use std::collections::HashMap;
use libflate::zlib;

#[derive(Debug)]
pub enum ServerError {
    InitFaild,
    ServiceFaild,
    RangeCoderFaild,
    HostFaild,
    PacketCreationFaild,
    PacketSendFaild,
    IOFaild,
    UnknownPlayerId,
}

impl std::convert::From<std::io::Error> for ServerError {
    fn from(_: std::io::Error) -> Self {
        ServerError::IOFaild
    }
}

/// AOS 0.75 server instance
/// 
/// SAFTEY: All usues of unsafe are checked for errors.
/// SAFTEY: This will call enet_initialize on creation and enet_deinitialize on drop.
pub struct Server {
    host: *mut ENetHost,
    clients: HashMap<*mut ENetPeer, Client>,
    map: std::fs::File,
    players: u8,

    team1: Team,
    team2: Team,
}

impl Server {
    pub fn new() -> Result<Self, ServerError> {
        neg_to_err(unsafe { enet::enet_initialize() }, ServerError::InitFaild)?;
        let host = unsafe {
            enet::enet_host_create(
                &enet::ENetAddress {
                    host: u32::from_be_bytes([0, 0, 0, 0]).to_be(),
                    port: 1273
                } as *const _,
                32,
                2,
                0,
                0
            )
        };

        if host.is_null() {
            return Err(ServerError::HostFaild)
        }
    
        neg_to_err(unsafe { enet::enet_host_compress_with_range_coder(host) }, ServerError::RangeCoderFaild)?;

        Ok(Self {
            host,
            clients: HashMap::new(),
            map: std::fs::File::open("testmap.vxl").unwrap(),
            players: 0,

            team1: Team::new("Blue      ", Color::new(0, 0, 255)),
            team2: Team::new("Yes       ", Color::new(255, 0, 0)),
        })
    }

    pub fn service(&mut self) -> Result<(), ServerError> {
        self.broudcast_packet(WorldUpdate::new(&self.clients))?;

        let mut event: enet::ENetEvent = unsafe { std::mem::zeroed() };

        // neg_to_err(unsafe { enet_sys::enet_host_service(self.host, &mut event, 1000) }, ServerError::ServiceFaild);
        // TODO Fix it erroring for one tick after someone connects
        // println!("{}", unsafe { enet_sys::enet_host_service(self.host, &mut event, 1000) });
        unsafe { enet_sys::enet_host_service(self.host, &mut event, 1) };
        let peer = event.peer;

        match event.type_ {
            ENET_NONE => {},
            ENET_CONNECT => {
                println!("Connection");
                if event.data == 3 {
                    self.send_map(peer)?;

                    for (_, client) in self.clients.iter() {
                        // println!("{:?}", client);

                        self.send_packet(ExistingPlayer::from(client), peer)?;
                    }

                    self.send_packet(
                        StateData {
                            player_id: self.players,
                            fog_color: Color::new(74, 74, 74),
                            team1: &self.team1,
                            team2: &self.team2,
                            gamemode: 0,
                        },
                        peer
                    )?;

                    println!("MAP SENT");
                    // println!("{:?}", peer);
                    // println!("{:?}", self.clients);
                } else {
                    unsafe {
                        enet::enet_peer_disconnect(peer, 3)
                    }
                }
            },
            ENET_DISCONNECT => {
                self.clients.remove(&peer);
                self.players -= 1;
                println!("Disconnection");
            },
            ENET_RECEIVE => {
                let data: &[u8] = unsafe { std::slice::from_raw_parts((*event.packet).data, (*event.packet).dataLength) };
                let client = self.clients.get_mut(&peer);
                if let None = client {
                    if let Packet::ExistingPlayer(data) = Packet::from(data) {
                        let client = Client::from(data.clone());
                        println!("{:?}", client);
                        self.clients.insert(peer, client);

                        self.broudcast_packet(
                            CreatePlayer {
                                player_id: data.player_id,
                                weapon: data.weapon,
                                team: data.team,
                                x: 0.0,
                                y: 50.0,
                                z: 0.0,
                                name: data.name.clone()
                            }
                        )?;

                        self.players += 1;
                    } else {
                        panic!("Client cant send packets without being initalised.")
                    }
                } else if let Some(client) = client {
                    match Packet::from(data) {
                        Packet::PositionData(data) => {
                            client.pos.px = data.x;
                            client.pos.py = data.y;
                            client.pos.pz = data.z;
                        },
                        Packet::OrientationData(data) => {
                            client.pos.ox = data.x;
                            client.pos.oy = data.y;
                            client.pos.oz = data.z;
                        },
                        Packet::BlockAction(data) => {
                            self.broudcast_packet(
                                data
                            )?;
                        },
                        Packet::ChatMessage(data) => {
                            self.broudcast_packet(
                                data
                            )?;
                        },
                        _ => {}//println!("{:?}", Packet::from(data))
                    }
                    // println!("{:?}", client);
                }

                // TODO ERROR HANDLE THIS!!!
                unsafe { enet::enet_packet_destroy(event.packet) };
            },
            _ => unreachable!(),
        };

        // self.broudcast_packet(WorldUpdate::new(&self.clients))?;

        // unsafe { enet::enet_host_flush(self.host) };

        Ok(())
    }

    fn send_packet(&self, packet: impl ServerPacket, peer: *mut ENetPeer) -> Result<(), ServerError> {
        let data = packet.ser().into_boxed_slice();

        let enet_packet = unsafe { enet::enet_packet_create((*data).as_ptr() as *const _, data.len(), enet::_ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE) };

        if enet_packet.is_null() {
            return Err(ServerError::PacketCreationFaild)
        }

        neg_to_err(unsafe { enet::enet_peer_send(peer, 0, enet_packet) }, ServerError::PacketSendFaild)?;

        Ok(())
    }

    fn send_map(&mut self, client: *mut ENetPeer) -> Result<(), ServerError> {
        // TODO FINISH THE MAP SO WE CAN SEND UPTO DATE MAPS INSTEAD OF RE READING THEM.
        let mut map = std::fs::File::open("testmap.vxl").unwrap();
        let mut encoder = zlib::Encoder::new(Vec::new())?;
        std::io::copy(&mut map, &mut encoder)?;
        let data = encoder.finish().into_result()?;

        self.send_packet(
            MapStart {
                map_size: data.len() as u32,
            }, 
            client
        )?;
        
        // TODO figure out how to send the world packets
        for b in data {
            self.send_packet(
                MapChuck {
                    map_data: b,
                }, 
                client
            )?;
        }

        Ok(())
    }

    fn broudcast_packet(&self, packet: impl ServerPacket + Clone) -> Result<(), ServerError> {
        for (i, _) in self.clients.iter() {
            self.send_packet(packet.clone(), *i)?;
        }

        Ok(())
    }

    fn get_player_from_id(&self, id: u8) -> Result<*mut ENetPeer, ServerError> {
        for (i, client) in self.clients.iter() {
            if client.player_id == id {
                return Ok(*i);
            }
        }

        Err(ServerError::UnknownPlayerId)
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        unsafe {
            enet::enet_host_destroy(self.host);
            enet::enet_deinitialize();
        }
    }
}

fn neg_to_err(data: i32, error: ServerError) -> Result<(), ServerError> {
    if data < 0 {
        Err(error)
    } else {
        Ok(())
    }
}