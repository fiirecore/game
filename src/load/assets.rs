use crate::engine::{log::info, text::font::FontSheet};
use battlecli::{
    battle::{
        default_engine::{scripting::MoveScripts, EngineMoves},
        pokedex::{item::Item, moves::Move, pokemon::Pokemon, BasicDex},
    },
};
use crate::pokengine::SerializedPokedexEngine;
use worldcli::worldlib::serialized::SerializedWorld;

use crate::storage::{self, error::DataError};

pub struct AssetContext {
    pub fonts: Vec<FontSheet<Vec<u8>>>,
    pub pokedex: BasicDex<Pokemon>,
    pub movedex: BasicDex<Move>,
    pub itemdex: BasicDex<Item>,
    pub battle: (EngineMoves, MoveScripts),
    pub dex: SerializedPokedexEngine,
    pub world: SerializedWorld,
    #[cfg(feature = "audio")]
    pub audio: crate::engine::utils::HashMap<crate::engine::music::MusicId, Vec<u8>>,
}

impl AssetContext {
    pub async fn load() -> Result<Self, DataError> {
        let path = storage::directory(false, crate::PUBLISHER, crate::APPLICATION)?.join("assets");

        use storage::get;

        let fonts = get(path.join("fonts.bin")).await?;
        info!("Loading dexes...");
        let (pokedex, movedex, itemdex) = get(path.join("dex.bin")).await?;

        info!("Loading battle data...");
        let battle = get(path.join("battle.bin")).await?;

        let dex = get(path.join("dex_engine.bin")).await?;
        let world = get(path.join("world.bin")).await?;

        #[cfg(feature = "audio")]
        let audio = get(path.join("audio.bin")).await?;

        Ok(Self {
            fonts,
            dex,
            world,
            pokedex,
            movedex,
            itemdex,
            battle,
            #[cfg(feature = "audio")]
            audio,
        })
    }
}
