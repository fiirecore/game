use macroquad::color_u8;
use macroquad::prelude::Color;
use macroquad::prelude::draw_line;
use util::Reset;
use input::{pressed, Control};
use storage::{data, data_mut};
use crate::text::TextColor;
use macroquad::prelude::{Texture2D, draw_rectangle, draw_texture_ex, WHITE, DrawTextureParams, Rect};
use deps::vec::ArrayVec;

use crate::graphics::{byte_texture, draw, draw_text_left};

use self::select::PartySelectMenu;
use self::summary::SummaryGui;

use super::health::HealthBar;
use super::pokemon::PokemonDisplay;

pub mod select;
pub mod summary;

pub struct PartyGui {

    pub alive: bool,
    
    select: PartySelectMenu,
    summary: SummaryGui,

    background: Texture2D,
    ball: Texture2D,
    health: Texture2D,
    
    pub pokemon: ArrayVec<[PokemonDisplay; 6]>,

    pub selected: Option<usize>,

    accumulator: f32,

    cursor: usize,
    right_cursor: Option<usize>,

    exitable: bool,

}

impl PartyGui {

    const LIGHT: Color = color_u8!(128, 192, 216, 255);
    const DARK: Color = color_u8!(56, 144, 216, 255);

    const HOVER_LIGHT: Color = color_u8!(168, 232, 248, 255);
    const HOVER_DARK: Color = color_u8!(120, 208, 232, 255);

    const HOVER_BORDER: Color = color_u8!(248, 112, 48, 255);

    const SELECT_LIGHT: Color = color_u8!(176, 248, 160, 255);
    const SELECT_DARK: Color = color_u8!(120, 216, 128, 255);

    const SELECT_BORDER: Color = color_u8!(248, 248, 112, 255);

    const SELECT_CORNER: Color = color_u8!(120, 152, 96, 255);

    pub fn new() -> Self {
        Self {
            alive: false,
            select: PartySelectMenu::new(),
            summary: SummaryGui::new(),
            background: byte_texture(include_bytes!("../../../assets/gui/party/background.png")),
            ball: byte_texture(include_bytes!("../../../assets/gui/party/ball.png")),
            health: HealthBar::texture(),
            accumulator: 0.0,
            pokemon: ArrayVec::new(),
            cursor: 0,
            right_cursor: None,
            selected: None,
            exitable: true,
        }

    }

    pub fn on_spawn(&mut self, world: Option<bool>) {
        self.alive = true;
        self.reset();        
        self.select.is_world = world;
    }

    pub fn spawn(&mut self, party: ArrayVec<[PokemonDisplay; 6]>, is_world: Option<bool>, exitable: bool) {
        self.on_spawn(is_world);
        self.pokemon = party;
        self.exitable = exitable;
    }

    pub fn spawn_world(&mut self) {
        self.on_spawn(Some(true));
        self.exitable = true;
        self.spawn(data().party.iter().map(|instance| PokemonDisplay::new(std::borrow::Cow::Borrowed(instance))).collect(), Some(true), true);
    }

