use firecore_pokedex::pokedex;
use firecore_pokedex::pokemon::texture::PokemonTexture;
use firecore_util::text::TextColor;
use macroquad::prelude::Texture2D;
use firecore_input::{pressed, Control};

use crate::graphics::{byte_texture, draw, draw_text_left};
use crate::textures::pokemon_texture;

use super::PartyGuiData;

pub struct SummaryGui {

    pub alive: bool,

    page: Page,
    pages: [Texture2D; 1],

    pokemon: Option<SummaryPokemon>,

}

enum Page {

    Info,

}

impl SummaryGui {

    pub fn new() -> Self {
        Self {
            alive: false,
            pages: [
                byte_texture(include_bytes!("../../../assets/gui/party/summary/info.png")),
                // byte_texture(include_bytes!("../../../../build/assets/gui/party/summary/1.png")),
                // byte_texture(include_bytes!("../../../../build/assets/gui/party/summary/2.png")),
            ],
            page: Page::Info,
            pokemon: None,
        }
    }

    pub fn input(&mut self) {
        // if pressed(Control::Left) && self.page > 0 {
        //     self.page -= 1;
        // }
        // if pressed(Control::Right) && self.page < 1 {
        //     self.page += 1;
        // }
        if pressed(Control::B) {
            self.despawn();
        }
    }

    pub fn render(&self) {
        if self.alive {
            match self.page {
                Page::Info => {
                    draw(self.pages[0], 0.0, 0.0);
                    if let Some(pokemon) = self.pokemon.as_ref() {
                        draw(pokemon.texture, 28.0, pokemon.texture_pos);
                        draw_text_left(1, &pokemon.pokemon.level, TextColor::White, 5.0, 19.0);
                        draw_text_left(1, &pokemon.pokemon_name, TextColor::White, 41.0, 19.0);
                        draw_text_left(1, &pokemon.pokemon_id, TextColor::Black, 168.0, 21.0);
                        draw_text_left(1, &pokemon.pokemon.name, TextColor::Black, 168.0, 36.0);
                    }
                }
            }
        }        
    }

    pub fn spawn(&mut self, pokemon: PartyGuiData) {
        self.alive = true;
        let dex_pokemon = pokedex().get(&pokemon.id).unwrap();
        let texture = pokemon_texture(&pokemon.id, PokemonTexture::Front);
        self.pokemon = Some(
            SummaryPokemon {
                pokemon_id: format!("{:#03}", pokemon.id),
                pokemon_name: dex_pokemon.data.name.to_ascii_uppercase(),
                texture_pos: 34.0 + (64.0 - texture.height()) / 2.0,
                pokemon,
                texture,
            }
        );
    }

    pub fn despawn(&mut self) {
        self.alive = false;
        self.pokemon = None;
    }

}

pub struct SummaryPokemon {
    pokemon: PartyGuiData,
    pokemon_id: String,
    pokemon_name: String,
    texture: Texture2D,
    texture_pos: f32,
}