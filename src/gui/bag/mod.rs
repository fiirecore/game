use crate::graphics::{byte_texture, draw_cursor, draw_o, draw_text_left};
use crate::gui::Panel;
use crate::input::{pressed, Control};
use crate::text::TextColor;
use atomic::Atomic;
use deps::tetra::{graphics::Texture, Context};
use pokedex::{
    item::{bag::Bag, ItemRef, ItemStackInstance},
    texture::ItemTextures,
};
use std::{
    cell::RefCell,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed},
};

static mut BAG_BACKGROUND: Option<Texture> = None;

fn bag_background(ctx: &mut Context) -> &Texture {
    unsafe {
        BAG_BACKGROUND.get_or_insert(byte_texture(
            ctx,
            include_bytes!("../../../assets/gui/bag/items.png"),
        ))
    }
}

// const WORLD_OPTIONS: &[&'static str] = &[
//     "Use",
//     "Give",
//     "Toss",
// ];

type TextOption = &'static [&'static str];

const BATTLE_OPTIONS: TextOption = &["Use"];

pub struct BagGui {
    alive: AtomicBool,
    background: Texture,
    cursor: AtomicUsize,

    selecting: AtomicBool,
    select: Panel,
    select_cursor: AtomicUsize,
    // select_text: Atomic<Option<TextOption>>,
    items: RefCell<Vec<ItemStackInstance>>,

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
                    self.select_cursor
                        .store(self.select_cursor.load(Relaxed) - 1, Relaxed);
                }
                if pressed(ctx, Control::Down) && cursor < BATTLE_OPTIONS.len() {
                    self.select_cursor
                        .store(self.select_cursor.load(Relaxed) + 1, Relaxed);
                }
                if pressed(ctx, Control::A) {
                    match cursor {
                        0 => {
                            self.selected.store(Some(cursor), Relaxed);
                        }
                        1 => (), // cancel
                        _ => unreachable!("Selected an option that is not use/cancel"),
                    }
                    self.selecting.store(false, Relaxed);
                }

                // }
                //     None => self.selecting = false,
                // }
            }
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
        self.background
            .draw(ctx, crate::graphics::position(0.0, 0.0));
        let mut items = self.items.borrow_mut();
        let cursor = self.cursor.load(Relaxed);
        for (index, item) in items.iter_mut().enumerate() {
            let y = 11.0 + (index << 4) as f32;
            draw_text_left(ctx, &1, &item.stack().item.name, &TextColor::Black, 98.0, y);
            draw_text_left(ctx, &1, "x", &TextColor::Black, 200.0, y);
            draw_text_left(ctx, &1, item.count(), &TextColor::Black, 208.0, y);
        }
        draw_text_left(
            ctx,
            &1,
            "Cancel",
            &TextColor::Black,
            98.0,
            11.0 + (items.len() << 4) as f32,
        );
        if let Some(item) = items.get(cursor) {
            let item = &item.stack().item;
            draw_o(
                ctx,
                <ItemTextures as deps::TextureManager>::try_get(&item.id),
                8.0,
                125.0,
            );
            for (index, line) in item.description.iter().enumerate() {
                draw_text_left(
                    ctx,
                    &1,
                    line,
                    &TextColor::White,
                    41.0,
                    117.0 + (index * 14) as f32,
                );
            }
        }
        draw_cursor(ctx, 91.0, 13.0 + (cursor << 4) as f32);
        if self.selecting.load(Relaxed) {
            // if let Some(text) = self.select_text {
            self.select.draw_text(
                ctx,
                146.0,
                util::HEIGHT,
                94.0,
                &BATTLE_OPTIONS,
                self.select_cursor.load(Relaxed),
                true,
                true,
            )
            // }
        }
    }

    fn spawn_select(&self) {
        self.selecting.store(true, Relaxed);
        self.select_cursor.store(0, Relaxed);
    }

    pub fn take_selected_despawn(&self) -> Option<ItemRef> {
        let selected = self.selected.load(Relaxed);
        selected
            .map(|selected| {
                self.selected.store(None, Relaxed);
                let items = self.items.borrow();
                let item = items[selected]
                    .stack()
                    .decrement()
                    .then(|| items[selected].stack().item);
                self.despawn();
                item
            })
            .flatten()
    }

    pub fn spawn(&self, bag: &mut Bag) {
        self.alive.store(true, Relaxed);
        // self.select_text.store(Some(BATTLE_OPTIONS), Relaxed);
        *self.items.borrow_mut() = bag.items.iter_mut().map(ItemStackInstance::from).collect();
    }

    pub fn despawn(&self) {
        self.alive.store(false, Relaxed);
    }

    pub fn alive(&self) -> bool {
        self.alive.load(Relaxed)
    }
}
