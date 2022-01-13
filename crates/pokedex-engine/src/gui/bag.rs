use core::ops::Deref;

use engine::{
    controls::{pressed, Control},
    graphics::{draw_cursor, draw_text_left, DrawParams, Texture},
    gui::Panel,
    text::TextColor,
    utils::HEIGHT,
    Context,
};
use firecore_engine::EngineContext;

use crate::pokedex::{
    item::{bag::Bag, Item, ItemId},
    Dex,
};

use crate::data::PokedexClientData;

use super::SizedStr;

// const WORLD_OPTIONS: &[&'static str] = &[
//     "Use",
//     "Give",
//     "Toss",
// ];

type TextOption = &'static [&'static str];

const BATTLE_OPTIONS: TextOption = &["Use"];

pub struct BagGui {
    alive: bool,

    background: Texture,
    cells: [Option<BagCell>; 8],
    hover: Option<HoverCell>,

    offset: usize,
    cursor: usize,

    selecting: bool,

    /// select menu cursor
    select_cursor: usize,

    update: bool,

    selected: Option<ItemId>,
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
    //         .skip(self.offset)
    //         .take(Self::SIZE)
    //         .enumerate()
    // }

    pub fn input<I>(&mut self, ctx: &Context, eng: &EngineContext, items: &mut Bag<I>) {
        match self.selecting {
            true => {
                // match self.select_text {
                // Some(text) => {
                if pressed(ctx, eng, Control::B) {
                    self.selecting = false;
                }
                if pressed(ctx, eng, Control::Up) && self.cursor > 0 {
                    self.select_cursor -= 1;
                }
                if pressed(ctx, eng, Control::Down) && self.cursor < BATTLE_OPTIONS.len() {
                    self.select_cursor += 1;
                }
                if pressed(ctx, eng, Control::A) {
                    match self.cursor {
                        0 => {}
                        1 => (), // cancel
                        _ => unreachable!("Selected an option that is not use/cancel"),
                    }
                    self.selecting = false;
                }
            }
            false => {
                if pressed(ctx, eng, Control::B) {
                    self.despawn();
                }
                if pressed(ctx, eng, Control::A) {
                    if self.cursor < items.len() {
                        self.spawn_select();
                    } else {
                        self.despawn();
                    }
                }
                if pressed(ctx, eng, Control::Up) && self.cursor > 0 {
                    self.cursor -= 1;
                }
                if pressed(ctx, eng, Control::Down) && self.cursor < items.len() {
                    self.cursor += 1;
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
        &mut self,
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

        self.hover = self
            .cells
            .get(self.cursor - self.offset)
            .map(Option::as_ref)
            .flatten()
            .is_some()
            .then(|| create(ctx, bag, self.cursor))
            .flatten();
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext) {
        self.background.draw(ctx, 0.0, 0.0, Default::default());
        for (index, cell) in self.cells.iter().enumerate() {
            if let Some(cell) = cell {
                let y = 11.0 + (index << 4) as f32;
                let color = DrawParams::color(TextColor::BLACK);
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
            11.0 + (self.cells.iter().filter(|c| c.is_some()).count() << 4) as f32,
            DrawParams::color(TextColor::BLACK),
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
        //                 color: TextColor::WHITE,
        //                 ..Default::default()
        //             },
        //         );
        //     }
        // }
        draw_cursor(
            ctx,
            eng,
            91.0,
            13.0 + (self.cursor << 4) as f32,
            Default::default(),
        );
        if self.selecting {
            // if let Some(text) = self.select_text {
            Panel::draw_text(
                ctx,
                eng,
                146.0,
                HEIGHT,
                94.0,
                BATTLE_OPTIONS,
                self.select_cursor,
                true,
                true,
            )
            // }
        }
    }

    fn spawn_select(&mut self) {
        self.selecting = true;
        self.select_cursor = 0;
    }

    // fn set_cell<'d>(&self, index: usize, stack: Option<&ItemRefStack<'d>>) {
    //     if let Some(cell) = self.items.get(index) {
    //         cell = stack.map(|stack| to_ascii4(stack.count).ok()).flatten())
    //     }
    // }

    pub fn take_selected_despawn<I: Clone>(&mut self, bag: &mut Bag<I>) -> Option<I> {
        self.selected
            .map(|selected| {
                self.selected = None;
                let item = bag.try_take(&selected, 1).map(|stack| stack.item);
                self.despawn();
                item
            })
            .flatten()
    }

    pub fn spawn(&mut self) {
        self.alive = true;
        // self.select_text = Some(BATTLE_OPTIONS));
    }

    pub fn despawn(&mut self) {
        self.alive = false;
        // self.items.clear()
    }

    pub fn alive(&self) -> bool {
        self.alive
    }
}
