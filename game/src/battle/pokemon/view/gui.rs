use crate::{
    deps::vec::ArrayVec,
    pokedex::pokemon::{Level, instance::PokemonInstance},
    tetra::Context,
};

use crate::battle::ui::{BattleGuiPosition, BattleGuiPositionIndex, pokemon::{PokemonRenderer, status::PokemonStatusGui}};

use super::{BattlePartyKnown, BattlePartyUnknown, PokemonKnowData};

pub type ActiveRenderer = ArrayVec<[ActivePokemonRenderer; 3]>;

pub struct ActivePokemonParty<T> {
    pub party: T,
    pub renderer: ActiveRenderer,
}

pub struct ActivePokemonRenderer {
    // pub pokemon: PokemonOption,
    pub status: PokemonStatusGui,
    pub renderer: PokemonRenderer,
    // pub last_move: Option<(usize, usize)>, // previous cursor pos
}

impl ActivePokemonRenderer {

    pub fn init_known(ctx: &mut Context, party: &BattlePartyKnown) -> ActiveRenderer {
        let size = party.active.len() as u8;
        party.active.iter().enumerate().map(|(i, index)| {
            let position = BattleGuiPositionIndex::new(BattleGuiPosition::Bottom, i as u8, size);
            let pokemon = (*index).map(|index| &party.pokemon[index]);
            Self {
                status: PokemonStatusGui::with_known(ctx, position, pokemon),
                renderer: PokemonRenderer::with(ctx, position, pokemon.map(|pokemon| pokemon.pokemon.id()), pokedex::texture::PokemonTexture::Back),
            }
        }).collect()
    }

    pub fn init_unknown(ctx: &mut Context, party: &BattlePartyUnknown) -> ActiveRenderer {
        let size = party.active.len() as u8;
        party.active.iter().enumerate().map(|(i, index)| {
            let position = BattleGuiPositionIndex::new(BattleGuiPosition::Top, i as u8, size);
            let pokemon = (*index).map(|index| party.pokemon[index]).flatten();
            Self {
                status: PokemonStatusGui::with_unknown(ctx, position, pokemon),
                renderer: PokemonRenderer::with(ctx, position, pokemon.map(|pokemon| *pokemon.pokemon.id()).as_ref(), pokedex::texture::PokemonTexture::Front),
            }
        }).collect()
    }

    pub fn update(&mut self, pokemon: Option<&dyn PokemonKnowData>) {
        self.update_status(pokemon, true);
        self.renderer.new_pokemon(pokemon.map(|pokemon| *pokemon.pokemon().id()).as_ref());
    }
    
    pub fn update_status(&mut self, pokemon: Option<&dyn PokemonKnowData>, reset: bool) {
        self.status.update_gui(pokemon, reset);
    }
    
    pub fn update_status_with_level(&mut self, pokemon: Option<&PokemonInstance>, level: Level, reset: bool) {
        self.status.update_gui_ex(pokemon.map(|i| (level, i as _)), reset)
    }

}