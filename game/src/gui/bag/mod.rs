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

use crate::gui::Panel;

use crate::graphics::{
    byte_texture,
    draw,
    draw_o,
    draw_text_left,
    draw_text_right,
    draw_cursor,
};

use macroquad::prelude::Texture2D;

static mut BAG_BACKGROUND: Option<Texture2D> = None;

fn bag_background() -> Texture2D {
    unsafe { *BAG_BACKGROUND.get_or_insert(byte_texture(include_bytes!("../../../assets/gui/bag/items.png"))) }
}

// const WORLD_OPTIONS: &[&'static str] = &[
//     "Use",
//     "Give",
//     "Toss",
// ];

type TextOption = &'static [&'static str];

const BATTLE_OPTIONS: TextOption = &[
    "Use",
];

pub struct BagGui {

    pub alive: bool,
    background: Texture2D,
    cursor: usize,

    selecting: bool,
    select: Panel,
    select_cursor: usize,
    select_text: Option<TextOption>,

    items: Vec<ItemStackInstance<'static>>,

    selected: Option<usize>,

}

impl BagGui {

    pub fn new() -> Self {
        Self {
            alive: false,
            background: bag_background(),
            cursor: 0,
            selecting: false,
            select: Panel::new(),
            select_cursor: 0,
            select_text: None,
            items: Vec::new(),
            selected: None,
        }
    }

    pub fn input(&mut self) {
        match self.selecting {
            true => {
                match self.select_text {
                    Some(text) => {
                        if pressed(Control::B) {
                            self.selecting = false;
                        }
                        if pressed(Control::Up) && self.cursor > 0 {
                            self.select_cursor -= 1;
                        }
                        if pressed(Control::Down) && self.cursor < text.len() {
                            self.select_cursor += 1;
                        }
                        if pressed(Control::A) {
                            match self.cursor {
                                0 => {   
                                    self.selected = Some(self.cursor);
                                },
                                1 => (), // cancel
                                _ => unreachable!("Selected an option that is not use/cancel"),
                            }
                            self.selecting = false;          
                        }
                        
                    }
                    None => self.selecting = false,
                }
            },
            false => {
                if pressed(Control::B) {
                    self.despawn();
                }
                if pressed(Control::A) {
                    if self.cursor < self.items.len() {
                        self.spawn_select();
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
        
    }

    pub fn render(&self) {
        draw(self.background, 0.0, 0.0);
        for (index, item) in self.items.iter().enumerate() {
            let y = 11.0 + (index << 4) as f32;
            draw_text_left(1, &item.stack.item.unwrap().name, TextColor::Black, 98.0, y);
            draw_text_left(1, "x", TextColor::Black, 200.0, y);
            draw_text_right(1, &item.count_string, TextColor::Black, 221.0, y);
        }
        draw_text_left(1, "Cancel", TextColor::Black, 98.0, 11.0 + (self.items.len() << 4) as f32);
        if let Some(item) = self.items.get(self.cursor) {
            let item = item.stack.item.unwrap();
            draw_o(item_texture(&item.id), 8.0, 125.0);
            for (index, line) in item.description.iter().enumerate() {
                draw_text_left(1, line, TextColor::White, 41.0, 117.0 + (index * 14) as f32);
            }
        }
        draw_cursor(91.0, 13.0 + (self.cursor << 4) as f32);
        if self.selecting {
            if let Some(text) = self.select_text {
                self.select.render_text(146.0, util::HEIGHT, 94.0, text, self.select_cursor, true, true)
            }
        }
    }

    fn spawn_select(&mut self) {
        self.selecting = true;
        self.select_cursor = 0;
    }

    pub fn take_selected_despawn(&mut self) -> Option<ItemRef> {
        self.selected.take().map(|selected| {
            let item = self.items[selected].decrement().then(|| self.items[selected].stack.item);
            self.despawn();
            item
        }).flatten()
    }

    pub fn spawn(&mut self) {
        self.alive = true;
        self.select_text = Some(BATTLE_OPTIONS);
        self.items = data_mut().bag.items.values_mut().map(|stack| ItemStackInstance {
            count_string: stack.count.to_string(),
            stack,
        }).collect();
    }

    pub fn despawn(&mut self) {
        self.alive = false;
        self.items.clear();
    }

}