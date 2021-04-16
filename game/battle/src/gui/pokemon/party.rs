use game::{
    pokedex::pokemon::texture::PokemonTexture::Icon,
    gui::party::{PokemonPartyGui, PartyGuiData},
    textures::pokemon_texture,
};

use crate::pokemon::BattleParty;

pub fn battle_party_gui(party_gui: &mut PokemonPartyGui, party: &BattleParty) {
    party_gui.on_spawn(false);
    for pokemon in party.pokemon.iter().map(|pokemon| &pokemon.pokemon){
        party_gui.pokemon.push(PartyGuiData {
            id: pokemon.pokemon.data.id,
            name: pokemon.name(),
            level: format!("Lv{}", pokemon.data.level),
            hp: format!("{}/{}", pokemon.current_hp, pokemon.base.hp),
            health_width: (pokemon.current_hp as f32 / pokemon.base.hp as f32).ceil() * 48.0,
            texture: pokemon_texture(&pokemon.pokemon.data.id, Icon),
        });
    }
}