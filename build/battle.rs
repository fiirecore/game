use std::{path::Path, fs::{read_dir, read_to_string, read}};

use firecore_dex_gen::{Client, moves::Execution};
use firecore_pokedex_engine_core::pokedex::moves::MoveId;

use crate::{readable, write};

pub fn build(root: impl AsRef<Path>, assets: &Path, client: Client) {

    if readable::<Execution, _>(&root, "battle_moves").is_none() {
        write(
            &root,
            "battle_moves",
            firecore_dex_gen::moves::generate_battle(client, crate::MOVES).unwrap(),
        );
    }

    if readable::<Scripts, _>(&root, "battle_move_scripts").is_none() {
        write(
            &root,
            "battle_move_scripts",
            get_moves(assets.join("battle/moves/scripts")),
        );
    }

    if readable::<BattleGuiTextures, _>(&root, "battle_gui").is_none() {
        write(
            &root,
            "battle_gui",
            battle_engine("crates/battle-engine/assets").unwrap(),
        );
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BattleGuiTextures<T = Vec<u8>> {
    pub background: T,
    // pub panel: T,
    pub ground: T,
    pub pokeball: T,
    // pub smallui: T,
    // pub padding: T,
    // pub largeui: T,
    pub player: T,
    pub grass: T,
    pub bar: T,
    // pub ball: T,
}

type Scripts = std::collections::HashMap<MoveId, String>;

fn get_moves<P: AsRef<Path>>(scripts: P) -> Scripts {
    let scripts = scripts.as_ref();

    read_dir(scripts)
        .unwrap_or_else(|err| {
            panic!(
                "Could not read move scripts directory at {:?} with error {}",
                scripts, err
            )
        })
        .flatten()
        .map(|d| d.path())
        .filter(|p| p.is_file())
        .map(|path| {
            (
                path.file_stem()
                    .unwrap_or_else(|| {
                        panic!("Could not get file name for script file at path {:?}", path,)
                    })
                    .to_string_lossy()
                    .parse::<MoveId>()
                    .unwrap_or_else(|err| {
                        panic!(
                            "Could not parse move script file {:?} into MoveScriptId with error {}",
                            path, err
                        )
                    }),
                read_to_string(&path).unwrap_or_else(|err| {
                    panic!(
                        "Could not read move file at {:?} to string with error {}",
                        path, err
                    )
                }),
            )
        })
        .collect()
}

fn battle_engine(path: impl AsRef<Path>) -> Result<BattleGuiTextures, std::io::Error> {
    let path = path.as_ref();
    Ok(BattleGuiTextures {
        background: read(path.join("background.png"))?,
        ground: read(path.join("ground.png"))?,
        pokeball: read(path.join("pokeball.png"))?,
        player: read(path.join("player.png"))?,
        grass: read(path.join("grass.png"))?,
        bar: read(path.join("gui/bar.png"))?,
    })
}