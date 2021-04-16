pub mod connect_scene;

use firecore_game::client::ClientSocketTrait;
use firecore_game::client::MessageSender;
use firecore_game::network::message::PlayerData;
use firecore_game::network::message::PlayerId;
use firecore_game::util::Position;

pub(crate) static mut SERVER: Option<Server> = None;

pub struct Server {

    pub socket: Box<dyn ClientSocketTrait>,
    pub sender: MessageSender,
    pub id: Option<PlayerId>,

}

pub struct LocalPlayerData {
    pub pos: Position,
}

impl From<PlayerData> for LocalPlayerData {
    fn from(data: PlayerData) -> Self {
        Self {
            pos: Position {
                coords: data.pos.coords,
                direction: data.pos.direction,
                ..Default::default()
            }
        }
    }
}