use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

use crate::texture::PokemonTextures;

use crate::pokedex::pokemon::{owned::OwnedPokemon, PokemonTexture};

use engine::egui;

pub struct SummaryGui {
    alive: AtomicBool,

    page: AtomicUsize,
    // headers: [&'static str; Self::PAGES],
    // pages: [Texture; Self::PAGES],
    // pokemon_background: Texture,
    pokemon: AtomicUsize,

    // offset: Offset,
    textures: Arc<PokemonTextures>,
}

// #[derive(Default)]
// struct Offset {
//     int: u8,
//     boolean: bool,
//     float: f32,
// }

impl SummaryGui {
    pub fn spawn(&self, pokemon: usize) {
        self.alive.store(true, Ordering::Relaxed);
        self.pokemon.store(pokemon, Ordering::Relaxed);
    }

    pub fn despawn(&self) {
        self.alive.store(false, Ordering::Relaxed);
    }

    pub fn alive(&self) -> bool {
        self.alive.load(Ordering::Relaxed)
    }

    pub fn new(textures: Arc<PokemonTextures>) -> Self {
        Self {
            alive: Default::default(),
            page: Default::default(),
            pokemon: Default::default(),
            textures,
        }
    }

    pub fn ui(&self, egui: &egui::Context, party: &[OwnedPokemon]) -> Option<()> {
        if self.alive() {
            egui::Window::new("Summary")
                .show(egui, |ui| {
                    egui::Grid::new("SummaryTabs").show(ui, |ui| {
                        if ui.button("Info").clicked() {
                            self.page.store(0, Ordering::Relaxed);
                        }
                        if ui.button("Skills").clicked() {
                            self.page.store(1, Ordering::Relaxed);
                        }
                        if ui.button("Moves").clicked() {
                            self.page.store(2, Ordering::Relaxed);
                        }
                        if ui.button("Exit").clicked() {
                            self.despawn();
                        }
                    });
                    ui.separator();
                    let index = self.pokemon.load(Ordering::Relaxed);
                    let pokemon = &party[index];
                    match self.page.load(Ordering::Relaxed) {
                        0 => {
                            if let Some((a, b)) = self.textures.egui_id(&pokemon.pokemon.id, PokemonTexture::Front) {
                                ui.image(a, b);
                            }
                            ui.label(pokemon.name());
                            ui.label(format!("Lv. {}", pokemon.level));
                        }
                        1 => {}
                        2 => {
                            egui::Grid::new("Summary Moves").show(ui, |ui| {
                                for m in pokemon.moves.iter() {
                                    ui.label(&m.0.name);
                                    ui.label(format!("PP: {}/{}", m.1, m.0.pp));
                                    ui.end_row();
                                    ui.separator();
                                    ui.end_row();
                                }
                            });
                        }
                        _ => unreachable!(),
                    }
                    Option::<()>::None
                })
                .and_then(|r| r.inner)
                .flatten()
        } else {
            None
        }
    }

    // pub fn draw(&self, ctx: &mut Context, eng: &EngineContext) {
    //     let current_page = self.page;
    //     let w = 114.0 + (current_page << 4) as f32;
    //     let rw = WIDTH - w;
    //     draw_rectangle(ctx, 0.0, 1.0, w, 15.0, Self::HEADER_LEFT);
    //     draw_rectangle(ctx, w, 1.0, rw, 16.0, Self::HEADER_RIGHT);
    //     draw_straight_line(ctx, 0.0, 16.5, w, true, 1.0, Self::HEADER_LEFT_DARK);
    //     draw_text_left(
    //         ctx,
    //         eng,
    //         &1,
    //         self.headers[current_page],
    //         5.0,
    //         1.0,
    //         DrawParams::color(TextColor::WHITE),
    //     );
    //     for page in 0..Self::PAGES {
    //         let color = if current_page < page {
    //             Self::HEADER_RIGHT_DARK
    //         } else if current_page == page {
    //             Panel::BACKGROUND
    //         } else {
    //             Self::HEADER_LEFT_DARK
    //         };
    //         draw_circle(ctx, 106.0 + (page << 4) as f32, 9.0, 6.0, color);
    //     }
    //     if let Some(summary) = &self.pokemon {
    //         self.pokemon_background
    //             .draw(ctx, 0.0, 17.0, DrawParams::default());
    //         summary.front.draw(
    //             ctx,
    //             28.0,
    //             summary.pos + self.offset.float,
    //             DrawParams::default(),
    //         );
    //         draw_text_left(
    //             ctx,
    //             eng,
    //             &1,
    //             LEVEL_PREFIX,
    //             5.0,
    //             19.0,
    //             DrawParams::color(TextColor::WHITE),
    //         );
    //         draw_text_left(
    //             ctx,
    //             eng,
    //             &1,
    //             &summary.level,
    //             15.0,
    //             19.0,
    //             DrawParams::color(TextColor::WHITE),
    //         );
    //         draw_text_left(
    //             ctx,
    //             eng,
    //             &1,
    //             &summary.name,
    //             41.0,
    //             19.0,
    //             DrawParams::color(TextColor::WHITE),
    //         );
    //         const TOP: f32 = 17.0;
    //         match self.page {
    //             0 => {
    //                 self.pages[0].draw(ctx, 0.0, TOP, Default::default());
    //                 draw_text_left(
    //                     ctx,
    //                     eng,
    //                     &1,
    //                     &summary.id,
    //                     168.0,
    //                     21.0,
    //                     DrawParams::color(TextColor::BLACK),
    //                 );
    //                 draw_text_left(
    //                     ctx,
    //                     eng,
    //                     &1,
    //                     &summary.name,
    //                     168.0,
    //                     36.0,
    //                     DrawParams::color(TextColor::BLACK),
    //                 );

    //                 for (index, display) in summary.types.iter().flatten().enumerate() {
    //                     let x = 168.0 + 37.0 * index as f32;
    //                     draw_rectangle(ctx, x, 52.0, 32.0, 6.0, display.upper);
    //                     draw_rectangle(ctx, x, 58.0, 32.0, 6.0, display.lower);
    //                     draw_text_center(
    //                         ctx,
    //                         eng,
    //                         &0,
    //                         display.name,
    //                         false,
    //                         x + 16.0,
    //                         52.0,
    //                         DrawParams::color(TextColor::WHITE),
    //                     )
    //                 }

    //                 // draw_text_left(1, &pokemon.item, &crate::TEXT_BLACK, 168.0, 96.0);
    //             }
    //             1 => {
    //                 self.pages[1].draw(ctx, 0.0, TOP, Default::default());
    //             }
    //             2 => {
    //                 self.pages[2].draw(ctx, 119.0, TOP, Default::default());
    //             }
    //             _ => unreachable!(),
    //         }
    //     }
    // }
}
