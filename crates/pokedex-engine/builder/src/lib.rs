pub extern crate enum_map;
pub extern crate firecore_pokedex as pokedex;
pub extern crate tinystr;

use serde::{Deserialize, Serialize};

use self::{item::ItemOutput, npc_group::NpcGroupOutput, pokemon::PokemonOutput};

// pub mod battle;
pub mod item;
pub mod npc_group;
pub mod pokemon;

#[derive(Deserialize, Serialize)]
pub struct SerializedPokedexEngine {
    pub pokemon: PokemonOutput,
    pub items: ItemOutput,
    pub npc_groups: NpcGroupOutput,
}

#[cfg(feature = "compile")]
pub fn compile(
    pokemon: impl AsRef<std::path::Path>,
    items: impl AsRef<std::path::Path>,
    npc_groups: impl AsRef<std::path::Path>,
) -> SerializedPokedexEngine {
    println!("Loading pokemon...");
    let pokemon = pokemon::get_pokemon(pokemon);
    println!("Loading items...");
    let items = item::get_items(items);
    println!("Loading {}s...", npc_group::NAME);
    let npc_groups = npc_group::get_npc_groups(npc_groups);
    println!("Done!");
    SerializedPokedexEngine {
        pokemon,
        items,
        npc_groups,
    }
    // let battle_moves = match battle {
    //     Some(battle) => {
    //         println!("Loading battle moves...");
    //         battle::get_battle_moves(battle)
    //     }
    //     None => Vec::new(),
    // };
}
