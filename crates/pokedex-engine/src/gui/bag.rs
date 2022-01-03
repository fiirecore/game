use core::{cell::Cell, ops::Deref};

use engine::{
    controls::{pressed, Control},
    graphics::{draw_cursor, draw_text_left, DrawParams, Texture},
    gui::Panel,
    text::MessagePage,
    utils::HEIGHT,
    Context,
};
use firecore_engine::EngineContext;

use crate::pokedex::{
    item::{bag::Bag, Item, ItemId},
    Dex,
};

use crate::data::PokedexClientData;

use super::{cellref, SizedStr};

// const WORLD_OPTIONS: &[&'static str] = &[
//     "Use",
//     "Give",
//     "Toss",
// ];

type TextOption = &'static [&'static str];

const BATTLE_OPTIONS: TextOption = &["Use"];

pub struct BagGui {
    alive: Cell<bool>,

    background: Texture,
    cells: [Cell<Option<BagCell>>; 8],
    hover: Cell<Option<HoverCell>>,

    offset: Cell<usize>,
    cursor: Cell<usize>,

    selecting: Cell<bool>,

    /// select menu cursor
    select_cursor: Cell<usize>,

    update: Cell<bool>,

    selected: Cell<Option<ItemId>>,
}

struct BagCell {
    name: SizedStr<20>,
    count: SizedStr<4>,
}

struct HoverCell {
    descripton: String,
    texture: Texture,
}

impl BagGui {
    pub const SIZE: usize = 8;

    pub fn new(ctx: &PokedexClientData) -> Self {
        Self {
            alive: Default::default(),
            background: ctx.bag_background.clone(),
            cells: Default::default(),
            hover: Default::default(),
            cursor: Default::default(),
            offset: Default::default(),
            selecting: Default::default(),
            select_cursor: Default::default(),
            update: Default::default(),
            // items: Default::default(),
            selected: Default::default(),
        }
    }

    // fn update_cells<I>(&self, bag: &Bag<I>) {
    //     for cell in self.cells.iter() {}
    //     bag.iter()
    //         .skip(self.offset.get())
    //         .take(Self::SIZE)
    //         .enumerate()
    // }

    pub fn input<I>(&self, ctx: &Context, eng: &EngineContext, items: &mut Bag<I>) {
        match self.selecting.get() {
            true => {
                // match self.select_text {
                // Some(text) => {
                let cursor = self.cursor.get();
                if pressed(ctx, eng, Control::B) {
                    self.selecting.set(false);
                }
                if pressed(ctx, eng, Control::Up) && cursor > 0 {
                    self.select_cursor.set(self.select_cursor.get() - 1);
                }
                if pressed(ctx, eng, Control::Down) && cursor < BATTLE_OPTIONS.len() {
                    self.select_cursor.set(self.select_cursor.get() + 1);
                }
                if pressed(ctx, eng, Control::A) {
                    match cursor {
                        0 => {}
                        1 => (), // cancel
                        _ => unreachable!("Selected an option that is not use/cancel"),
                    }
                    self.selecting.set(false);
                }

                // }
                //     None => self.selecting = false,
                // }
            }
            false => {
                if pressed(ctx, eng, Control::B) {
                    self.despawn();
                }
                let cursor = self.cursor.get();
                if pressed(ctx, eng, Control::A) {
                    if cursor < items.len() {
                        self.spawn_select();
                    } else {
                        self.despawn();
                    }
                }
                if pressed(ctx, eng, Control::Up) && cursor > 0 {
                    self.cursor.set(cursor - 1);
                }
                if pressed(ctx, eng, Control::Down) && cursor < items.len() {
                    self.cursor.set(cursor + 1);
                }
            }
        }
    }

