use firecore_util::{Reset, text::TextColor};
use firecore_input::{pressed, Control};
use firecore_pokedex::pokemon::party::PokemonParty;
use firecore_pokedex::pokemon::texture::PokemonTexture::Icon;

use macroquad::prelude::{collections::storage::get, Texture2D, draw_rectangle_lines, draw_texture_ex, LIME, RED, WHITE, DrawTextureParams, Rect};
use smallvec::SmallVec;

use crate::battle::pokemon::BattleParty;
use firecore_data::player::PlayerSaves;

use crate::util::graphics::{byte_texture, draw, draw_rect, draw_text_left};
use crate::util::pokemon::pokemon_texture;

use self::select::SelectMenu;

use super::health_bar::HealthBar;

pub mod select;
pub mod summary;

const TEXTURE_TICK: f32 = 0.15;

pub struct PokemonPartyGui {

    alive: bool,

    background: Texture2D,
    primary_slot: Texture2D,
    pokemon_slot: Texture2D,
    
    pokemon: SmallVec<[PartyGuiData; 6]>,

    pub selected: Option<u8>,
    
    select: SelectMenu,

    accumulator: f32,

    cursor: u8,
    right_cursor: Option<u8>,

    swaps: Vec<(usize, usize)>,

}

impl PokemonPartyGui {

    pub fn new() -> Self {
        Self {
            alive: false,
            background: byte_texture(include_bytes!("../../../../build/assets/gui/party/background.png")),
            primary_slot: byte_texture(include_bytes!("../../../../build/assets/gui/party/primary.png")),
            pokemon_slot: byte_texture(include_bytes!("../../../../build/assets/gui/party/pokemon.png")),
            accumulator: 0.0,
            pokemon: SmallVec::new(),
            select: SelectMenu::new(),
            cursor: 0,
            right_cursor: None,
            selected: None,
            swaps: Vec::new(),
        }

    }

    fn on_spawn(&mut self, world: bool) {
        self.alive = true;
        self.reset();        
        self.select.is_world = world;
    }

    pub fn spawn_battle(&mut self, party: &BattleParty) {
        self.on_spawn(false);
        for pokemon in party.pokemon.iter().map(|pokemon| &pokemon.pokemon){
            self.pokemon.push(PartyGuiData {
                name: pokemon.name(),
                level: format!("Lv{}", pokemon.data.level),
                hp: format!("{}/{}", pokemon.current_hp, pokemon.base.hp),
                health_width: (pokemon.current_hp as f32 / pokemon.base.hp as f32).ceil() * 48.0,
                texture: pokemon_texture(&pokemon.pokemon.data.id, Icon),
            });
        }
    }

    pub fn spawn_world(&mut self) {
        self.on_spawn(true);
        if let Some(saves) = get::<PlayerSaves>() {
            for pokemon in saves.get().party.iter() {
                if let Some(pokemon_data) = firecore_pokedex::pokedex().get(&pokemon.id) {
    
                    let max = firecore_pokedex::pokemon::instance::calculate_hp(pokemon_data.base.hp, pokemon.data.ivs.hp, pokemon.data.evs.hp, pokemon.data.level);
                    let current = pokemon.current_hp.unwrap_or(max);
        
                    self.pokemon.push(PartyGuiData {
                        name: pokemon.data.nickname.as_ref().map(|nick| nick.clone()).unwrap_or(pokemon_data.data.name.to_ascii_uppercase()),
                        level: format!("Lv{}", pokemon.data.level),
                        hp: format!("{}/{}", current, max),
                        health_width: HealthBar::get_hp_width(current, max),
                        texture: pokemon_texture(&pokemon_data.data.id, Icon),
                    });
                }            
            }
        }
    }

