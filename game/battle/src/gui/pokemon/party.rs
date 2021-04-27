use game::gui::pokemon::PokemonDisplay;
use game::gui::party::PartyGui;

use crate::pokemon::BattleParty;

pub fn battle_party_gui(party_gui: &mut PartyGui, party: &BattleParty) {
    party_gui.spawn(party.pokemon.iter().map(|pokemon| PokemonDisplay::new(pokemon.pokemon.clone())).collect(), Some(false));
}