#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(box_syntax)]

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
    // println!("{}", std::mem::size_of::<[[[map::Voxel; 64]; 512]; 512]>());
}