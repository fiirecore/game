mod select;

use core::ops::Deref;

use crate::pokedex::item::{bag::Bag, Item, ItemId};

use engine::egui;

use crate::data::PokedexClientData;

// const WORLD_OPTIONS: &[&'static str] = &[
//     "Use",
//     "Give",
//     "Toss",
// ];

pub struct BagGui<D: Deref<Target = PokedexClientData>> {
    alive: bool,
    data: D,
    select: select::BagSelect,
}

pub enum BagAction {
    Use(ItemId),
}

impl<D: Deref<Target = PokedexClientData>> BagGui<D> {
    pub const SIZE: usize = 8;

    pub fn new(data: D) -> Self {
        Self {
            alive: Default::default(),
            data,
            select: Default::default(),
        }
    }

    pub fn ui<I: Deref<Target = Item>>(
        &mut self,
        egui: &egui::Context,
        bag: &mut Bag<I>,
    ) -> Option<BagAction> {
        if self.alive {
            egui::Window::new("Bag").show(egui, |ui| {
                egui::Grid::new("Items").show(ui, |ui| {
                    for stack in bag.iter().filter(|stack| stack.count != 0) {
                        let (id, size) = self
                            .data
                            .item_textures
                            .egui_id(&stack.item.id)
                            .copied()
                            .unwrap_or((Default::default(), (2.0, 2.0)));
                        if ui.add(egui::ImageButton::new(id, size)).clicked() {
                            if !self.select.alive() {
                                self.select.spawn(stack.item.id);
                            }
                        }

                        ui.label(&stack.item.name);

                        let mut count = [0u8; 5];

                        use std::io::Write;

                        if let Ok(()) = write!(&mut count as &mut [u8], "x{}", stack.count) {
                            if let Ok(count) = std::str::from_utf8(&count) {
                                ui.label(count);
                            }
                        }

                        ui.end_row();
                    }
                    if ui.button("Close").clicked() {
                        self.alive = false;
                    }
                })
            });

            if self.select.alive() {
                if let Some(action) = self.select.ui(egui) {
                    match action {
                        select::SelectAction::Select(item) => {
                            return Some(BagAction::Use(item));
                        }
                    }
                }
            }
        }
        None
        // self.background.draw(ctx, 0.0, 0.0, Default::default());
        // for (index, cell) in self.cells.iter().enumerate() {
        //     if let Some(cell) = cell {
        //         let y = 11.0 + (index << 4) as f32;
        //         let color = DrawParams::color(TextColor::BLACK);
        //         draw_text_left(app, plugins, &1, &cell.name, 98.0, y, color);
        //         draw_text_left(app, plugins, &1, "x", 200.0, y, color);
        //         draw_text_left(app, plugins, &1, &cell.count, 208.0, y, color);
        //     }
        // }
        // draw_text_left(
        //     ctx,
        //     eng,
        //     &1,
        //     "Cancel",
        //     98.0,
        //     11.0 + (self.cells.iter().filter(|c| c.is_some()).count() << 4) as f32,
        //     DrawParams::color(TextColor::BLACK),
        // );
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
        // draw_cursor(
        //     ctx,
        //     eng,
        //     91.0,
        //     13.0 + (self.cursor << 4) as f32,
        //     Default::default(),
        // );
        // if self.selecting {
        //     // if let Some(text) = self.select_text {
        //     Panel::draw_text(
        //         ctx,
        //         eng,
        //         146.0,
        //         HEIGHT,
        //         94.0,
        //         BATTLE_OPTIONS,
        //         self.select_cursor,
        //         true,
        //         true,
        //     )
        //     // }
        // }
    }

    // fn spawn_select(&mut self) {
    //     self.selecting = true;
    //     self.select_cursor = 0;
    // }

    // // fn set_cell<'d>(&self, index: usize, stack: Option<&ItemRefStack<'d>>) {
    // //     if let Some(cell) = self.items.get(index) {
    // //         cell = stack.map(|stack| to_ascii4(stack.count).ok()).flatten())
    // //     }
    // // }

    // pub fn take_selected_despawn<I: Clone>(&mut self, bag: &mut Bag<I>) -> Option<I> {
    //     self.selected
    //         .map(|selected| {
    //             self.selected = None;
    //             let item = bag.try_take(&selected, 1).map(|stack| stack.item);
    //             self.despawn();
    //             item
    //         })
    //         .flatten()
    // }

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
