use enet_sys::ENetPeer;

#[derive(Debug)]
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