    pub fn input(&mut self) {

        if self.select.alive {
            if let Some(action) = self.select.input() {
                match action {
                    select::SelectAction::Select => {
                        self.selected = Some(self.cursor);
                        self.select.alive = false;
                    }
                    select::SelectAction::Summary => {
                        macroquad::prelude::warn!("Function not implemented!");
                    }
                }
            }            
        } else {

            if pressed(Control::A) {
                if let Some(selected) = self.selected.take() {
                    if self.select.is_world {
                        let swap = (self.cursor as usize, selected as usize);
                        self.pokemon.swap(swap.0, swap.1);
                        self.swaps.push(swap);
                    }
                } else {
                    self.select.toggle();
                }
            } else {
                if pressed(Control::Up) && self.cursor > 1 {
                    self.cursor -= 1;
                }
                if pressed(Control::Down) {
                    if self.cursor < self.pokemon.len() as u8 - 1 {
                        self.cursor += 1;
                    }            
                }
                if pressed(Control::Left) && self.cursor != 0 {
                    self.right_cursor = Some(self.cursor);
                    self.cursor = 0;
                }
                if pressed(Control::Right) && self.cursor == 0 {
                    self.cursor = self.right_cursor.unwrap_or(1);
                }
            }           
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.alive {
            self.accumulator += delta;
            if self.accumulator > TEXTURE_TICK * 2.0 {
                self.accumulator = 0.0;
            }
        }
    }

    pub fn render(&self) {
        if self.alive {
            draw(self.background, 0.0, 0.0);
            self.pokemon.iter().enumerate().for_each(|(index, pokemon)| {
                if index == 0 {
                    self.render_primary(pokemon);                    
                } else {
                    self.render_cell(index, pokemon);
                }
            });
            if self.cursor == 0 {
                draw_rectangle_lines(8.0, 26.0, 79.0, 49.0, 2.0, RED);
            } else {
                draw_rectangle_lines(89.0, -14.0 + 24.0 * self.cursor as f32, 150.0, 22.0, 2.0, RED);
            }
            if self.select.is_world {
                if let Some(selected) = self.selected {
                    if selected == 0 {
                        draw_rectangle_lines(8.0, 26.0, 79.0, 49.0, 2.0, LIME);
                    } else {
                        draw_rectangle_lines(89.0, -14.0 + 24.0 * selected as f32, 150.0, 22.0, 2.0, LIME);
                    }
                }
            }
        }
        self.select.render();
    }

    fn render_primary(&self, pokemon: &PartyGuiData) {
        draw(self.primary_slot, 3.0, 20.0);
        draw_texture_ex(pokemon.texture, 0.0, 26.0, WHITE, DrawTextureParams {
            source: Some(
                    Rect::new(
                        0.0, 
                        if self.accumulator > TEXTURE_TICK { 32.0 } else { 0.0 }, 
                        32.0, 
                        32.0
                    )
                ),
            ..Default::default()
        });
        draw_text_left(0, &pokemon.name, TextColor::White, 33.0, 36.0);
        draw_text_left(0, &pokemon.level, TextColor::White, 41.0, 45.0);
        draw_text_left(0, &pokemon.hp, TextColor::White, 52.0, 61.0);
        draw_rect(HealthBar::UPPER_COLOR, 33.0, 59.0, pokemon.health_width, 1.0);
        draw_rect(HealthBar::LOWER_COLOR, 33.0, 60.0, pokemon.health_width, 2.0);
    }

    fn render_cell(&self, index: usize, pokemon: &PartyGuiData) {
        let offset = -14.0 + (24.0 * index as f32);
        draw(self.pokemon_slot, 89.0, offset);
        draw_texture_ex(pokemon.texture, 84.0, offset - 8.0, WHITE, DrawTextureParams {
            source: Some(
                Rect::new(
                    0.0, 
                    if self.accumulator > TEXTURE_TICK { 32.0 } else { 0.0 }, 
                    32.0, 
                    32.0
                )
            ),
            ..Default::default()
        });
        draw_text_left(0, &pokemon.name, TextColor::White, 119.0, offset);
        draw_text_left(0, &pokemon.level, TextColor::White, 129.0, offset + 9.0);
        draw_text_left(0, &pokemon.hp, TextColor::White, 209.0, offset + 11.0);
        draw_rect(HealthBar::UPPER_COLOR, 185.0, offset + 8.0, pokemon.health_width, 1.0);
        draw_rect(HealthBar::LOWER_COLOR, 185.0, offset + 9.0, pokemon.health_width, 2.0);
    }

    pub fn on_finish(&mut self, party: &mut PokemonParty) {
        for swap in &self.swaps {
            party.swap(swap.0, swap.1);
        }
        self.swaps.clear();
    }

    pub fn despawn(&mut self) {
        self.alive = false;
        self.select.alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

}

impl Reset for PokemonPartyGui {
    fn reset(&mut self) {
        self.cursor = 0;
        self.right_cursor = None;
        self.accumulator = 0.0;
        self.pokemon.clear();
        self.swaps.clear();
    }
}

#[derive(Clone)]
pub struct PartyGuiData {

    texture: Texture2D,
    name: String,
    level: String,
    hp: String,
    health_width: f32,

}