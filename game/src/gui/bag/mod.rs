use std::ops::AddAssign;
use std::ops::SubAssign;

use data::{get, get_mut, player::PlayerSaves};
use firecore_pokedex::item::Item;
use firecore_pokedex::itemdex;
use firecore_util::text::TextColor;
use input::{pressed, Control};
use firecore_pokedex::item::ItemStackInstance;
use macroquad::prelude::Texture2D;
use select::BagSelectMenu;

use crate::textures::item_texture;
use crate::graphics::{
    byte_texture,
    draw,
    draw_o,
    draw_text_left,
    draw_text_right,
    draw_cursor,
};
use crate::textures::pokemon_texture;

use self::select::BagSelectAction;

use super::party::PartyGui;
use super::party::PartyGuiData;

pub mod select;

pub struct BagGui {

    pub alive: bool,

    background: Texture2D,
    select: BagSelectMenu,

    pub items: Vec<ItemStackInstance>,

    pub selected: BagOption,

    cursor: usize,

}

pub enum BagOption {
    Take(usize),
    Selected(usize),
    None,
}

impl Default for BagOption {
    fn default() -> Self {
        Self::None
    }
}

impl BagOption {
    pub fn take(&mut self) -> Option<usize> {
        match self {
            BagOption::Take(selected) => {
                let selected = *selected;
                *self = BagOption::None;
                Some(selected)
            },
            _ => None,
        }
    }
}

impl BagGui {

    pub fn new() -> Self {
        Self {
            alive: false,
            background: byte_texture(include_bytes!("../../../assets/gui/bag/items.png")),
            select: BagSelectMenu::new(),
            items: Vec::new(),
            selected: BagOption::None,
            cursor: 0,
        }
    }

    pub fn input(&mut self, party_gui: &mut PartyGui) {
        if party_gui.is_alive() {
            party_gui.input();
        } else if self.select.alive {
            if let Some(action) = self.select.input() {
                match action {
                    BagSelectAction::Use => {
                        self.selected = BagOption::Take(self.cursor);
                        self.select.alive = false;
                    },
                    BagSelectAction::Give => {
                        self.selected = BagOption::Selected(self.cursor);
                        if let Some(mut saves) = get_mut::<PlayerSaves>() {
                            let save = saves.get_mut();
                            party_gui.spawn(save.party.iter().map(|saved| pokedex::pokedex().get(&saved.id).map(|pokemon| -> PartyGuiData {

                                let mut types = Vec::with_capacity(if pokemon.data.secondary_type.is_some() { 2 } else { 1 });

                                types.push(super::party::pokemon_type_display(pokemon.data.primary_type));
            
                                if let Some(secondary) = pokemon.data.secondary_type {
                                    types.push(super::party::pokemon_type_display(secondary));
                                }

                                let max = firecore_pokedex::pokemon::instance::calculate_hp(pokemon.base.hp, saved.data.ivs.hp, saved.data.evs.hp, saved.data.level);
                                let current = saved.current_hp.unwrap_or(max);

                                PartyGuiData {
                                    texture: pokemon_texture(&saved.id, firecore_pokedex::pokemon::texture::PokemonTexture::Icon),
                                    id: saved.id,
                                    name: saved.data.nickname.as_ref().map(|nick| nick.clone()).unwrap_or(pokemon.data.name.to_ascii_uppercase()),
                                    level: format!("Lv{}", saved.data.level),
                                    hp: format!("{}/{}", current, max),
                                    types,
                                    item: saved.item.as_ref().map(|id| pokedex::itemdex().get(id)).flatten().map(|item| item.name.to_ascii_uppercase()).unwrap_or("NONE".to_owned()),
                                    health_width: super::health_bar::HealthBar::get_hp_width(current, max),

                                }
                            })).flatten().collect());
                        }
                        self.select.alive = false;
                    }
                    BagSelectAction::Toss => {
                        
                    }
                }
            }
        } else {
            if pressed(Control::B) {
                self.despawn();
            }
            if pressed(Control::A) {
                if self.cursor < self.items.len() {
                    self.select.spawn();
                } else {
                    self.despawn();
                }
            }
            if pressed(Control::Up) && self.cursor > 0 {
                self.cursor -= 1;
            }
            if pressed(Control::Down) && self.cursor < self.items.len() {
                self.cursor += 1;
            }
        }
        
    }

