use game::{
    graphics::ZERO,
    pokedex::{
        battle::{
            party::knowable::{BattlePartyKnown, BattlePartyUnknown},
            view::PokemonView,
        },
        pokemon::{instance::PokemonInstance, Level},
        texture::PokemonTexture,
    },
    tetra::{graphics::Color, Context},
};

use super::{
    pokemon::{flicker::Flicker, PokemonRenderer, PokemonStatusGui},
    BattleGuiPosition, BattleGuiPositionIndex,
};

pub type ActiveRenderer = Vec<ActivePokemonRenderer>;

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
    pub fn init_known<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord>(
        ctx: &mut Context,
        party: &BattlePartyKnown<ID>,
    ) -> ActiveRenderer {
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
                        pokemon.map(|pokemon| *pokemon.pokemon.id()),
                        PokemonTexture::Back,
                    ),
                    status: PokemonStatusGui::with_known(ctx, position, pokemon),
                }
            })
            .collect()
    }

    pub fn init_unknown<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord>(
        ctx: &mut Context,
        party: &BattlePartyUnknown<ID>,
    ) -> ActiveRenderer {
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
                        pokemon.map(|pokemon| *pokemon.pokemon().id()),
                        PokemonTexture::Front,
                    ),
                    status: PokemonStatusGui::with_unknown(ctx, position, pokemon),
                }
            })
            .collect()
    }

    pub fn update(&mut self, pokemon: Option<&dyn PokemonView>) {
        self.update_status(pokemon, true);
        self.renderer
            .new_pokemon(pokemon.map(|pokemon| *pokemon.pokemon().id()));
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
        self.renderer.draw(ctx, ZERO, Color::WHITE);
        self.status.draw(
            ctx,
            0.0,
            if self.renderer.flicker.accumulator % Flicker::HALF > Flicker::HALF / 8.0
                && self.renderer.flicker.remaining > (Flicker::TIMES >> 1)
            {
                0.0
            } else {
                1.0
            },
        );
        self.renderer.moves.draw(ctx);
    }
}
