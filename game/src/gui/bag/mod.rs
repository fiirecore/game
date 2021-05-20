use pokedex::{
    item::{
        ItemRef,
        ItemStackInstance,
    },
    texture::item_texture,
};

use storage::data_mut;

use input::{pressed, Control};

use crate::text::TextColor;

use crate::graphics::{
    byte_texture,
    draw,
    draw_o,
    draw_text_left,
    draw_text_right,
    draw_cursor,
};

use macroquad::prelude::Texture2D;

use self::select::{BagSelectMenu, BagSelectAction};

pub mod select;

pub struct BagGui {

    pub alive: bool,

    background: Texture2D,
    select: BagSelectMenu,

    items: Vec<ItemStackInstance<'static>>,

    selected: Option<usize>,

    cursor: usize,

}

impl BagGui {

    pub fn new() -> Self {
        Self {
            alive: false,
            background: byte_texture(include_bytes!("../../../assets/gui/bag/items.png")),
            select: BagSelectMenu::new(),
            items: Vec::new(),
            selected: None,
            cursor: 0,
        }
    }

    pub fn input(&mut self) {
        // if party_gui.is_alive() {
        //     party_gui.input();
        // } else 
        if self.select.alive {
            if let Some(action) = self.select.input() {
                match action {
                    BagSelectAction::Use => {
                        self.selected = Some(self.cursor);
                        self.select.alive = false;
                    },
                    // BagSelectAction::Give => todo!(),//{
                    //     // self.selected = BagOption::Selected(self.cursor);
                    //     // party_gui.spawn(data().party.iter().map(|instance| PokemonDisplay::new(std::borrow::Cow::Borrowed(instance))).collect(), None, true);
                    //     // self.select.alive = false;
                    // // }
                    // BagSelectAction::Toss => todo!(),
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

    // pub fn update(&mut self, party_gui: &mut PartyGui) {
        // if self.alive {
        //     if party_gui.is_alive() {
        //         if let Some(pokemon) = party_gui.selected.take() {
        //             let save = data_mut();
        //             if let Some(pokemon) = save.party.get_mut(pokemon) {
        //                 if let BagOption::Selected(selected) = self.selected {
        //                     todo!()
        //                     // let mut push_item = None;
        //                     // if let Some(instance) = self.items.get_mut(selected) {
        //                     //     if let Some(count) = save.items.get_mut(&instance.stack.item.id) {
        //                     //         count.sub_assign(1);
        //                     //         instance.stack.count -= 1;
        //                     //     }
        //                     //     if let Some(item) = pokemon.item.replace(instance.stack.item) {
        //                     //         if let Some(count) = save.items.get_mut(&item.id) {
        //                     //             count.add_assign(1);
        //                     //         } else {
        //                     //             save.items.insert(item.id, 1);
        //                     //         }
        //                     //         push_item = Some(item);
        //                     //     }
        //                     //     if instance.stack.count == 0 {
        //                     //         if self.cursor > 0 {
        //                     //             self.cursor -= 1;
        //                     //         }
        //                     //         self.items.remove(selected);
        //                     //     } else {
        //                     //         instance.count_string = instance.stack.count.to_string();
        //                     //     }

        //                     // }
        //                     // if let Some(push_item) = push_item {
        //                     //     if let Some(index) = self.items.iter().position(|instance| instance.stack.item.id == push_item.id) {
        //                     //         if let Some(item) = self.items.get_mut(index) {
        //                     //             item.stack.count += 1;
        //                     //             item.count_string = item.stack.count.to_string();
        //                     //         }
        //                     //     } else {  
        //                     //         self.items.push(ItemStackInstance {
        //                     //             stack: ItemStack {
        //                     //                 item: push_item,
        //                     //                 count: 1,
        //                     //             },
        //                     //             count_string: 1.to_string(),
        //                     //         });
        //                     //     }
        //                     // }
        //                 }
        //                 party_gui.despawn();
        //             }
        //         }
        //     }
        // }
    // }

    pub fn render(&self) {
        draw(self.background, 0.0, 0.0);
        for (index, item) in self.items.iter().enumerate() {
            let y = 11.0 + (index << 4) as f32;
            draw_text_left(1, &item.stack.item.value().name, TextColor::Black, 98.0, y);
            draw_text_left(1, "x", TextColor::Black, 200.0, y);
            draw_text_right(1, &item.count_string, TextColor::Black, 221.0, y);
        }
        draw_text_left(1, "Cancel", TextColor::Black, 98.0, 11.0 + (self.items.len() << 4) as f32);
        if let Some(item) = self.items.get(self.cursor) {
            let item = item.stack.item.value();
            draw_o(item_texture(&item.id), 8.0, 125.0);
            for (index, line) in item.description.iter().enumerate() {
                draw_text_left(1, line, TextColor::White, 41.0, 117.0 + (index * 14) as f32);
            }
        }
        draw_cursor(91.0, 13.0 + (self.cursor << 4) as f32);
        self.select.render();
    }

    pub fn take_selected_despawn(&mut self) -> Option<ItemRef> {
        self.selected.take().map(|selected| {
            let item = self.items[selected].decrement().then(|| self.items[selected].stack.item);
            self.despawn();
            item
        }).flatten()
    }

    pub fn spawn(&mut self, is_world: bool) {
        self.alive = true;
        self.select.is_world = is_world;
        self.items = data_mut().items.values_mut().map(|stack| ItemStackInstance {
            count_string: stack.count.to_string(),
            stack,
        }).collect();
    }

    pub fn despawn(&mut self) {
        self.alive = false;
        self.items.clear();
    }

}