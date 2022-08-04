use std::sync::Arc;

use pokengine::{
    engine::{
        graphics::{Color, Draw, DrawImages, DrawShapes, Texture},
        text::{MessagePage, MessageState},
        App,
    },
    pokedex::Money, texture::TrainerGroupTextures,
};

use crate::{
    players::{GuiLocalPlayer, GuiRemotePlayers},
    ui::{text::BattleMessageState, BattleGui},
};

pub struct BattleCloser {
    trainers: Arc<TrainerGroupTextures>,
    alive: bool,
    trainer: Option<Texture>,
    offset: f32,
    alpha: f32,
}

impl BattleCloser {
    const XPOS: f32 = 172.0; // 144 = pokemon

    pub fn new(trainers: Arc<TrainerGroupTextures>) -> Self {
        Self {
            trainers,
            alive: Default::default(),
            trainer: Default::default(),
            offset: Default::default(),
            alpha: Default::default(),
        }
    }

    pub fn spawn<ID: PartialEq>(
        &mut self,
        local: &GuiLocalPlayer<ID>,
        opponents: &GuiRemotePlayers<ID>,
        winner: Option<&ID>,
        text: &mut BattleMessageState,
    ) {
        self.reset();
        self.alive = true;
        match winner == Some(local.player.id()) {
            true => {
                // crate::engine::log::debug!("todo set trainer textures and name in intro");

                // text.reset();
                // text.clear();

                // text.push(MessagePage {
                //     lines: vec![
                //         String::from("Player defeated"),
                //         format!("{} {}!", trainer_data.prefix, trainer_data.name),
                //     ],
                //     wait: None,
                // });

                let text = text.get_or_insert_with(MessageState::default);

                match opponents.players.get_index(opponents.current) {
                    Some((.., opponent)) => {
                        if let Some(trainer) = opponent.trainer.as_ref() {
                            self.trainer = self
                                .trainers
                                .get(&trainer.texture)
                                .cloned();

                            text.pages.extend_from_slice(&trainer.defeat);
                        }
                    }
                    None => {}
                }

                let gain = opponents
                    .players
                    .values()
                    .flat_map(|o| o.trainer.as_ref().map(|t| t.worth))
                    .sum::<Money>();

                if gain != 0 {
                    text.pages.push(MessagePage {
                        lines: vec![
                            format!("{} got ${}", local.player.name(), gain),
                            String::from("for winning!"),
                        ],
                        ..Default::default()
                    });
                }
            }
            false => {}
        }
    }

    pub fn update(&mut self, app: &mut App, text: &BattleMessageState) {
        let delta = app.timer.delta_f32();
        if let BattleMessageState::Running(text) = text {
            // text.update(app, plugins, delta);
            if text.page() == 1 && self.offset > Self::XPOS {
                self.offset += 300.0 * delta;
                if self.offset < Self::XPOS {
                    self.offset = Self::XPOS;
                }
            }
        } else {
            self.alpha += delta * 4.5;
        }
    }

    pub fn world_active(&self) -> bool {
        self.alpha > 1.0
    }

    pub fn draw<ID>(
        &self,
        draw: &mut Draw,
        elements: &BattleGui<ID>,
        local: Option<&GuiLocalPlayer<ID>>,
    ) {
        elements.background.draw(draw, 0.0);
        elements.draw_panel(draw);
        if let Some(local) = local {
            elements
                .pokemon
                .draw_local(draw, local, Default::default(), Color::WHITE);
        }
        match self.trainer.as_ref() {
            Some(texture) => {
                let w = draw.width();
                draw.image(texture)
                    .position(w - self.offset, 74.0 - texture.height());
            }
            None => (),
        }
        draw.rect((0.0, 0.0), (draw.width(), draw.height()))
            .color(Color::BLACK)
            .alpha(self.alpha);
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

    pub fn finished(&self) -> bool {
        self.alpha > 2.0
    }

    pub fn reset(&mut self) {
        self.trainer = None;
        self.alpha = 0.0;
        self.offset = 0.0;
    }
}
