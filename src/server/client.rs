use crate::server::packets::ExistingPlayer;

#[derive(Debug)]
pub struct Client {
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

impl std::convert::From<&mut Client> for ExistingPlayer {
    fn from(client: &mut Client) -> Self {
        Self {
            player_id: client.player_id,
            team: client.team,
            weapon: client.weapon,
            held_item: client.held_item,
            kills: client.kills,
            blue: client.blue,
            green: client.green,
            red: client.red,
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
            blue: player.blue,
            green: player.green,
            red: player.red,
            name: player.name
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