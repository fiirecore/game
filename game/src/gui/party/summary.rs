use pokedex::texture::{
    pokemon_texture,
    PokemonTexture::Front,
};

use crate::input::{pressed, Control};

use crate::text::TextColor;
use crate::graphics::{byte_texture, position, draw_text_left, draw_rectangle, draw_line, draw_circle};

use crate::tetra::{
    Context,
    graphics::{
        Color,
        Texture,
        DrawParams,
    },
};

use crate::gui::pokemon::{PokemonDisplay, PokemonTypeDisplay};

pub struct SummaryGui {

    pub alive: bool,

    page: usize,
    headers: [&'static str; Self::PAGES],
    pages: [Texture; Self::PAGES],
    pokemon_background: Texture,
    offset: (u8, bool, f32),

    pokemon: Option<SummaryPokemon>,

}

impl SummaryGui {

    const PAGES: usize = 3;

    const HEADER_LEFT: Color = Color::rgb(224.0 / 255.0, 216.0 / 255.0, 152.0 / 255.0);
    const HEADER_LEFT_DARK: Color = Color::rgb(192.0 / 255.0, 184.0 / 255.0, 112.0 / 255.0);
    const HEADER_RIGHT: Color = Color::rgb(0.0, 120.0 / 255.0, 192.0 / 255.0);
    const HEADER_RIGHT_DARK: Color = Color::rgb(0.0, 72.0 / 255.0, 144.0 / 255.0);

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            alive: false,
            headers: [
                "POKEMON INFO",
                "POKEMON SKILLS",
                "KNOWN MOVES"
            ],
            pages: [
                byte_texture(ctx, include_bytes!("../../../assets/gui/party/summary/info.png")),
                byte_texture(ctx, include_bytes!("../../../assets/gui/party/summary/skills.png")),
                byte_texture(ctx, include_bytes!("../../../assets/gui/party/summary/moves.png")),
            ],
            offset: Default::default(),
            pokemon_background: byte_texture(ctx, include_bytes!("../../../assets/gui/party/summary/pokemon.png")),
            page: 0,
            pokemon: None,
        }
    }

    pub fn input(&mut self, ctx: &Context) {
        if pressed(ctx, Control::Left) && self.page > 0 {
            self.page -= 1;
        }
        if pressed(ctx, Control::Right) && self.page < Self::PAGES - 1 {
            self.page += 1;
        }
        if pressed(ctx, Control::B) {
            self.despawn();
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let w = 114.0 + (self.page << 4) as f32;
        let rw = firecore_util::WIDTH - w;
        draw_rectangle(ctx, 0.0, 1.0, w, 15.0, Self::HEADER_LEFT);
        draw_rectangle(ctx, w, 1.0, rw, 16.0, Self::HEADER_RIGHT);
        draw_line(ctx, 0.0, 16.5, w, 16.5, 1.0, Self::HEADER_LEFT_DARK);
        draw_text_left(ctx, &1, self.headers[self.page], TextColor::White, 5.0, 1.0);
        for page in 0..Self::PAGES {
            let color = if self.page < page {
                Self::HEADER_RIGHT_DARK
            } else if self.page == page {
                crate::gui::panel::WHITE
            } else {
                Self::HEADER_LEFT_DARK
            };
            draw_circle(ctx, 106.0 + (page << 4) as f32, 9.0, 6.0, color);
        }
        if let Some(pokemon) = self.pokemon.as_ref() {
            self.pokemon_background.draw(ctx, position(0.0, 17.0));
            pokemon.front.0.draw(ctx, position(28.0, pokemon.front.1 + self.offset.2));
            draw_text_left(ctx, &1, &pokemon.display.level, TextColor::White, 5.0, 19.0);
            draw_text_left(ctx, &1, pokemon.pokemon.1, TextColor::White, 41.0, 19.0);
            const TOP: DrawParams = position(0.0, 17.0);
            match self.page {
                0 => {
                    self.pages[0].draw(ctx, TOP);
                    draw_text_left(ctx, &1, &pokemon.pokemon.0, TextColor::Black, 168.0, 21.0);
                    draw_text_left(ctx, &1, &pokemon.display.name, TextColor::Black, 168.0, 36.0);

                        for (index, display) in pokemon.types.iter().enumerate() {
                            let x = 168.0 + 37.0 * index as f32;
                            draw_rectangle(ctx, x, 52.0, 32.0, 6.0, display.upper);
                            draw_rectangle(ctx, x, 58.0, 32.0, 6.0, display.lower);
                            crate::graphics::draw_text_center(ctx, &0, &display.name, TextColor::White, x + 16.0, 52.0)
                        }

                        // draw_text_left(1, &pokemon.item, TextColor::Black, 168.0, 96.0);
                }
                1 => {
                    self.pages[1].draw(ctx, TOP);
                },
                2 => {
                    self.pages[2].draw(ctx, position(119.0, 17.0));
                },
                _ => unreachable!(),
            }
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.offset.0 < 2 {
            match self.offset.1 {
                false => {
                    self.offset.2 -= delta * 120.0;
                    if self.offset.2 < -10.0 {
                        self.offset.1 = true;
                    }
                }
                true => {
                    self.offset.2 += delta * 120.0;
                    if self.offset.2 > 0.0 {
                        self.offset.0 += 1;
                        self.offset.1 = false;
                    }
                }
            }
        }
    }

    pub fn spawn(&mut self, pokemon: PokemonDisplay) {
        self.alive = true;
        self.offset = Default::default();
        self.pokemon = Some(SummaryPokemon::new(pokemon));
    }

    pub fn despawn(&mut self) {
        self.alive = false;
        self.pokemon = None;
    }

}

pub struct SummaryPokemon {
    display: PokemonDisplay,
    pokemon: (String, &'static String), // id and name
    front: (Texture, f32), // texture and pos
    types: Vec<PokemonTypeDisplay>,
    // item: String,
}

impl SummaryPokemon {

    pub fn new(display: PokemonDisplay) -> Self {
        let pokemon = display.instance.pokemon.unwrap();
        let mut types = Vec::with_capacity(if pokemon.secondary_type.is_some() { 2 } else { 1 });

        types.push(PokemonTypeDisplay::new(pokemon.primary_type));

        if let Some(secondary) = pokemon.secondary_type {
            types.push(PokemonTypeDisplay::new(secondary));
        }

        let texture = pokemon_texture(&pokemon.id, Front);

        Self {
            pokemon: (pokemon.id.to_string(), &pokemon.name),
            front: (texture.clone(), 34.0 + (64.0 - texture.height() as f32) / 2.0),
            types,
            // item: pokemon.instance.item.map(|item| item.name.to_ascii_uppercase()).unwrap_or("NONE".to_owned()),
            display,

        }
    }

}