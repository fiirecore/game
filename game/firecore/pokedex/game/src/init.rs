use deps::{
    hash::HashMap,
    tetra::{graphics::Texture, Context, Result},
    TextureManager,
};

use crate::{
    battle::BattleMovedex,
    Dex,
    item::Itemdex,
    moves::Movedex,
    pokemon::Pokedex,
    serialize::{SerializedDex, SerializedPokemon},
    texture::{ItemTextures, PokemonTextures, TrainerTextures},
};

pub fn init(ctx: &mut Context, dex: SerializedDex, callback: impl Fn(&mut SerializedPokemon)) -> Result {
    let mut pokedex = Pokedex::with_capacity(dex.pokemon.len());

    let mut pokemon_textures = PokemonTextures::with_capacity(dex.pokemon.len());

    // pokedex.insert(
    //     Pokemon::UNKNOWN,
    //     Pokemon {
    //         id: Pokemon::UNKNOWN,
    //         name: "Unknown".to_string(),
    //         primary_type: PokemonType::default(),
    //         secondary_type: None,
    //         base: Default::default(),
    //         data: PokedexData {
    //             species: "Unknown".to_string(),
    //             height: 0,
    //             weight: 0,
    //         },
    //         training: Training {
    //             base_exp: 0,
    //             growth_rate: Default::default(),
    //         },
    //         breeding: Breeding {
    //             gender: None,
    //         },
    //         moves: Vec::new(),
    //     }
    // );

    for mut pokemon in dex.pokemon {
        pokemon_textures.insert(ctx, &pokemon)?;

        #[cfg(feature = "audio")]
        (callback)(&mut pokemon);

        pokedex.insert(pokemon.pokemon.id, pokemon.pokemon);
    }

    Pokedex::set(pokedex);

    unsafe {
        crate::texture::POKEMON_TEXTURES = Some(pokemon_textures);
    }

    let mut movedex = Movedex::with_capacity(dex.moves.len());

    let mut battle_movedex = BattleMovedex::with_capacity(0);

    for serialized_move in dex.moves {
        let pmove = serialized_move.pokemon_move;
        if let Some(battle_move) = serialized_move.battle_move {
            battle_movedex.insert(pmove.id, battle_move.into(ctx));
        }
        movedex.insert(pmove.id, pmove);
    }

    Movedex::set(movedex);
    BattleMovedex::set(battle_movedex);

    let mut itemdex = Itemdex::with_capacity(dex.items.len());

    let mut item_textures = HashMap::with_capacity(dex.items.len());

    for item in dex.items {
        item_textures.insert(item.item.id, Texture::from_file_data(ctx, &item.texture)?);
        itemdex.insert(item.item.id, item.item);
    }

    TrainerTextures::set(
        dex.trainers
            .into_iter()
            .map(|(k, bytes)| (k, Texture::from_file_data(ctx, &bytes).unwrap()))
            .collect(),
    );

    Itemdex::set(itemdex);

    ItemTextures::set(item_textures);

    Ok(())
}
