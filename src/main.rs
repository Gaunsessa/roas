#![allow(incomplete_features)]
#![feature(box_syntax)]

#[macro_use]
extern crate aos_packet_derive;

// mod master;
mod color;
// mod map;
mod server;
use server::Server;

fn main() {
    let mut server = Server::new().unwrap();
    
    loop {
        server.service().unwrap();
    }

    // let yes = map::Map::empty();
    // println!("{}", std::mem::size_of::<[[map::VoxelColumn; 512]; 512]>());
    // println!("{}", std::mem::size_of::<[server::packets::PlayerPOS; 32]>());
}