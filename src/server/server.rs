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
    clients: Vec<Client>,
    map: std::fs::File,

    team1: Team,
    team2: Team,
}

impl Server {
    pub fn new() -> Result<Self, ServerError> {
        neg_to_err(unsafe { enet::enet_initialize() }, ServerError::InitFaild)?;
        let host = unsafe {
            enet::enet_host_create(
                &enet::ENetAddress {
                    host: u32::from_be_bytes([127, 0, 0, 1]).to_be(),
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
            clients: Vec::new(),
            map: std::fs::File::open("0048.vxl").unwrap(),

            team1: Team::new("Blue      ", Color::new(0, 0, 255)),
            team2: Team::new("Sex       ", Color::new(255, 0, 0)),
        })
    }

    pub fn service(&mut self) -> Result<(), ServerError> {
        let mut event: enet::ENetEvent = unsafe { std::mem::zeroed() };

        // neg_to_err(unsafe { enet_sys::enet_host_service(self.host, &mut event, 1000) }, ServerError::ServiceFaild);
        // TODO Fix it erroring for one tick after someone connects
        // println!("{}", unsafe { enet_sys::enet_host_service(self.host, &mut event, 1000) });
        unsafe { enet_sys::enet_host_service(self.host, &mut event, 1000) };

        match event.type_ {
            ENET_NONE => {},
            ENET_CONNECT => {
                println!("Connection");
                if event.data == 3 {
                    // let mut map = std::fs::File::open("testmap.vxl").unwrap();
                    self.clients.push(Client::new(event.peer));

                    self.send_map(self.clients.len() - 1)?;

                    self.send_packet(
                        StateData {
                            player_id: self.clients.len() as u8 - 1,
                            fog_color: Color::new(0, 0, 0),
                            team1: &self.team1,
                            team2: &self.team2,
                            gamemode: 0,
                        },
                        event.peer
                    )?;

                    println!("MAP SENT");

                    // self.send_packet(
                    //     PositionData {
                    //         x: 0.0,
                    //         y: 0.0,
                    //         z: 0.0
                    //     }, 
                    //     event.peer
                    // )?;
                    
                    // for client in self.clients.iter() {
                    //     println!("{:?}", unsafe { (*client.inner).address.host });
                    // }

                    println!("{:?}", self.clients);
                } else {
                    unsafe {
                        enet::enet_peer_disconnect(event.peer, 3)
                    }
                }
            },
            ENET_DISCONNECT => {
                println!("Disconnection");
            },
            ENET_RECEIVE => {
                let data: &[u8] = unsafe { std::slice::from_raw_parts((*event.packet).data, (*event.packet).dataLength) };

                println!("Packet Recived:\n    ID: {}",  data[0]);

                if data[0] == 9 {
                    let player_data = ExistingPlayer::der(data);

                    self.send_packet(
                        CreatePlayer {
                            player_id: player_data.player_id,
                            weapon: player_data.weapon,
                            team: player_data.team,
                            x: 0.0,
                            y: 50.0,
                            z: 0.0,
                            name: &player_data.name
                        }, 
                        event.peer
                    )?;

                    // self.send_packet(
                    //     PositionData {
                    //         x: 0.0,
                    //         y: 0.0,
                    //         z: 0.0
                    //     }, 
                    //     event.peer
                    // )?;
                }

                // TODO ERROR HANDLE THIS!!!
                unsafe { enet::enet_packet_destroy(event.packet) };
            },
            _ => unreachable!(),
        };

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

    fn send_map(&mut self, client_id: usize) -> Result<(), ServerError> {
        let mut encoder = zlib::Encoder::new(Vec::new())?;
        std::io::copy(&mut self.map, &mut encoder)?;
        let data = encoder.finish().into_result()?;

        self.send_packet(
            MapStart {
                map_size: data.len() as u32,
            }, 
            self.clients[client_id].inner
        )?;
        
        // TODO figure out how to send the world packets
        for b in data {
            self.send_packet(
                MapChuck {
                    map_data: b,
                }, 
                self.clients[client_id].inner
            )?;
        }

        Ok(())
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