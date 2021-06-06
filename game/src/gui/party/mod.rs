use std::{cell::RefCell, sync::atomic::{
    AtomicBool, AtomicUsize,
    Ordering::Relaxed,
}};

use atomic::Atomic;
use crate::input::{pressed, Control};
use storage::{data, data_mut};
use crate::text::TextColor;
use deps::vec::ArrayVec;

use crate::graphics::{byte_texture, position, draw_line, draw_rectangle, draw_text_left};

use crate::tetra::{
    Context,
    graphics::{
        Color,
        Texture,
        Rectangle,
    },
};

use self::select::PartySelectMenu;
use self::summary::SummaryGui;

use super::health::HealthBar;
use super::pokemon::PokemonDisplay;

pub mod select;
pub mod summary;

pub struct PartyGui {

    alive: AtomicBool,
    
    select: PartySelectMenu,
    summary: SummaryGui,

    background: Texture,
    ball: Texture,
    health: Texture,
    
    pokemon: RefCell<ArrayVec<[PokemonDisplay; 6]>>,

    selected: Atomic<Option<usize>>,

    accumulator: Atomic<f32>,

    cursor: AtomicUsize,
    right_cursor: Atomic<Option<usize>>,

    exitable: AtomicBool,

}

impl PartyGui {

    const LIGHT: Color = Color::rgb(128.0 / 255.0, 192.0 / 255.0, 216.0 / 255.0);
    const DARK: Color = Color::rgb(56.0 / 255.0, 144.0 / 255.0, 216.0 / 255.0);

    const HOVER_LIGHT: Color = Color::rgb(168.0 / 255.0, 232.0 / 255.0, 248.0 / 255.0);
    const HOVER_DARK: Color = Color::rgb(120.0 / 255.0, 208.0 / 255.0, 232.0 / 255.0);

    const HOVER_BORDER: Color = Color::rgb(248.0 / 255.0, 112.0 / 255.0, 48.0 / 255.0);

    const SELECT_LIGHT: Color = Color::rgb(176.0 / 255.0, 248.0 / 255.0, 160.0 / 255.0);
    const SELECT_DARK: Color = Color::rgb(120.0 / 255.0, 216.0 / 255.0, 128.0 / 255.0);

    const SELECT_BORDER: Color = Color::rgb(248.0 / 255.0, 248.0 / 255.0, 112.0 / 255.0);

