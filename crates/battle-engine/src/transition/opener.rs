use crate::{
    context::BattleGuiData,
    players::{GuiLocalPlayer, GuiRemotePlayers},
    ui::BattleGui,
};
use battle::prelude::BattleType;
use core::ops::Deref;
use pokengine::{
    engine::{
        graphics::{Color, Draw, DrawExt, DrawImages, DrawParams, DrawShapes, Texture},
        math::{const_vec2, Rect, Vec2},
        text::{MessagePage, MessageState},
        App,
    },
    pokedex::{item::Item, moves::Move, pokemon::Pokemon},
    PokedexClientData,
};

pub struct BattleOpener<D: Deref<Target = PokedexClientData> + Clone> {
    dexengine: D,
    type_: BattleOpenerType,
    grass: Texture,
    player: Texture,
    wait: f32,
    offset: f32,
    rect_size: f32,
    shrink_by: f32,
    leaving: bool,
}

enum BattleOpenerType {
    Wild(Vec2),
    Trainer(Option<Texture>),
    None,
}

impl BattleOpenerType {
    const GRASS_WIDTH: f32 = 128.0;
    const GRASS_HEIGHT: f32 = 47.0;
    pub const WILD: Self = Self::Wild(const_vec2!([Self::GRASS_WIDTH, Self::GRASS_HEIGHT]));
}

impl<D: Deref<Target = PokedexClientData> + Clone> BattleOpener<D> {
    const LIGHTGRAY: Color = Color::new(0.78, 0.78, 0.78, 1.0);
    const RECT_SIZE: f32 = 80.0;
    const SHRINK_BY_DEF: f32 = 1.0;
    const SHRINK_BY_FAST: f32 = 4.0;
    const OFFSET: f32 = 153.0 * 2.0;
    const WAIT: f32 = 0.5;
    const PLAYER_T1: f32 = 42.0;
    const PLAYER_T2: f32 = Self::PLAYER_T1 + 18.0;
    const PLAYER_T3: f32 = Self::PLAYER_T2 + 18.0;
    const PLAYER_DESPAWN: f32 = 104.0;
    const FINAL_TRAINER_OFFSET: f32 = 126.0;

    pub fn new(dexengine: D, ctx: &BattleGuiData) -> Self {
        Self {
            dexengine: dexengine,
            type_: BattleOpenerType::None,
            grass: ctx.grass.clone(),
            wait: Self::WAIT,
            rect_size: Self::RECT_SIZE,
            shrink_by: Self::SHRINK_BY_DEF,
            offset: Self::OFFSET,
            player: ctx.player.clone(),
            leaving: false,
        }
    }
}

