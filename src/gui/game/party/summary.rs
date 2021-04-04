use firecore_pokedex::pokemon::instance::PokemonInstance;
use macroquad::prelude::Texture2D;
use firecore_input::{pressed, Control};

use crate::util::graphics::{byte_texture, draw};

pub struct SummaryGui {

    alive: bool,

    pages: [Texture2D; 1],

    page: Page,

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
                byte_texture(include_bytes!("../../../../build/assets/gui/party/summary/info.png")),
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
        match self.page {
            Page::Info => {
                draw(self.pages[0], 0.0, 0.0);
            }
        }
    }

    pub fn spawn(&mut self, pokemon: PokemonInstance) {
        self.alive = true;
        self.pokemon = Some(
            SummaryPokemon {
                texture: crate::util::pokemon::pokemon_texture(&pokemon.pokemon.data.id, firecore_pokedex::pokemon::texture::PokemonTexture::Front),
                pokemon,
            }
        );
    }

    pub fn despawn(&mut self) {
        self.alive = false;
    }

}

pub struct SummaryPokemon {
    pokemon: PokemonInstance,
    texture: Texture2D,
}