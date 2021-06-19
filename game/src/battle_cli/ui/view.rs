use crate::{
    deps::vec::ArrayVec,
    pokedex::pokemon::{instance::PokemonInstance, Level},
    tetra::Context,
};

use battle::pokemon::view::{BattlePartyKnown, BattlePartyUnknown, PokemonView};

use crate::battle_cli::ui::{
    pokemon::{PokemonRenderer, PokemonStatusGui},
    BattleGuiPosition, BattleGuiPositionIndex,
};

pub type ActiveRenderer = ArrayVec<[ActivePokemonRenderer; 3]>;

#[derive(Default)]
pub struct ActivePokemonParty<T> {
    pub party: T,
    pub renderer: ActiveRenderer,
}

pub struct ActivePokemonRenderer {
    pub renderer: PokemonRenderer,
    pub status: PokemonStatusGui,
}

impl ActivePokemonRenderer {
    pub fn init_known(ctx: &mut Context, party: &BattlePartyKnown) -> ActiveRenderer {
        let size = party.active.len() as u8;
        party
            .active
            .iter()
            .enumerate()
            .map(|(i, index)| {
                let position =
                    BattleGuiPositionIndex::new(BattleGuiPosition::Bottom, i as u8, size);
                let pokemon = (*index).map(|index| &party.pokemon[index]);
                Self {
                    renderer: PokemonRenderer::with(
                        ctx,
                        position,
                        pokemon.map(|pokemon| pokemon.pokemon.id()),
                        pokedex::texture::PokemonTexture::Back,
                    ),
                    status: PokemonStatusGui::with_known(ctx, position, pokemon),
                }
            })
            .collect()
    }

    pub fn init_unknown(ctx: &mut Context, party: &BattlePartyUnknown) -> ActiveRenderer {
        let size = party.active.len() as u8;
        party
            .active
            .iter()
            .enumerate()
            .map(|(i, index)| {
                let position = BattleGuiPositionIndex::new(BattleGuiPosition::Top, i as u8, size);
                let pokemon = (*index)
                    .map(|index| party.pokemon[index].as_ref())
                    .flatten();
                Self {
                    renderer: PokemonRenderer::with(
                        ctx,
                        position,
                        pokemon.map(|pokemon| *pokemon.pokemon().id()).as_ref(),
                        pokedex::texture::PokemonTexture::Front,
                    ),
                    status: PokemonStatusGui::with_unknown(ctx, position, pokemon),
                }
            })
            .collect()
    }

    pub fn update(&mut self, pokemon: Option<&dyn PokemonView>) {
        self.update_status(pokemon, true);
        self.renderer
            .new_pokemon(pokemon.map(|pokemon| *pokemon.pokemon().id()).as_ref());
    }

    pub fn update_status(&mut self, pokemon: Option<&dyn PokemonView>, reset: bool) {
        self.status.update_gui(pokemon, reset);
    }

    pub fn update_status_with_level(
        &mut self,
        pokemon: Option<&PokemonInstance>,
        level: Level,
        reset: bool,
    ) {
        self.status
            .update_gui_ex(pokemon.map(|i| (level, i as _)), reset)
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.renderer.draw(
            ctx,
            crate::graphics::ZERO,
            deps::tetra::graphics::Color::WHITE,
        );
        self.status.draw(ctx, 0.0, 0.0);
        self.renderer.moves.draw(ctx);
    }
}
