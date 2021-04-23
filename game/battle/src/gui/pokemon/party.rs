use game::{
    pokedex::pokemon::texture::PokemonTexture::Icon,
    gui::party::{PartyGui, PartyGuiData},
    textures::pokemon_texture,
};

use crate::pokemon::BattleParty;

pub fn battle_party_gui(party_gui: &mut PartyGui, party: &BattleParty) {
    party_gui.on_spawn(Some(false));
    party_gui.pokemon = party.pokemon.iter().map(|pokemon| &pokemon.pokemon).map(|pokemon| {
        
        let mut types = Vec::with_capacity(if pokemon.pokemon.data.secondary_type.is_some() { 2 } else { 1 });

        types.push(game::gui::party::pokemon_type_display(pokemon.pokemon.data.primary_type));

        if let Some(secondary) = pokemon.pokemon.data.secondary_type {
            types.push(game::gui::party::pokemon_type_display(secondary));
        }

        PartyGuiData {
            id: pokemon.pokemon.data.id,
            name: pokemon.name(),
            level: format!("Lv{}", pokemon.data.level),
            hp: format!("{}/{}", pokemon.current_hp, pokemon.base.hp),
            types,
            item: pokemon.item.as_ref().map(|(_, item)| item.name.to_ascii_uppercase()).unwrap_or("NONE".to_owned()),
            health_width: game::gui::health_bar::HealthBar::get_hp_width(pokemon.current_hp, pokemon.base.hp),
            texture: pokemon_texture(&pokemon.pokemon.data.id, Icon),
        }

    }).collect();
}