    pub fn input(&mut self) {
        if self.summary.alive {
            self.summary.input();
        } else if self.select.alive {
            if let Some(action) = self.select.input() {
                match action {
                    select::PartySelectAction::Select => {
                        self.selected = Some(self.cursor);
                        self.select.alive = false;
                    }
                    select::PartySelectAction::Summary => {
                        self.summary.spawn(self.pokemon[self.cursor].clone());
                        self.select.alive = false;
                    }
                }
            }            
        } else {

            if pressed(Control::A) {
                if let Some(selected) = self.selected.take() {
                    if let Some(is_world) = self.select.is_world {
                        if is_world {
                            self.pokemon.swap(self.cursor, selected);
                            data_mut().party.swap(self.cursor, selected);
                        }
                    }
                } else {
                    if self.select.is_world.is_some() {
                        self.select.toggle();
                    } else {
                        self.selected = Some(self.cursor);
                    }
                }
            } else {
                if pressed(Control::Up) && self.cursor > 1 {
                    self.cursor -= 1;
                }
                if pressed(Control::Down) {
                    if self.cursor < self.pokemon.len() - 1 {
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
                if (pressed(Control::B) || pressed(Control::Start)) && self.exitable {
                    self.despawn();
                }
            }           
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.alive {
            self.accumulator += delta;
            if self.accumulator > PokemonDisplay::ICON_TICK * 2.0 {
                self.accumulator = 0.0;
            }
            if let Some(is_world) = self.select.is_world {
                if is_world && self.summary.alive {
                    self.summary.update(delta);
                }
            }
        }
    }

    pub fn render(&self) {
        if self.summary.alive {
            self.summary.render();
        } else {
            draw(self.background, 0.0, 0.0);
            self.pokemon.iter().enumerate().for_each(|(index, pokemon)| {
                if index == 0 {
                    self.render_primary(pokemon);                    
                } else {
                    self.render_cell(index, pokemon, self.cursor == index);
                }
            });
            if self.select.is_world.is_some() {
                self.select.render();
            }
        }
    }

    fn render_primary(&self, pokemon: &PokemonDisplay) {
        let selected = self.cursor == 0;
        let mut skip = false;
        if self.select.is_world.is_some() {
            if let Some(selected_index) = self.selected {
                let selected_index = selected_index == 0;
                if selected_index || selected {
                    
                    draw_line(10.5, 28.0, 10.5, 73.0, 2.0, Self::SELECT_LIGHT);
                    draw_line(10.0, 28.5, 84.0, 28.5, 2.0, Self::SELECT_LIGHT);

                    draw_line(83.5, 28.0, 83.5, 73.0, 1.0, Self::SELECT_CORNER);
                    draw_line(10.0, 72.5, 84.0, 72.5, 1.0, Self::SELECT_CORNER);
                    
                    self.render_primary_color(Self::SELECT_LIGHT, Self::SELECT_DARK, Some( if selected { Self::HOVER_BORDER } else { Self::SELECT_BORDER }));
                    skip = true;
                }
            }
        }
        if !skip {
            if selected {
                self.render_primary_color(Self::HOVER_LIGHT, Self::HOVER_DARK, Some(Self::HOVER_BORDER));
            } else {
                self.render_primary_color(Self::LIGHT, Self::DARK, None);
            }
        }
        self.render_ball(3.0, 20.0, selected);
        self.render_pokemon(pokemon.icon, 0.0, 20.0, selected);
        draw_text_left(0, &pokemon.name, TextColor::White, 33.0, 36.0);
        draw_text_left(0, &pokemon.level, TextColor::White, 41.0, 45.0);
        self.render_health(pokemon, 17.0, 57.0);
    }

    fn render_primary_color(&self, light: Color, dark: Color, border: Option<Color>) {
        draw_rectangle(11.0, 29.0, 72.0, 27.0, dark);
        draw_line(11.0, 56.5, 83.0, 56.5, 1.0, light);
        draw_line(11.0, 57.5, 83.0, 57.5, 1.0, dark);
        draw_rectangle(11.0, 58.0, 72.0, 14.0, light);
        if let Some(border) = border {
            draw_line(9.0, 27.0, 85.0, 27.0, 2.0, border);
            draw_line(9.0, 27.0, 9.0, 74.0, 2.0, border);
            draw_line(9.0, 74.0, 85.0, 74.0, 2.0, border);
            draw_line(85.0, 27.0, 85.0, 74.0, 2.0, border);
        } 
    }

    fn render_cell(&self, index: usize, pokemon: &PokemonDisplay, selected: bool) {
        let offset = -14.0 + (24.0 * index as f32);
        let mut skip = false;
        if self.select.is_world.is_some() {
            if let Some(selected_index) = self.selected {
                let selected_index = selected_index == index;
                if selected_index || selected {
                    self.render_cell_color(offset, Self::SELECT_LIGHT, Self::SELECT_DARK, Some( if selected { Self::HOVER_BORDER } else { Self::SELECT_BORDER }));
                    skip = true;
                }
            }
        }
        if !skip {
            if selected {
                self.render_cell_color(offset, Self::HOVER_LIGHT, Self::HOVER_DARK, Some(Self::HOVER_BORDER));
            } else {
                self.render_cell_color(offset, Self::LIGHT, Self::DARK, None);
            }
        }
        self.render_ball(88.0, offset - 1.0, selected);
        self.render_pokemon(pokemon.icon, 87.0, offset - 8.0, selected);
        draw_text_left(0, &pokemon.name, TextColor::White, 119.0, offset);
        draw_text_left(0, &pokemon.level, TextColor::White, 129.0, offset + 9.0);
        self.render_health(pokemon, 170.0, offset + 6.0);
    }

    fn render_cell_color(&self, y: f32, light: Color, dark: Color, border: Option<Color>) { // 89 + 11
        draw_rectangle(98.0, y + 2.0, 138.0, 12.0, dark);
        let y1 = y + 14.5;
        draw_line(98.0, y1, 236.0, y1, 1.0, light);
        let y1 = y1 + 1.0;
        draw_line(98.0, y1, 236.0, y1, 1.0, dark);
        draw_rectangle(98.0, y + 16.0, 138.0, 4.0, light);
        if let Some(border) = border {
            let y1 = y + 1.0;
            draw_line(97.0, y1, 237.0, y1, 2.0, border);
            let y2 = y1 + 20.0;
            draw_line(97.0, y2, 237.0, y2, 2.0, border);
            draw_line(237.0, y1, 237.0, y2, 2.0, border);
        }
    }

    fn render_ball(&self, x: f32, y: f32, selected: bool) {
        draw_texture_ex(self.ball, x, y, WHITE, DrawTextureParams {
            source: Some(
                Rect::new(
                    0.0,
                    if selected { 24.0 } else { 0.0 },
                    20.0,
                    24.0
                )
            ),
            ..Default::default()
        });
    }

    fn render_pokemon(&self, icon: Texture2D, x: f32, y: f32, selected: bool) {
        let second = self.accumulator > PokemonDisplay::ICON_TICK;
        draw_texture_ex(icon, x - 3.0, if second && selected { y - 5.0 } else { y }, WHITE, DrawTextureParams {
            source: Some(
                    Rect::new(
                        0.0, 
                        if second { 32.0 } else { 0.0 }, 
                        32.0, 
                        32.0
                    )
                ),
            ..Default::default()
        });
    }

    fn render_health(&self, pokemon: &PokemonDisplay, x: f32, y: f32) {
        draw(self.health, x, y);
        draw_text_left(0, &pokemon.health.0, TextColor::White, x + 35.0, y + 5.0);
        let x = x + 15.0;
        draw_rectangle(x, y + 2.0, pokemon.health.1, 1.0, HealthBar::UPPER);
        draw_rectangle(x, y + 3.0, pokemon.health.1, 2.0, HealthBar::LOWER);
    }

    pub fn despawn(&mut self) {
        self.alive = false;
        self.select.alive = false;
    }

}

impl Reset for PartyGui {
    fn reset(&mut self) {
        self.cursor = 0;
        self.right_cursor = None;
        self.accumulator = 0.0;
        self.pokemon.clear();
    }
}