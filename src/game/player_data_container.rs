// use crate::gui::game::pokemon_party_gui::PokemonPartyGui;
use crate::io::data::player_data::PlayerData;
use crate::io::data::pokemon::saved_pokemon::SavedPokemon;
use crate::util::Load;
use crate::util::file::PersistantData;

use super::pokedex::pokemon::pokemon_owned::OwnedPokemon;

pub struct PlayerDataContainer {

    player_data: PlayerData,

    //party_gui: PokemonPartyGui,

}

impl PlayerDataContainer {

    pub fn new(player_data: PlayerData) -> Self {

        Self {

            player_data: player_data,

            //party_gui: PokemonPartyGui::new(),

        }

    }

    pub fn get(&self) -> &PlayerData {
        &self.player_data
    }

    pub fn get_mut(&mut self) -> &mut PlayerData {
        &mut self.player_data
    }

    pub fn save(&mut self) {
        self.player_data.save();
    }

    pub fn get_pokemon(&self, index: usize) -> Option<&SavedPokemon> {
        return if index < self.player_data.party.pokemon.len() {
            Some(&self.player_data.party.pokemon[index])
        } else {
            None
        };        
    }

    pub fn save_pokemon(&mut self, index: usize, pokemon: OwnedPokemon) {
        if index < self.player_data.party.pokemon.len() {
            self.player_data.party.pokemon[index] = SavedPokemon::from_owned_pokemon(pokemon);
        }
    }

}

impl Load for PlayerDataContainer {

    fn load(&mut self) {
        //self.party_gui.load();
    }

}