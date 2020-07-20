use enet_sys::ENetPeer;
use enet_sys::ENetHost;

pub struct Master {
    host: *mut ENetHost,
    inner: *mut ENetPeer,
}

// TODO FIX THE FUCKING MASTER SERVER AND ALSO FIGURE OUT THE CONNECTION PREDR

impl Master {
    pub fn new() -> Self {
        let host = unsafe { enet_sys::enet_host_create(std::ptr::null(), 1, 2, 0, 0) };

        // let mut address = unsafe { std::mem::zeroed() };
        // println!("{:?}", unsafe { enet_sys::enet_address_set_host(&mut address, "master.buildandshoot.com".as_ptr() as *const i8) });
        // address.port = 32886;

        let address = enet_sys::ENetAddress {
            host: u32::from_be_bytes([199, 195, 254, 202]).to_be(),
            port: 32886
        };

        let inner = unsafe { enet_sys::enet_host_connect(host, &address, 1, 31) };

        println!("{}", inner.is_null());

        Self {
            host,
            inner
        }
    }

    pub fn major_update(&mut self) {
        let mut packet: Vec<u8> = vec!(32);
        packet.append(&mut (32887 as u16).to_le_bytes().to_vec());
        packet.append(&mut "hamburger\0".as_bytes().to_vec());
        packet.append(&mut "ctf\0".as_bytes().to_vec());
        packet.append(&mut "chemes\0".as_bytes().to_vec());

        let yes = &packet[..];

        // let cvoid: *const std::ffi::c_void = &packet as *const _ as *const std::ffi::c_void;

        let enet_packet = unsafe { enet_sys::enet_packet_create(yes.as_ptr() as *const _, packet.len(), enet_sys::_ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE) };
        println!("{}", enet_packet.is_null());
        println!("{}", unsafe { enet_sys::enet_peer_send(self.inner, 0, enet_packet) });
    }
}