use std::ops::DerefMut;
use firecore_util::Entity;
use crate::gui::battle::health_bar;
use firecore_util::text::TextColor;
use crate::util::graphics::Texture;
use crate::gui::GuiComponent;
use crate::gui::background::Background;
use crate::io::data::player::PlayerData;
use crate::util::graphics::draw;
use crate::util::graphics::draw_rect;
use crate::util::graphics::draw_text_left_color;
use crate::util::graphics::texture::byte_texture;

const TEXTURE_TICK: f32 = 0.15;

pub static mut SPAWN: bool = false;

// pub fn toggle() {
//     let mut gui = PARTY_GUI.write();
//     if gui.is_alive() {
//         gui.despawn();
//     } else {
//         gui.spawn();
//         gui.on_start();
//     }
// }

pub struct PokemonPartyGui {

    alive: bool,

    background: Background,
    primary_texture: Texture,
    pokemon_texture: Texture,

    accumulator: f32,
    
    pokemon: [Option<PartyGuiData>; 6],

}

impl PokemonPartyGui {

    pub fn new() -> Self {
        Self {
            alive: false,
            background: Background::new(byte_texture(include_bytes!("../../../build/assets/gui/party/background.png")), 0.0, 0.0),
            primary_texture: byte_texture(include_bytes!("../../../build/assets/gui/party/primary.png")),
            pokemon_texture: byte_texture(include_bytes!("../../../build/assets/gui/party/pokemon.png")),
            accumulator: 0.0,
            pokemon: [None, None, None, None, None, None],
        }

    }

    pub fn on_start(&mut self) {
        let mut player_data = macroquad::prelude::collections::storage::get_mut::<PlayerData>().expect("Could not get player data!");
        let player_data = player_data.deref_mut();
        self.pokemon.fill(None);
        for pokemon in player_data.party.pokemon.iter().enumerate() {
            if pokemon.0 == 6 {
                break;
            }
            
            if let Some(pokemon_data) = firecore_pokedex::POKEDEX.get(&pokemon.1.id) {
                let pokemon_data = pokemon_data.value();

                let max = firecore_pokedex::pokemon::battle::calculate_hp(pokemon_data.base.hp, pokemon.1.ivs.hp, pokemon.1.evs.hp, pokemon.1.level);
                let curr = pokemon.1.current_hp.unwrap_or(max);
    
                let texture = if let Some(file) = crate::util::file::noasync::read_noasync(format!("assets/pokedex/textures/icon/{}.png", pokemon_data.data.name.to_ascii_lowercase())) {
                    byte_texture(&file)
                } else {
                    crate::util::graphics::texture::debug_texture()
                };
    
                self.pokemon[pokemon.0] = Some(PartyGuiData {
                    name: pokemon.1.nickname.as_ref().unwrap_or(&pokemon_data.data.name).to_ascii_uppercase(),
                    level: format!("Lv{}", pokemon.1.level),
                    hp: format!("{}/{}", curr, max),
                    health_width: ((curr as f32 / max as f32).ceil() * 48.0) as u32,
                    texture: texture,
                });
            }            
        }
    }

    fn render_cell(&self, index: usize, data: &PartyGuiData) {
        let offset = -14.0 + (24.0 * index as f32);
        draw(self.pokemon_texture, 89.0, offset);
        macroquad::prelude::draw_texture_ex(data.texture, 84.0, offset - 8.0, macroquad::prelude::WHITE, macroquad::prelude::DrawTextureParams {
            source: Some(macroquad::prelude::Rect::new(0.0, if self.accumulator > TEXTURE_TICK { 32.0 } else { 0.0 }, 32.0, 32.0)),
            ..Default::default()
        });
        draw_text_left_color(0, &data.name, TextColor::White, 119.0, offset/* + 1.0*/);
        draw_text_left_color(0, &data.level, TextColor::White, 129.0, offset + 13.0 - 4.0);
        draw_text_left_color(0, &data.hp, TextColor::White, 209.0, offset + 13.0 - 1.0);
        draw_rect(health_bar::UPPER_COLOR, 185.0, offset + 8.0, data.health_width, 1);
        draw_rect(health_bar::LOWER_COLOR, 185.0, offset + 9.0, data.health_width, 2);
    }

}

impl crate::util::Update for PokemonPartyGui {
    fn update(&mut self, delta: f32) {
        if self.is_alive() {
            self.accumulator += delta;
            if self.accumulator > TEXTURE_TICK * 2.0 {
                self.accumulator = 0.0;
            }
        }
    }
}

impl crate::util::Render for PokemonPartyGui {

    fn render(&self) {
        if self.is_alive() {
            self.background.render();
            for pokemon in self.pokemon.iter().enumerate() {
                if let Some(data) = pokemon.1 {
                    if pokemon.0 == 0 {
                        const OFFSET: f32 = 26.0;
                        draw(self.primary_texture, 3.0, OFFSET - 6.0);
                        macroquad::prelude::draw_texture_ex(data.texture, 0.0, OFFSET, macroquad::prelude::WHITE, macroquad::prelude::DrawTextureParams {
                            source: Some(macroquad::prelude::Rect::new(0.0, if self.accumulator > TEXTURE_TICK { 32.0 } else { 0.0 }, 32.0, 32.0)),
                            ..Default::default()
                        });
                    } else {
                        self.render_cell(pokemon.0, data);
                    }
                }
            }
        }        
    }

}

impl crate::util::Input for PokemonPartyGui {
    fn input(&mut self, _delta: f32) {
        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Escape) || firecore_input::pressed(firecore_input::Control::Start) {
            self.despawn();
        }
    }
}

impl Entity for PokemonPartyGui {
    fn spawn(&mut self) {
        self.alive = true;
        self.background.spawn();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.background.despawn();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

#[derive(Clone)]
pub struct PartyGuiData {

    texture: Texture,
    name: String,
    level: String,
    hp: String,
    health_width: u32,

}