    pub fn update_cells_uninit<'d, I: Deref<Target = Item>>(
        &self,
        ctx: &PokedexClientData,
        dex: &'d dyn Dex<'d, Item, I>,
        bag: &Bag<ItemId>,
    ) {
    }

    pub fn update_cells_init<I: Deref<Target = Item>>(
        &self,
        ctx: &PokedexClientData,
        bag: &Bag<I>,
    ) {
        fn create<I: Deref<Target = Item>>(
            ctx: &PokedexClientData,
            bag: &Bag<I>,
            cursor: usize,
        ) -> Option<HoverCell> {
            let (.., item) = bag.iter().enumerate().find(|(i, ..)| i == &cursor)?;
            Some(HoverCell {
                descripton: item.item.description.clone(),
                texture: ctx.item_textures.get(&item.item.id)?.clone(),
            })
        }

        let cursor = self.cursor.get();

        if self
            .cells
            .get(cursor - self.offset.get())
            .map(cellref)
            .map(Option::as_ref)
            .flatten()
            .is_some()
        {
            self.hover.set(create(ctx, bag, cursor));
        } else {
            self.hover.set(None);
        }
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext) {
        self.background.draw(ctx, 0.0, 0.0, Default::default());
        let cursor = self.cursor.get();
        for (index, cell) in self.cells.iter().enumerate() {
            if let Some(cell) = super::cellref(cell).as_ref() {
                let y = 11.0 + (index << 4) as f32;
                let color = DrawParams::color(MessagePage::BLACK);
                draw_text_left(ctx, eng, &1, &cell.name, 98.0, y, color);
                draw_text_left(ctx, eng, &1, "x", 200.0, y, color);
                draw_text_left(ctx, eng, &1, &cell.count, 208.0, y, color);
            }
        }
        draw_text_left(
            ctx,
            eng,
            &1,
            "Cancel",
            98.0,
            11.0 + (self
                .cells
                .iter()
                .filter(|c| super::cellref(c).is_some())
                .count()
                << 4) as f32,
            DrawParams::color(MessagePage::BLACK),
        );
        // if let Some(stack) = self.get_item_at_cursor(bag).map(|id| bag.get(id)).flatten() {
        //     if let Some(texture) = dex.item_textures.try_get(&stack.item.id) {
        //         texture.draw(ctx, 8.0, 125.0, Default::default());
        //     }
        //     for (index, line) in stack.item.description.lines().enumerate() {
        //         draw_text_left(
        //             ctx,
        //             &1,
        //             line,
        //             41.0,
        //             117.0 + (index * 14) as f32,
        //             DrawParams {
        //                 color: MessagePage::WHITE,
        //                 ..Default::default()
        //             },
        //         );
        //     }
        // }
        draw_cursor(ctx, eng, 91.0, 13.0 + (cursor << 4) as f32, Default::default());
        if self.selecting.get() {
            // if let Some(text) = self.select_text {
            Panel::draw_text(
                ctx,
                eng, 
                146.0,
                HEIGHT,
                94.0,
                BATTLE_OPTIONS,
                self.select_cursor.get(),
                true,
                true,
            )
            // }
        }
    }

    fn spawn_select(&self) {
        self.selecting.set(true);
        self.select_cursor.set(0);
    }

    // fn set_cell<'d>(&self, index: usize, stack: Option<&ItemRefStack<'d>>) {
    //     if let Some(cell) = self.items.get(index) {
    //         cell.set(stack.map(|stack| to_ascii4(stack.count).ok()).flatten())
    //     }
    // }

    pub fn take_selected_despawn<I: Clone>(&self, bag: &mut Bag<I>) -> Option<I> {
        let selected = self.selected.get();
        selected
            .map(|selected| {
                self.selected.set(None);
                let item = bag.try_take(&selected, 1).map(|stack| stack.item);
                self.despawn();
                item
            })
            .flatten()
    }

    pub fn spawn(&self) {
        self.alive.set(true);
        // self.select_text.set(Some(BATTLE_OPTIONS));
    }

    pub fn despawn(&self) {
        self.alive.set(false);
        // self.items.clear()
    }

    pub fn alive(&self) -> bool {
        self.alive.get()
    }
}
