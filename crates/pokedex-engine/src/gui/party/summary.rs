use core::{cell::Cell, ops::Deref};

use crate::{
    get::GetPokemonData,
    gui::{cellref, party::PartyCell, pokemon::PokemonTypeDisplay, SizedStr, LEVEL_PREFIX},
    texture::PokemonTexture::Front,
    PokedexClientData,
};

use engine::{
    graphics::{
        draw_circle, draw_rectangle, draw_straight_line, draw_text_center, draw_text_left, Color,
        DrawParams, Texture,
    },
    gui::Panel,
    controls::{pressed, Control},
    text::MessagePage,
    utils::WIDTH,
    Context,
};
use firecore_engine::EngineContext;

use crate::pokedex::{pokemon::Pokemon, Dex};

use super::PartyError;

pub struct SummaryGui {
    pub alive: Cell<bool>,

    page: Cell<usize>,
    headers: [&'static str; Self::PAGES],
    pages: [Texture; Self::PAGES],
    pokemon_background: Texture,

    offset: Offset,

    pokemon: Cell<Option<SummaryPokemon>>,
}

#[derive(Default)]
struct Offset {
    int: Cell<u8>,
    boolean: Cell<bool>,
    float: Cell<f32>,
}

impl SummaryGui {
    const PAGES: usize = 3;

    const HEADER_LEFT: Color = Color::rgb(224.0 / 255.0, 216.0 / 255.0, 152.0 / 255.0);
    const HEADER_LEFT_DARK: Color = Color::rgb(192.0 / 255.0, 184.0 / 255.0, 112.0 / 255.0);
    const HEADER_RIGHT: Color = Color::rgb(0.0, 120.0 / 255.0, 192.0 / 255.0);
    const HEADER_RIGHT_DARK: Color = Color::rgb(0.0, 72.0 / 255.0, 144.0 / 255.0);

    pub fn new(ctx: &PokedexClientData) -> Self {
        Self {
            alive: Default::default(),
            headers: ["POKEMON INFO", "POKEMON SKILLS", "KNOWN MOVES"],
            pages: ctx.party.summary.pages.clone(),
            offset: Default::default(),
            pokemon_background: ctx.party.summary.background.clone(),
            page: Default::default(),
            pokemon: Default::default(),
        }
    }

    pub fn input(&self, ctx: &Context, eng: &EngineContext) {
        let page = self.page.get();
        if pressed(ctx, eng, Control::Left) && page > 0 {
            self.page.set(page - 1);
        }
        if pressed(ctx, eng, Control::Right) && page < Self::PAGES - 1 {
            self.page.set(page + 1);
        }
        if pressed(ctx, eng, Control::B) {
            self.despawn();
        }
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext) {
        let current_page = self.page.get();
        let w = 114.0 + (current_page << 4) as f32;
        let rw = WIDTH - w;
        draw_rectangle(ctx, 0.0, 1.0, w, 15.0, Self::HEADER_LEFT);
        draw_rectangle(ctx, w, 1.0, rw, 16.0, Self::HEADER_RIGHT);
        draw_straight_line(ctx, 0.0, 16.5, w, true, 1.0, Self::HEADER_LEFT_DARK);
        draw_text_left(
            ctx,
            eng,
            &1,
            self.headers[current_page],
            5.0,
            1.0,
            DrawParams::color(MessagePage::WHITE),
        );
        for page in 0..Self::PAGES {
            let color = if current_page < page {
                Self::HEADER_RIGHT_DARK
            } else if current_page == page {
                Panel::BACKGROUND
            } else {
                Self::HEADER_LEFT_DARK
            };
            draw_circle(ctx, 106.0 + (page << 4) as f32, 9.0, 6.0, color);
        }
        if let Some(summary) = cellref(&self.pokemon) {
            self.pokemon_background
                .draw(ctx, 0.0, 17.0, DrawParams::default());
            summary.front.draw(
                ctx,
                28.0,
                summary.pos + self.offset.float.get(),
                DrawParams::default(),
            );
            draw_text_left(
                ctx,
                eng,
                &1,
                LEVEL_PREFIX,
                5.0,
                19.0,
                DrawParams::color(MessagePage::WHITE),
            );
            draw_text_left(
                ctx,
                eng,
                &1,
                &summary.level,
                15.0,
                19.0,
                DrawParams::color(MessagePage::WHITE),
            );
            draw_text_left(
                ctx,
                eng,
                &1,
                &summary.name,
                41.0,
                19.0,
                DrawParams::color(MessagePage::WHITE),
            );
            const TOP: f32 = 17.0;
            match self.page.get() {
                0 => {
                    self.pages[0].draw(ctx, 0.0, TOP, Default::default());
                    draw_text_left(
                        ctx,
                        eng,
                        &1,
                        &summary.id,
                        168.0,
                        21.0,
                        DrawParams::color(MessagePage::BLACK),
                    );
                    draw_text_left(
                        ctx,
                        eng,
                        &1,
                        &summary.name,
                        168.0,
                        36.0,
                        DrawParams::color(MessagePage::BLACK),
                    );

                    for (index, display) in summary.types.iter().flatten().enumerate() {
                        let x = 168.0 + 37.0 * index as f32;
                        draw_rectangle(ctx, x, 52.0, 32.0, 6.0, display.upper);
                        draw_rectangle(ctx, x, 58.0, 32.0, 6.0, display.lower);
                        draw_text_center(
                            ctx,
                            eng,
                            &0,
                            display.name,
                            false,
                            x + 16.0,
                            52.0,
                            DrawParams::color(MessagePage::WHITE),
                        )
                    }

                    // draw_text_left(1, &pokemon.item, &crate::TEXT_BLACK, 168.0, 96.0);
                }
                1 => {
                    self.pages[1].draw(ctx, 0.0, TOP, Default::default());
                }
                2 => {
                    self.pages[2].draw(ctx, 119.0, TOP, Default::default());
                }
                _ => unreachable!(),
            }
        }
    }