impl<D: Deref<Target = PokedexClientData> + Clone> BattleOpener<D> {
    pub fn spawn<
        ID,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        elements: &mut BattleGui<ID, D, M>,
        local: &GuiLocalPlayer<ID, P, M, I>,
        remotes: &GuiRemotePlayers<ID, P>,
    ) {
        match local.data.type_ {
            BattleType::Wild => {
                self.type_ = BattleOpenerType::WILD;
            }
            BattleType::Trainer | BattleType::GymLeader => {
                if let Some((.., remote)) = remotes.players.get_index(remotes.current) {
                    if let Some(trainer) = remote.trainer.as_ref() {
                        self.type_ = BattleOpenerType::Trainer(
                            self.dexengine
                                .trainer_group_textures
                                .get(&trainer.texture)
                                .cloned(),
                        );
                    }
                    elements
                        .trainer
                        .spawn(local.player.pokemon.len(), remote.pokemon.len());
                }
            }
        }
    }

    pub fn update<
        ID,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        app: &mut App,
        elements: &mut BattleGui<ID, D, M>,
        local: &GuiLocalPlayer<ID, P, M, I>,
        remotes: &GuiRemotePlayers<ID, P>,
    ) -> bool {
        let delta = app.timer.delta_f32();
        let half_finished = ((match &self.type_ {
            BattleOpenerType::Wild(offset) => offset.y <= 0.0,
            _ => true,
        }) && self.offset <= 0.0)
            || self.leaving;

        match half_finished {
            false => {
                match self.wait < 0.0 {
                    false => self.wait -= delta,
                    true => {
                        if self.offset > 0.0 {
                            self.offset -= 120.0 * delta;
                            if self.offset < 0.0 {
                                self.offset = 0.0;
                            }
                        }
                        if self.rect_size > 0.0 {
                            if self.rect_size > 0.0 {
                                self.rect_size -= self.shrink_by * 60.0 * delta;
                                if self.rect_size < 0.0 {
                                    self.rect_size = 0.0;
                                }
                            } else {
                                self.rect_size = 0.0;
                            }
                            if self.rect_size <= 58.0 && self.shrink_by != Self::SHRINK_BY_FAST {
                                self.shrink_by = Self::SHRINK_BY_FAST;
                            }
                        }
                    }
                }

                if let BattleOpenerType::Wild(offset) = &mut self.type_ {
                    if offset.y > 0.0 {
                        offset.x -= 360.0 * delta;
                        if offset.x < 0.0 {
                            offset.x += BattleOpenerType::GRASS_WIDTH;
                        }
                        if self.offset <= 130.0 {
                            offset.y -= 60.0 * delta;
                        }
                    }
                }
            }
            true => match elements.text.state.as_ref() {
                Some(text) => {

                    if text.waiting() && text.page() >= text.pages() - 2 {
                        self.leaving = true;
                    }
                    if self.leaving && self.offset < Self::FINAL_TRAINER_OFFSET {
                        self.offset += 300.0 * app.timer.delta_f32();
                    }

                    elements.trainer.update(delta);
                    if elements.text.page() > Some(0)
                        && !elements.trainer.ending()
                        && !matches!(local.data.type_, BattleType::Wild)
                    {
                        elements.trainer.end();
                    }
                }
                None => match self.leaving {
                    true => {

                    },
                    false => {
                        if let Some((.., remote)) = remotes.players.get_index(remotes.current) {
                            let sent =
                                Self::concatenate(remote.active_iter().map(|(.., p)| {
                                    p.as_ref().map(|p| p.name()).unwrap_or("Unknown")
                                }));
                            match &self.type_ {
                                BattleOpenerType::Wild(_) => {
                                    let text = elements
                                        .text
                                        .state
                                        .get_or_insert_with(MessageState::default);
                                    text.pages.push(MessagePage {
                                        lines: vec![format!("Wild {} appeared!", sent,)],
                                        ..Default::default()
                                    });
                                }
                                BattleOpenerType::Trainer(_) => {
                                    let text = elements
                                        .text
                                        .state
                                        .get_or_insert_with(MessageState::default);
                                    text.pages.push(MessagePage {
                                        lines: vec![
                                            remote.name().to_owned(),
                                            "would like to battle!".to_owned(),
                                        ],
                                        wait: None,
                                        ..Default::default()
                                    });
                                    text.pages.push(MessagePage {
                                        lines: vec![
                                            format!("{} sent", remote.name()),
                                            format!("out {}", sent,),
                                        ],
                                        wait: Some(0.5),
                                        ..Default::default()
                                    });
                                }
                                BattleOpenerType::None => unreachable!(),
                            }
                        }
                    }
                },
            },
        }

        self.leaving && !elements.text.alive()
    }

    pub fn draw<ID, P: Deref<Target = Pokemon> + Clone, M: Deref<Target = Move> + Clone>(
        &self,
        draw: &mut Draw,
        elements: &BattleGui<ID, D, M>,
        remotes: &GuiRemotePlayers<ID, P>,
    ) {
        elements.background.draw(draw, self.offset());
        match &self.type_ {
            BattleOpenerType::Wild(..) => {
                elements.pokemon.draw_remotes(
                    draw,
                    remotes,
                    Vec2::new(-self.offset, 0.0),
                    Self::LIGHTGRAY,
                );
            }
            BattleOpenerType::Trainer(trainer) => {
                if let Some(texture) = trainer {
                    draw.image(texture)
                        .position(144.0 - self.offset, 74.0 - texture.height());
                }
            }
            BattleOpenerType::None => (),
        }

        draw.texture(
            &self.player,
            41.0 + self.offset,
            49.0,
            DrawParams {
                source: Some(Rect {
                    x: 0.0,
                    y: if self.offset <= -Self::PLAYER_T3 {
                        // 78.0
                        256.0
                    } else if self.offset <= -Self::PLAYER_T2 {
                        // 60.0
                        192.0
                    } else if self.offset <= -Self::PLAYER_T1 {
                        // 42.0
                        128.0
                    } else if self.offset < 0.0 {
                        64.0
                    } else {
                        0.0
                    },
                    width: self.player.height(),
                    height: self.player.height(),
                }),
                ..Default::default()
            },
        );

        if let BattleOpenerType::Wild(offset) = &self.type_ {
            if offset.y > 0.0 {
                let y = 114.0 - offset.y;
                draw.image(&self.grass)
                    .position(offset.x - BattleOpenerType::GRASS_WIDTH, y);
                draw.image(&self.grass).position(offset.x, y);
                draw.image(&self.grass)
                    .position(offset.x + BattleOpenerType::GRASS_WIDTH, y);
            }
        }

        elements.trainer.draw(draw);
        elements.draw_panel(draw);

        draw.rect((0.0, 0.0), (draw.width(), self.rect_size))
            .color(Color::BLACK);
        draw.rect(
            (0.0, 160.0 - self.rect_size),
            (draw.width(), self.rect_size),
        )
        .color(Color::BLACK);
    }

    pub fn offset(&self) -> f32 {
        self.offset
    }

    pub fn reset(&mut self) {
        self.offset = Self::OFFSET;
        self.rect_size = Self::RECT_SIZE;
        self.shrink_by = Self::SHRINK_BY_DEF;
        self.wait = Self::WAIT;
        self.type_ = BattleOpenerType::None;
        self.leaving = false;
    }

    pub fn alive(&self) -> bool {
        !matches!(self.type_, BattleOpenerType::None)
    }

    /// To - do: fix this function
    pub(crate) fn concatenate<'a>(mut names: impl DoubleEndedIterator<Item = &'a str> + 'a) -> String {
        let last = names.next_back();
        let first = names.next();
        match first {
            Some(first) => {
                let mut string = String::from(first);
                while let Some(name) = names.next() {
                    string.push_str(", ");
                    string.push_str(name);
                }
                string.push_str(" and ");
                string.push_str(last.unwrap());
                string
            }
            None => match last {
                Some(last) => last.to_string(),
                None => String::new(),
            },
        }
    }
}
