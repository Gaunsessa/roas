use enet_sys::ENetPeer;

pub struct Client {
    pub inner: *mut ENetPeer,
}

impl Client {
    pub fn new(enet_peer: *mut ENetPeer) -> Self {
        Client {
            inner: enet_peer,
        }
    }
}