    pub fn update(&self, delta: f32) {
        let int = self.offset.int.get();
        if int < 2 {
            let float = self.offset.float.get();
            match self.offset.boolean.get() {
                false => {
                    self.offset.float.set(float - delta * 120.0);
                    if float < -10.0 {
                        self.offset.boolean.set(true);
                    }
                }
                true => {
                    self.offset.float.set(float + delta * 120.0);
                    if float > 0.0 {
                        self.offset.int.set(int + 1);
                        self.offset.boolean.set(false);
                    }
                }
            }
        }
    }

    pub fn spawn<'d, P: Deref<Target = Pokemon>, I: GetPokemonData>(
        &self,
        ctx: &PokedexClientData,
        pokedex: &'d dyn Dex<'d, Pokemon, P>,
        pokemon: &I,
        cell: &PartyCell,
    ) {
        match SummaryPokemon::new(ctx, pokedex, pokemon, cell) {
            Ok(pokemon) => {
                self.alive.set(true);
                self.offset.int.set(Default::default());
                self.offset.boolean.set(Default::default());
                self.offset.float.set(Default::default());
                self.pokemon.set(Some(pokemon))
            }
            Err(err) => {
                engine::log::error!("Cannot create summary gui pokemon with error: {}", err)
            }
        }
    }

    pub fn despawn(&self) {
        self.alive.set(false)
    }

    pub fn alive(&self) -> bool {
        self.alive.get()
    }
}

struct SummaryPokemon {
    id: SizedStr<4>, // id and name
    name: String,
    front: Texture,
    pos: f32, // texture and pos
    types: [Option<PokemonTypeDisplay>; 2],
    level: SizedStr<4>,
    // health: CellHealth,
    // item: String,
}

impl SummaryPokemon {
    pub fn new<'d, P: Deref<Target = Pokemon>, I: GetPokemonData>(
        ctx: &PokedexClientData,
        pokedex: &'d dyn Dex<'d, Pokemon, P>,
        instance: &I,
        cell: &PartyCell,
    ) -> Result<Self, PartyError> {
        let pokemon = pokedex
            .try_get(instance.pokemon_id())
            .ok_or(PartyError::MissingPokemon)?;
        let texture = ctx
            .pokemon_textures
            .get(&pokemon.id, Front)
            .ok_or(PartyError::MissingTexture)?;
        Ok(Self {
            id: SizedStr::new(pokemon.id)?,
            name: instance.name().unwrap_or_else(|| pokemon.name.as_ref()).to_owned(),
            front: texture.clone(),
            types: [
                Some(PokemonTypeDisplay::new(pokemon.types.primary)),
                pokemon.types.secondary.map(PokemonTypeDisplay::new),
            ],
            pos: 34.0 + (64.0 - texture.height() as f32) / 2.0,
            level: cell.level.clone(),
            // health: cell.health.clone(),
        })
    }
}
