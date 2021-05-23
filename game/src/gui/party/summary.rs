use pokedex::texture::{
    pokemon_texture,
    PokemonTexture::Front,
};

use input::{pressed, Control};

use crate::text::TextColor;
use crate::graphics::{byte_texture, draw, draw_text_left};

use macroquad::prelude::{
    Color,
    color_u8,
    Texture2D,
    draw_circle,
    draw_line,
    draw_rectangle
};

use crate::gui::pokemon::{PokemonDisplay, PokemonTypeDisplay};

pub struct SummaryGui {

    pub alive: bool,

    page: usize,
    headers: [&'static str; Self::PAGES],
    pages: [Texture2D; Self::PAGES],
    pokemon_background: Texture2D,
    offset: (u8, bool, f32),

    pokemon: Option<SummaryPokemon>,

}

impl SummaryGui {

    const PAGES: usize = 3;

    const HEADER_LEFT: Color = color_u8!(224, 216, 152, 255);
    const HEADER_LEFT_DARK: Color = color_u8!(192, 184, 112, 255);
    const HEADER_RIGHT: Color = color_u8!(0, 120, 192, 255);
    const HEADER_RIGHT_DARK: Color = color_u8!(0, 72, 144, 255);

    pub fn new() -> Self {
        Self {
            alive: false,
            headers: [
                "POKEMON INFO",
                "POKEMON SKILLS",
                "KNOWN MOVES"
            ],
            pages: [
                byte_texture(include_bytes!("../../../assets/gui/party/summary/info.png")),
                byte_texture(include_bytes!("../../../assets/gui/party/summary/skills.png")),
                byte_texture(include_bytes!("../../../assets/gui/party/summary/moves.png")),
            ],
            offset: Default::default(),
            pokemon_background: byte_texture(include_bytes!("../../../assets/gui/party/summary/pokemon.png")),
            page: 0,
            pokemon: None,
        }
    }

    pub fn input(&mut self) {
        if pressed(Control::Left) && self.page > 0 {
            self.page -= 1;
        }
        if pressed(Control::Right) && self.page < Self::PAGES - 1 {
            self.page += 1;
        }
        if pressed(Control::B) {
            self.despawn();
        }
    }

    pub fn render(&self) {
        let w = 114.0 + (self.page << 4) as f32;
        let rw = firecore_util::WIDTH - w;
        draw_rectangle(0.0, 1.0, w, 15.0, Self::HEADER_LEFT);
        draw_rectangle(w, 1.0, rw, 16.0, Self::HEADER_RIGHT);
        draw_line(0.0, 16.5, w, 16.5, 1.0, Self::HEADER_LEFT_DARK);
        draw_text_left(1, self.headers[self.page], TextColor::White, 5.0, 1.0);
        for page in 0..Self::PAGES {
            let color = if self.page < page {
                Self::HEADER_RIGHT_DARK
            } else if self.page == page {
                crate::graphics::WHITE
            } else {
                Self::HEADER_LEFT_DARK
            };
            draw_circle(106.0 + (page << 4) as f32, 9.0, 6.0, color);
        }
        if let Some(pokemon) = self.pokemon.as_ref() {
            draw(self.pokemon_background, 0.0, 17.0);
            draw(pokemon.front.0, 28.0, pokemon.front.1 + self.offset.2);
            draw_text_left(1, &pokemon.display.level, TextColor::White, 5.0, 19.0);
            draw_text_left(1, pokemon.pokemon.1, TextColor::White, 41.0, 19.0);
            match self.page {
                0 => {
                    draw(self.pages[0], 0.0, 17.0);
                    draw_text_left(1, &pokemon.pokemon.0, TextColor::Black, 168.0, 21.0);
                    draw_text_left(1, &pokemon.display.name, TextColor::Black, 168.0, 36.0);

                        for (index, display) in pokemon.types.iter().enumerate() {
                            let x = 168.0 + 37.0 * index as f32;
                            draw_rectangle(x, 52.0, 32.0, 6.0, display.upper);
                            draw_rectangle(x, 58.0, 32.0, 6.0, display.lower);
                            crate::graphics::draw_text_center(0, &display.name, TextColor::White, x + 16.0, 52.0)
                        }

                        // draw_text_left(1, &pokemon.item, TextColor::Black, 168.0, 96.0);
                }
                1 => {
                    draw(self.pages[1], 0.0, 17.0);
                },
                2 => {
                    draw(self.pages[2], 119.0, 17.0);
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
    front: (Texture2D, f32), // texture and pos
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
            front: (texture, 34.0 + (64.0 - texture.height()) / 2.0),
            types,
            // item: pokemon.instance.item.map(|item| item.name.to_ascii_uppercase()).unwrap_or("NONE".to_owned()),
            display,

        }
    }

}