    pub fn update(&mut self, party_gui: &mut PartyGui) {
        if self.alive {
            if party_gui.is_alive() {
                if let Some(pokemon) = party_gui.selected.take() {
                    if let Some(mut saves) = get_mut::<PlayerSaves>() {
                        let save = saves.get_mut();
                        if let Some(pokemon) = save.party.get_mut(pokemon as usize) {
                            if let BagOption::Selected(selected) = self.selected {
                                let mut push_item = None;
                                if let Some(instance) = self.items.get_mut(selected) {
                                    if let Some(count) = save.items.get_mut(&instance.id) {
                                        count.sub_assign(1);
                                        instance.count -= 1;
                                    }
                                    if let Some(item) = pokemon.item.replace(instance.id) {
                                        if let Some(count) = save.items.get_mut(&item) {
                                            count.add_assign(1);
                                        } else {
                                            save.items.insert(item, 1);
                                        }
                                        push_item = Some(item);
                                    }
                                    if instance.count == 0 {
                                        if self.cursor > 0 {
                                            self.cursor -= 1;
                                        }
                                        self.items.remove(selected);
                                    } else {
                                        instance.count_string = instance.count.to_string();
                                    }

                                }
                                if let Some(push_item) = push_item {
                                    if let Some(item) = itemdex().get(&push_item) {
                                        if let Some(index) = self.items.iter().position(|instance| instance.id == push_item) {
                                            if let Some(item) = self.items.get_mut(index) {
                                                item.count += 1;
                                                item.count_string = item.count.to_string();
                                            }
                                        } else {  
                                            self.items.push(ItemStackInstance {
                                                item: item,
                                                id: push_item,
                                                count: 1,
                                                count_string: 1.to_string(),
                                            });
                                        }
                                    }
                                }
                            }
                            party_gui.despawn();
                        }

                    }
                }
            }
        }
    }

    pub fn render(&self) {
        if self.alive {
            draw(self.background, 0.0, 0.0);
            for (index, item) in self.items.iter().enumerate() {
                let y = 11.0 + (index << 4) as f32;
                draw_text_left(1, &item.item.name, TextColor::Black, 98.0, y);
                draw_text_left(1, "x", TextColor::Black, 200.0, y);
                draw_text_right(1, &item.count_string, TextColor::Black, 221.0, y);
            }
            draw_text_left(1, "Cancel", TextColor::Black, 98.0, 11.0 + (self.items.len() << 4) as f32);
            if let Some(item) = self.items.get(self.cursor) {
                draw_o(item_texture(&item.id), 8.0, 125.0);
                for (index, line) in item.item.description.iter().enumerate() {
                    draw_text_left(1, line, TextColor::White, 41.0, 117.0 + (index * 14) as f32);
                }
            }
            draw_cursor(91.0, 13.0 + (self.cursor << 4) as f32);
            self.select.render();
        }
    }

    pub fn take_selected_despawn(&mut self) -> Option<&Item> {
        self.selected.take().map(|selected| {
            let item = self.items.remove(selected);
            self.despawn();
            item.item
        })
    }

    pub fn spawn(&mut self, is_world: bool) {
        self.alive = true;
        self.select.is_world = is_world;
        if let Some(saves) = get::<PlayerSaves>() {
            let save = saves.get();
            self.items = save.items.iter().map(|(id, count)| itemdex().get(id).map(|item| {
                if 0.lt(count) {
                    Some(ItemStackInstance {
                        item,
                        id: *id,
                        count_string: count.to_string(),
                        count: *count,
                    })
                } else {
                    None
                }                
            })).flatten().flatten().collect();
        }
    }

    pub fn despawn(&mut self) {
        self.alive = false;
        self.items.clear();
        self.selected = BagOption::None;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

}