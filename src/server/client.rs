use crate::server::packets::ExistingPlayer;
use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Client {
    pub player_id: u8,
    pub name: String,
    pub team: u8,
    pub kills: u32,
    pub weapon: u8,
    pub held_item: u8,
    pub color: Color,
    pub pos: PlayerPOS,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerPOS {
    pub px: f32,
    pub py: f32,
    pub pz: f32,
    pub ox: f32,
    pub oy: f32,
    pub oz: f32,
}

impl PlayerPOS {
    fn empty() -> Self {
        Self {
            px: 0.0,
            py: 0.0,
            pz: 0.0,
            ox: 0.0,
            oy: 0.0,
            oz: 0.0
        }
    }
}

impl std::convert::From<&Client> for ExistingPlayer {
    fn from(client: &Client) -> Self {
        Self {
            player_id: client.player_id,
            team: client.team,
            weapon: client.weapon,
            held_item: client.held_item,
            kills: client.kills,
            blue: client.color.b,
            green: client.color.g,
            red: client.color.r,
            name: client.name.clone()
        }
    }
}

impl std::convert::From<ExistingPlayer> for Client {
    fn from(player: ExistingPlayer) -> Self {
        Self {
            player_id: player.player_id,
            team: player.team,
            weapon: player.weapon,
            held_item: player.held_item,
            kills: player.kills,
            color: Color::new(player.red, player.green, player.blue),
            name: player.name,
            pos: PlayerPOS::empty()
        }
    }
}

impl Client {
    // pub fn new(name: &str) -> Self {
    //     Client {
    //         // inner: enet_peer,
    //         name: name.to_string()
    //     }
    // }
}