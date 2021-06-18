use std::path::PathBuf;
use std::fs::read;

use deps::tetra::{Context, graphics::Texture};
use firecore_pokedex::moves::MoveId;
use serde::{Deserialize, Serialize};

use super::{BattleMove, script::BattleActionScript};

#[derive(Deserialize, Serialize)]
pub struct SerializedBattleMove<T> {
    pub id: MoveId,
    pub texture: Option<T>,
    pub script: BattleActionScript,
}

pub type SerializedBattleMoveFile = SerializedBattleMove<String>;
pub type SerializedBattleMoveBytes = SerializedBattleMove<Vec<u8>>;

impl SerializedBattleMoveFile {
    pub fn into(self, dir: PathBuf) -> SerializedBattleMove<Vec<u8>> {
        let texture = self.texture.map(|path| dir.join(path));
        let texture = if let Some(texture) = texture {
            match read(texture) {
                Ok(bytes) => Some(bytes),
                Err(err) => panic!("Could not read battle texture file for {} with error {}", self.id, err),
            }
        } else {
            None
        };
        let id = self.id;
        let script = self.script;
        SerializedBattleMoveBytes {
            id,
            texture,
            script,
        }
    }
}


impl SerializedBattleMoveBytes {
    pub fn into(self, ctx: &mut Context) -> BattleMove {
        BattleMove {
            id: self.id,
            texture: self.texture.map(|bytes| Texture::from_file_data(ctx, &bytes).ok()).flatten(),
            script: self.script,
        }
    }
}