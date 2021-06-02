use std::{cell::RefCell, sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed}};

use atomic::Atomic;
use pokedex::{
    item::{
        ItemRef,
        ItemStackInstance,
    },
    texture::item_texture,
};

use storage::data_mut;

use crate::input::{pressed, Control};

use crate::text::TextColor;

use crate::gui::Panel;

use crate::graphics::{
    byte_texture,
    draw_o,
    draw_text_left,
    draw_text_right,
    draw_cursor,
};

use deps::tetra::{Context, graphics::Texture};

static mut BAG_BACKGROUND: Option<Texture> = None;

fn bag_background(ctx: &mut Context) -> &Texture {
    unsafe { BAG_BACKGROUND.get_or_insert(byte_texture(ctx, include_bytes!("../../../assets/gui/bag/items.png"))) }
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

    alive: AtomicBool,
    background: Texture,
    cursor: AtomicUsize,

    selecting: AtomicBool,
    select: Panel,
    select_cursor: AtomicUsize,
    // select_text: Atomic<Option<TextOption>>,

    items: RefCell<Vec<ItemStackInstance<'static>>>,

    selected: Atomic<Option<usize>>,

}

impl BagGui {

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            alive: AtomicBool::new(false),
            background: bag_background(ctx).clone(),
            cursor: AtomicUsize::new(0),
            selecting: AtomicBool::new(false),
            select: Panel::new(ctx),
            select_cursor: AtomicUsize::new(0),
            // select_text: Atomic::new(None),
            items: RefCell::new(Vec::new()),
            selected: Atomic::new(None),
        }
    }

    pub fn input(&self, ctx: &Context) {
        match self.selecting.load(Relaxed) {
            true => {
                // match self.select_text {
                    // Some(text) => {
                        let cursor = self.cursor.load(Relaxed);
                        if pressed(ctx, Control::B) {
                            self.selecting.store(false, Relaxed);
                        }
                        if pressed(ctx, Control::Up) && cursor > 0 {
                            self.select_cursor.store(self.select_cursor.load(Relaxed) - 1, Relaxed);
                        }
                        if pressed(ctx, Control::Down) && cursor < BATTLE_OPTIONS.len() {
                            self.select_cursor.store(self.select_cursor.load(Relaxed) + 1, Relaxed);
                        }
                        if pressed(ctx, Control::A) {
                            match cursor {
                                0 => {   
                                    self.selected.store(Some(cursor), Relaxed);
                                },
                                1 => (), // cancel
                                _ => unreachable!("Selected an option that is not use/cancel"),
                            }
                            self.selecting.store(false, Relaxed);          
                        }
                        
                    // }
                //     None => self.selecting = false,
                // }
            },
            false => {
                if pressed(ctx, Control::B) {
                    self.despawn();
                }
                let cursor = self.cursor.load(Relaxed);
                if pressed(ctx, Control::A) {
                    if cursor < self.items.borrow().len() {
                        self.spawn_select();
                    } else {
                        self.despawn();
                    }
                }
                if pressed(ctx, Control::Up) && cursor > 0 {
                    self.cursor.store(cursor - 1, Relaxed);
                }
                if pressed(ctx, Control::Down) {
                    if cursor < self.items.borrow().len() {
                        self.cursor.store(cursor + 1, Relaxed);
                    }
                }
            }
        }
        
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.background.draw(ctx, crate::graphics::position(0.0, 0.0));
        let items = self.items.borrow();
        for (index, item) in items.iter().enumerate() {
            let y = 11.0 + (index << 4) as f32;
            draw_text_left(ctx, &1, &item.stack.item.value().name, TextColor::Black, 98.0, y);
            draw_text_left(ctx, &1, "x", TextColor::Black, 200.0, y);
            draw_text_right(ctx, &1, &item.count_string, TextColor::Black, 221.0, y);
        }
        draw_text_left(ctx, &1, "Cancel", TextColor::Black, 98.0, 11.0 + (items.len() << 4) as f32);
        let cursor = self.cursor.load(Relaxed);
        if let Some(item) = items.get(cursor) {
            let item = item.stack.item.value();
            draw_o(ctx, item_texture(&item.id), 8.0, 125.0);
            for (index, line) in item.description.iter().enumerate() {
                draw_text_left(ctx, &1, line, TextColor::White, 41.0, 117.0 + (index * 14) as f32);
            }
        }
        draw_cursor(ctx, 91.0, 13.0 + (cursor << 4) as f32);
        if self.selecting.load(Relaxed) {
            // if let Some(text) = self.select_text {
                self.select.draw_text(ctx, 146.0, util::HEIGHT, 94.0, &BATTLE_OPTIONS, self.select_cursor.load(Relaxed), true, true)
            // }
        }
    }

    fn spawn_select(&self) {
        self.selecting.store(true, Relaxed);
        self.select_cursor.store(0, Relaxed);
    }

    pub fn take_selected_despawn(&self) -> Option<ItemRef> {
        let selected = self.selected.load(Relaxed);
        selected.map(|selected|{
            self.selected.store(None, Relaxed);
            let mut items = self.items.borrow_mut();
            let item = items[selected].decrement().then(|| items[selected].stack.item);
            self.despawn();
            item
        }).flatten()
    }

    pub fn spawn(&self) {
        self.alive.store(true, Relaxed);
        // self.select_text.store(Some(BATTLE_OPTIONS), Relaxed);
        *self.items.borrow_mut() = data_mut().bag.items.values_mut().map(|stack| ItemStackInstance {
            count_string: stack.count.to_string(),
            stack,
        }).collect();
    }

    pub fn despawn(&self) {
        self.alive.store(false, Relaxed);
        self.items.borrow_mut().clear();
    }

    pub fn alive(&self) -> bool {
        self.alive.load(Relaxed)
    }

}