use firecore_util::Coordinate;
use firecore_util::Destination;
use firecore_util::Direction;
use firecore_util::Position;
use firecore_util::TinyStr16;
use serde::{Deserialize, Serialize};

pub type SessionToken = u32;

pub type PlayerId = u16;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {

    Connect,
    Join(FullPlayerData),
    Disconnect,

    MovePlayer(PlayerPos),
    Warp((Option<TinyStr16>, TinyStr16, Destination)),
    
}

/// Messages that Server sends to Client
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    // Version
    // Version(String, Compatibility),

    // Server info
    // StaticServerInfo(ServerInfo),
    // DynamicServerInfo(Vec<char>), //player list

    // Login messages
    // LoginStatus(LoginStatus), //player, status
    Connect(PlayerId),

    MapPlayers(crate::hash::HashMap<PlayerId, PlayerData>),

    // Udp handshake
    // UdpConnected,

    // Game messages
    SpawnPlayer(PlayerId, PlayerData),
    MovePlayer(PlayerId, PlayerPos),
    DespawnPlayer(PlayerId),

    // Arena messages
    // WaitArena(Duration),
    // StartArena(ArenaInfo),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum LoggedKind {
    FirstTime,
    Reconnection,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum LoginStatus {
    Logged(SessionToken, LoggedKind),
    // InvalidPlayerName,
    AlreadyLogged,
    PlayerLimit,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ServerInfo {
//     pub udp_port: u16,
//     pub players_number: u8,
//     pub map_size: u16,
//     pub winner_points: u16,
//     pub logged_players: Vec<char>,
// }

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct FullPlayerData {
    pub map: Option<TinyStr16>,
    pub index: TinyStr16,
    pub data: PlayerData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PlayerData {
    pub pos: PlayerPos,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PlayerPos {
    pub coords: Coordinate,
    pub direction: Direction,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct GameInfo {
//     // pub characters: Vec<Character>,
//     // pub players: Vec<(CharacterId, usize)>, //id, points
// }

impl From<Position> for PlayerPos {
    fn from(pos: Position) -> Self {
        Self {
            coords: pos.coords,
            direction: pos.direction,
        }
    }
}