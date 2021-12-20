use std::path::PathBuf;

use battlelib::default_engine::{scripting::MoveScripts, EngineMoves};
use firecore_battle::pokedex::{item::Item, moves::Move, pokemon::Pokemon, BasicDex};
use firecore_battle_gui::pokedex::{
    engine::{fs, text::font::FontSheet, log::info},
    SerializedPokedexEngine,
};
use serde::de::DeserializeOwned;
use worldlib::serialized::SerializedWorld;

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
    pub audio: std::collections::HashMap<firecore_battle_gui::pokedex::engine::audio::MusicId, Vec<u8>>,
}

impl AssetContext {
    pub async fn load() -> Result<Self, DataError> {
        let path = storage::directory(false, crate::PUBLISHER, crate::APPLICATION)?.join("assets");

        async fn get<T: DeserializeOwned>(path: &PathBuf, file: &str) -> Result<T, DataError> {
            let bytes = fs::read(path.join(file)).await?;
            Ok(storage::from_bytes::<T>(&bytes)?)
        }

        let fonts = get(&path, "fonts.bin").await?;
        info!("Loading dexes...");
        let (pokedex, movedex, itemdex) = get(&path, "dex.bin").await?;

        info!("Loading battle data...");
        let battle = get(&path, "battle.bin").await?;
        
        let dex = get(&path, "dex_engine.bin").await?;
        let world = get(&path, "world.bin").await?;

        #[cfg(feature = "audio")]
        let audio = get(&path, "audio.bin").await?;

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
