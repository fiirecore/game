use firecore_input::Control;
use firecore_input::pressed;
use firecore_pokedex::pokemon::battle::BattlePokemon;
use firecore_util::Entity;
use crate::gui::battle::health_bar;
use crate::util::Reset;
use firecore_util::text::TextColor;
use crate::util::graphics::Texture;
use crate::gui::GuiComponent;
use crate::gui::background::Background;
use crate::util::graphics::draw;
use crate::util::graphics::draw_rect;
use crate::util::graphics::draw_text_left_color;
use crate::util::graphics::texture::byte_texture;

const TEXTURE_TICK: f32 = 0.15;

pub static mut SPAWN: Option<bool> = None;

pub fn spawn(despawn_on_select: bool) {
    unsafe { SPAWN = Some(despawn_on_select); }
}

pub struct PokemonPartyGui {

    alive: bool,

    background: Background,
    primary_texture: Texture,
    pokemon_texture: Texture,

    accumulator: f32,
    
    pokemon: [Option<PartyGuiData>; 6],

    despawn_on_select: bool,
    cursor_pos: u8,

    pub selected: Option<u8>,

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
            cursor_pos: 0,
            selected: None,
            despawn_on_select: false,
        }

    }

    fn on_start(&mut self, despawn_on_select: bool) {
        self.despawn_on_select = despawn_on_select;
    }

    pub fn on_battle_start(&mut self, player_pokemon: &Vec<BattlePokemon>) {
        for (index, pokemon) in player_pokemon.iter().enumerate() {
            let texture = crate::pokemon::pokemon_texture(&pokemon.pokemon.data.number, firecore_pokedex::pokemon::texture::PokemonTexture::Icon);
            self.pokemon[index] = Some(PartyGuiData {
                name: pokemon.nickname.as_ref().unwrap_or(&pokemon.pokemon.data.name).to_ascii_uppercase(),
                level: format!("Lv{}", pokemon.level),
                hp: format!("{}/{}", pokemon.current_hp, pokemon.base.hp),
                health_width: ((pokemon.current_hp as f32 / pokemon.base.hp as f32).ceil() * 48.0) as u32,
                texture,
            });
        }
        self.on_start(true);
    }

    pub fn on_world_start(&mut self, despawn_on_select: bool) {
        if let Some(player_data) = crate::io::data::player::PLAYER_DATA.write().as_mut() {
            for pokemon in player_data.party.pokemon.iter().enumerate() {
                if pokemon.0 == 6 {
                    break;
                }
                
                if let Some(pokemon_data) = firecore_pokedex::POKEDEX.get(&pokemon.1.id) {
                    let pokemon_data = pokemon_data.value();
    
                    let max = firecore_pokedex::pokemon::battle::calculate_hp(pokemon_data.base.hp, pokemon.1.ivs.hp, pokemon.1.evs.hp, pokemon.1.level);
                    let curr = pokemon.1.current_hp.unwrap_or(max);
        
                    let texture = crate::pokemon::pokemon_texture(&pokemon_data.data.number, firecore_pokedex::pokemon::texture::PokemonTexture::Icon);
        
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
        self.on_start(despawn_on_select);
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
            if self.cursor_pos == 0 {
                macroquad::prelude::draw_rectangle_lines(8.0, 26.0, 79.0, 49.0, 2.0, macroquad::prelude::RED);
            } else {
                let index = -14.0 + (24.0 * self.cursor_pos as f32);
                macroquad::prelude::draw_rectangle_lines(89.0, index, 150.0, 22.0, 2.0, macroquad::prelude::RED);
            }
        }        
    }

}

impl crate::util::Input for PokemonPartyGui {
    fn input(&mut self, _delta: f32) {
        if pressed(Control::Start) || macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Escape) {
            self.despawn();
        }
        if pressed(Control::Up) {
            if self.cursor_pos > 0 {
                self.cursor_pos -= 1;
            }
        }
        if pressed(Control::Down) {
            if self.cursor_pos < 5 {
                self.cursor_pos += 1;
            }
        }
        if pressed(Control::A) {
            self.selected = Some(self.cursor_pos);
            if self.despawn_on_select {
                self.despawn();
            }
        }
    }
}

impl Entity for PokemonPartyGui {
    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
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

impl Reset for PokemonPartyGui {
    fn reset(&mut self) {
        self.cursor_pos = 0;
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