use crate::{
    deps::vec::ArrayVec,
    pokedex::pokemon::{Level, instance::PokemonInstance},
    tetra::Context,
    battle::pokemon::view::{BattlePartyKnown, BattlePartyUnknown, PokemonView}
};

use crate::battle::ui::{BattleGuiPosition, BattleGuiPositionIndex, pokemon::{PokemonRenderer, status::PokemonStatusGui}};

pub type ActiveRenderer = ArrayVec<[ActivePokemonRenderer; 3]>;

#[derive(Default)]
pub struct ActivePokemonParty<T> {
    pub party: T,
    pub renderer: ActiveRenderer,
}

// use crate::battle::pokemon::view::BattlePartyTrait;

// impl<T: BattlePartyTrait> BattlePartyTrait for ActivePokemonParty<T> {
//     fn id(&self) -> &pokemon_firered_clone_storage::player::PlayerId {
//         self.party.id()
//     }

//     fn active(&self, active: usize) -> Option<&dyn PokemonView> {
//         self.party.active(active)
//     }

//     fn active_mut(&mut self, active: usize) -> Option<&mut dyn PokemonView> {
//         self.party.active_mut(active)
//     }

//     fn active_len(&self) -> usize {
//         self.party.active_len()
//     }

//     fn len(&self) -> usize {
//         self.party.len()
//     }

//     fn active_eq(&self, active: usize, index: Option<usize>) -> bool {
//         self.party.active_eq(active, index)
//     }

//     fn pokemon(&self, index: usize) -> Option<&dyn PokemonView> {
//         self.party.pokemon(index)
//     }

//     fn add(&mut self, index: usize, unknown: firecore_battle::pokemon::view::PokemonUnknown) {
//         self.party.add(index, unknown)
//         // update gui here
//     }

//     fn replace(&mut self, active: usize, new: Option<usize>) {
//         self.party.replace(active, new)
//         self.renderer.update
//     }

//     fn any_inactive(&self) -> bool {
//         todo!()
//     }
// }

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
            let pokemon = (*index).map(|index| party.pokemon[index].as_ref()).flatten();
            Self {
                status: PokemonStatusGui::with_unknown(ctx, position, pokemon),
                renderer: PokemonRenderer::with(ctx, position, pokemon.map(|pokemon| *pokemon.pokemon().id()).as_ref(), pokedex::texture::PokemonTexture::Front),
            }
        }).collect()
    }

    pub fn update(&mut self, pokemon: Option<&dyn PokemonView>) {
        self.update_status(pokemon, true);
        self.renderer.new_pokemon(pokemon.map(|pokemon| *pokemon.pokemon().id()).as_ref());
    }
    
    pub fn update_status(&mut self, pokemon: Option<&dyn PokemonView>, reset: bool) {
        self.status.update_gui(pokemon, reset);
    }
    
    pub fn update_status_with_level(&mut self, pokemon: Option<&PokemonInstance>, level: Level, reset: bool) {
        self.status.update_gui_ex(pokemon.map(|i| (level, i as _)), reset)
    }

}