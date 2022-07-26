use crate::pokedex::item::ItemId;

use crate::engine::egui;

#[derive(Default)]
pub struct BagSelect {
    alive: bool,
    item: Option<ItemId>,
}

pub enum SelectAction {
    Select(ItemId),
}

impl BagSelect {
    pub fn spawn(&mut self, item: ItemId) {
        self.alive = true;
        self.item = Some(item);
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

    pub fn ui(&mut self, egui: &egui::Context) -> Option<SelectAction> {
        egui::Window::new("Bag: Select")
            .show(egui, |ui| {
                if ui.button("Select").clicked() {
                    if let Some(item) = self.item.take() {
                        self.alive = false;
                        return Some(SelectAction::Select(item));
                    }
                }
                None
            })
            .and_then(|i| i.inner)
            .flatten()
    }
}