    const SELECT_CORNER: Color = Color::rgb(120.0 / 255.0, 152.0 / 255.0, 96.0 / 255.0);

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            alive: AtomicBool::new(false),
            select: PartySelectMenu::new(ctx),
            summary: SummaryGui::new(ctx),
            background: byte_texture(ctx, include_bytes!("../../../assets/gui/party/background.png")),
            ball: byte_texture(ctx, include_bytes!("../../../assets/gui/party/ball.png")),
            health: HealthBar::texture(ctx).clone(),
            accumulator: Atomic::new(0.0),
            pokemon: RefCell::new(ArrayVec::new()),
            cursor: AtomicUsize::new(0),
            right_cursor: Atomic::new(None),
            selected: Atomic::new(None),
            exitable: AtomicBool::new(true),
        }

    }

    pub fn on_spawn(&self, world: Option<bool>) {
        self.alive.store(true, Relaxed);
        self.reset();        
        self.select.is_world.store(world, Relaxed);
    }

    pub fn spawn(&self, party: ArrayVec<[PokemonDisplay; 6]>, is_world: Option<bool>, exitable: bool) {
        self.on_spawn(is_world);
        *self.pokemon.borrow_mut() = party;
        self.exitable.store(exitable, Relaxed);
    }

    pub fn spawn_world(&self) {
        self.on_spawn(Some(true));
        self.exitable.store(true, Relaxed);
        self.spawn(data().party.iter().map(|instance| PokemonDisplay::new(std::borrow::Cow::Borrowed(instance))).collect(), Some(true), true);
    }

    pub fn input(&self, ctx: &Context) {
        if self.summary.alive() {
            self.summary.input(ctx);
        } else if self.select.alive.load(Relaxed) {
            if let Some(action) = self.select.input(ctx) {
                let cursor = self.cursor.load(Relaxed);
                match action {
                    select::PartySelectAction::Select => {
                        self.selected.store(Some(cursor), Relaxed);
                        self.select.alive.store(false, Relaxed);
                    }
                    select::PartySelectAction::Summary => {
                        self.summary.spawn(self.pokemon.borrow()[cursor].clone());
                        self.select.alive.store(false, Relaxed);
                    }
                }
            }            
        } else {

            if pressed(ctx, Control::A) {
                let is_world = self.select.is_world.load(Relaxed);
                if let Some(selected) = self.take_selected() {
                    if let Some(is_world) = is_world {
                        if is_world {
                            self.pokemon.borrow_mut().swap(self.cursor.load(Relaxed), selected);
                            data_mut().party.swap(self.cursor.load(Relaxed), selected);
                        }
                    }
                } else {
                    if is_world.is_some() {
                        self.select.toggle();
                    } else {
                        self.selected.store(Some(self.cursor.load(Relaxed)), Relaxed);
                    }
                }
            } else {
                let cursor = self.cursor.load(Relaxed);
                if pressed(ctx, Control::Up) && cursor > 1 {
                    self.cursor.store(cursor - 1, Relaxed);
                }
                if pressed(ctx, Control::Down) {
                    if cursor < self.pokemon.borrow().len() - 1 {
                        self.cursor.store(cursor + 1, Relaxed);  
                    }        
                }
                if pressed(ctx, Control::Left) && cursor != 0 {
                    self.right_cursor.store(Some(cursor), Relaxed);
                    self.cursor.store(0, Relaxed);
                }
                if pressed(ctx, Control::Right) && cursor == 0 {
                    self.cursor.store(self.right_cursor.load(Relaxed).unwrap_or(1), Relaxed);
                }
                if (pressed(ctx, Control::B) || pressed(ctx, Control::Start)) && self.exitable.load(Relaxed) {
                    self.despawn();
                }
            }           
        }
    }

    pub fn update(&self, delta: f32) {
        if self.alive.load(Relaxed) {
            let acc = self.accumulator.load(Relaxed) + delta;
            self.accumulator.store(if acc > PokemonDisplay::ICON_TICK * 2.0 { 0.0 } else { acc }, Relaxed);
            if let Some(is_world) = self.select.is_world.load(Relaxed) {
                if is_world && self.summary.alive() {
                    self.summary.update(delta);
                }
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.summary.alive() {
            self.summary.draw(ctx);
        } else {
            self.background.draw(ctx, position(0.0, 0.0));
            self.pokemon.borrow().iter().enumerate().for_each(|(index, pokemon)| {
                if index == 0 {
                    self.draw_primary(ctx, pokemon);                    
                } else {
                    self.draw_cell(ctx, index, pokemon, self.cursor.load(Relaxed) == index);
                }
            });
            if self.select.is_world.load(Relaxed).is_some() {
                self.select.draw(ctx);
            }
        }
    }

    fn draw_primary(&self, ctx: &mut Context, pokemon: &PokemonDisplay) {
        let selected = self.cursor.load(Relaxed) == 0;
        let mut skip = false;
        if self.select.is_world.load(Relaxed).is_some() {
            if let Some(selected_index) = self.selected.load(Relaxed) {
                let selected_index = selected_index == 0;
                if selected_index || selected {
                    
                    draw_line(ctx, 10.5, 28.0, 45.0, false, 2.0, Self::SELECT_LIGHT);
                    draw_line(ctx, 10.0, 28.5, 74.0, true, 2.0, Self::SELECT_LIGHT);

                    draw_line(ctx, 83.5, 28.0, 45.0, false, 1.0, Self::SELECT_CORNER);
                    draw_line(ctx, 10.0, 72.5, 74.0, true, 1.0, Self::SELECT_CORNER);
                    
                    self.draw_primary_color(ctx, Self::SELECT_LIGHT, Self::SELECT_DARK, Some( if selected { Self::HOVER_BORDER } else { Self::SELECT_BORDER }));
                    skip = true;
                }
            }
        }
        if !skip {
            if selected {
                self.draw_primary_color(ctx, Self::HOVER_LIGHT, Self::HOVER_DARK, Some(Self::HOVER_BORDER));
            } else {
                self.draw_primary_color(ctx, Self::LIGHT, Self::DARK, None);
            }
        }
        self.draw_ball(ctx, 3.0, 20.0, selected);
        self.draw_pokemon(ctx, &pokemon.icon, 0.0, 20.0, selected);
        draw_text_left(ctx, &0, &pokemon.name, TextColor::White, 33.0, 36.0);
        draw_text_left(ctx, &0, &pokemon.level, TextColor::White, 41.0, 45.0);
        self.draw_health(ctx, pokemon, 17.0, 57.0);
    }

    fn draw_primary_color(&self, ctx: &mut Context, light: Color, dark: Color, border: Option<Color>) {
        draw_rectangle(ctx, 11.0, 29.0, 72.0, 27.0, dark);
        draw_line(ctx, 11.0, 56.5, 72.0, true, 1.0, light);
        draw_line(ctx, 11.0, 57.5, 72.0, true, 1.0, dark);
        draw_rectangle(ctx, 11.0, 58.0, 72.0, 14.0, light);
        if let Some(border) = border {
            draw_line(ctx, 9.0, 27.0, 76.0, true, 2.0, border);
            draw_line(ctx, 9.0, 27.0, 47.0, false, 2.0, border);
            draw_line(ctx, 9.0, 74.0, 75.0, true, 2.0, border);
            draw_line(ctx, 85.0, 27.0, 47.0, false, 2.0, border);
        } 
    }

    fn draw_cell(&self, ctx: &mut Context, index: usize, pokemon: &PokemonDisplay, selected: bool) {
        let offset = -14.0 + (24.0 * index as f32);
        let mut skip = false;
        if self.select.is_world.load(Relaxed).is_some() {
            if let Some(selected_index) = self.selected.load(Relaxed) {
                let selected_index = selected_index == index;
                if selected_index || selected {
                    self.draw_cell_color(ctx, offset, Self::SELECT_LIGHT, Self::SELECT_DARK, Some( if selected { Self::HOVER_BORDER } else { Self::SELECT_BORDER }));
                    skip = true;
                }
            }
        }
        if !skip {
            if selected {
                self.draw_cell_color(ctx, offset, Self::HOVER_LIGHT, Self::HOVER_DARK, Some(Self::HOVER_BORDER));
            } else {
                self.draw_cell_color(ctx, offset, Self::LIGHT, Self::DARK, None);
            }
        }
        self.draw_ball(ctx, 88.0, offset - 1.0, selected);
        self.draw_pokemon(ctx, &pokemon.icon, 87.0, offset - 8.0, selected);
        draw_text_left(ctx, &0, &pokemon.name, TextColor::White, 119.0, offset);
        draw_text_left(ctx, &0, &pokemon.level, TextColor::White, 129.0, offset + 9.0);
        self.draw_health(ctx, pokemon, 170.0, offset + 6.0);
    }

    fn draw_cell_color(&self, ctx: &mut Context, y: f32, light: Color, dark: Color, border: Option<Color>) { // 89 + 11
        draw_rectangle(ctx, 98.0, y + 2.0, 138.0, 12.0, dark);
        let y1 = y + 14.5;
        draw_line(ctx, 98.0, y1, 138.0, true, 1.0, light);
        let y1 = y1 + 1.0;
        draw_line(ctx, 98.0, y1, 138.0, true, 1.0, dark);
        draw_rectangle(ctx, 98.0, y + 16.0, 138.0, 4.0, light);
        if let Some(border) = border {
            let y1 = y + 1.0;
            const XLEN: f32 = 140.0;
            const YLEN: f32 = 20.0;
            draw_line(ctx, 97.0, y1, XLEN, true, 2.0, border);
            draw_line(ctx, 97.0, y1 + YLEN, XLEN, true, 2.0, border);
            draw_line(ctx, 237.0, y1, YLEN, false, 2.0, border);
        }
    }

    fn draw_ball(&self, ctx: &mut Context, x: f32, y: f32, selected: bool) {
        self.ball.draw_region(
            ctx, 
            Rectangle::new(
                0.0,
                if selected { 24.0 } else { 0.0 },
                20.0,
                24.0
            ), 
            position(x, y),
        );
    }

    fn draw_pokemon(&self, ctx: &mut Context, icon: &Texture, x: f32, y: f32, selected: bool) {
        let second = self.accumulator.load(Relaxed) > PokemonDisplay::ICON_TICK;
        icon.draw_region(
            ctx,
            Rectangle::new(
                0.0, 
                if second { 32.0 } else { 0.0 }, 
                32.0, 
                32.0
            ),
            position(x - 3.0, if second && selected { y - 5.0 } else { y }),
        );
    }

    fn draw_health(&self, ctx: &mut Context, pokemon: &PokemonDisplay, x: f32, y: f32) {
        self.health.draw(ctx, position(x, y));
        draw_text_left(ctx, &0, &pokemon.health.0, TextColor::White, x + 35.0, y + 5.0);
        let x = x + 15.0;
        draw_rectangle(ctx, x, y + 2.0, pokemon.health.1, 1.0, HealthBar::UPPER);
        draw_rectangle(ctx, x, y + 3.0, pokemon.health.1, 2.0, HealthBar::LOWER);
    }

    pub fn take_selected(&self) -> Option<usize> {
        let selected = self.selected.load(Relaxed);
        if selected.is_some() {
            self.selected.store(None, Relaxed);
            selected
        } else {
            None
        }
    }

    pub fn despawn(&self) {
        self.alive.store(false, Relaxed);
        self.select.alive.store(false, Relaxed);
    }

    pub fn alive(&self) -> bool {
        self.alive.load(Relaxed)
    }
    
    pub fn reset(&self) {
        self.cursor.store(0, Relaxed);
        self.right_cursor.store(None, Relaxed);
        self.accumulator.store(0.0, Relaxed);
        self.selected.store(None, Relaxed);
        self.pokemon.borrow_mut().clear();
    }

}