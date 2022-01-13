use core::ops::Deref;

use pokedex::{
    engine::{
        graphics::{draw_text_left, draw_text_right, DrawParams, Texture},
        math::{vec2, Vec2},
        text::TextColor,
        utils::Entity,
        Context, EngineContext,
    },
    gui::health::HealthBar,
    pokemon::{
        owned::{OwnablePokemon, OwnedPokemon},
        Health, Level, Nature, Pokemon,
    },
    PokedexClientData,
};

use battle::pokemon::remote::UnknownPokemon;

use log::warn;

use crate::{
    context::BattleGuiData,
    ui::{exp_bar::ExperienceBar, BattleGuiPosition, BattleGuiPositionIndex},
    view::{BasePokemonView, GuiPokemonView},
};

#[derive(Default, Clone)]
pub struct PokemonStatusGui {
    alive: bool,

    position: BattleGuiPosition,

    origin: Vec2,

    background: Option<(Option<Texture>, Texture)>,
    small: bool,
    data_pos: PokemonStatusPos,
    health: (HealthBar, Vec2),
    data: PokemonStatusData,
    exp: ExperienceBar,
}

#[derive(Default, Clone)]
struct PokemonStatusData {
    active: bool,
    name: String,
    level: (String, Level),
    health: String,
}

#[derive(Default, Clone, Copy)]
struct PokemonStatusPos {
    name: f32,
    level: f32,
}

use pokedex::{item::Item, moves::Move};

impl PokemonStatusGui {
    pub const BATTLE_OFFSET: f32 = 24.0 * 5.0;

    const HEALTH_Y: f32 = 15.0;

    pub fn new<'d>(
        ctx: &BattleGuiData,
        dex: &PokedexClientData,
        index: BattleGuiPositionIndex,
    ) -> Self {
        let (((background, origin, small), data_pos, hb), position) = Self::attributes(ctx, index);

        Self {
            alive: false,

            position,

            origin,

            small,
            background: Some(background),
            data_pos,
            health: (HealthBar::new(dex), hb),
            exp: ExperienceBar::new(),
            data: Default::default(),
        }
    }

    pub fn with_known<P: Deref<Target = Pokemon>, M, I, G>(
        ctx: &BattleGuiData,
        dex: &PokedexClientData,
        index: BattleGuiPositionIndex,
        pokemon: Option<&OwnablePokemon<P, M, I, G, Nature, Health>>,
    ) -> Self {
        let (((background, origin, small), data_pos, hb), position) = Self::attributes(ctx, index);
        Self {
            alive: false,
            position,
            origin,
            small,
            background: Some(background),
            data: pokemon
                .map(|pokemon| PokemonStatusData {
                    active: true,
                    name: pokemon.name().to_owned(),
                    level: Self::level(pokemon.level),
                    health: format!("{}/{}", pokemon.hp(), pokemon.max_hp()),
                })
                .unwrap_or_default(),
            data_pos,
            health: (
                HealthBar::with_size(
                    dex,
                    pokemon
                        .map(|pokemon| HealthBar::width(pokemon.percent_hp()))
                        .unwrap_or_default(),
                ),
                hb,
            ),
            exp: ExperienceBar::new(),
        }
    }

    pub fn with_unknown<P: Deref<Target = Pokemon>>(
        ctx: &BattleGuiData,
        dex: &PokedexClientData,
        index: BattleGuiPositionIndex,
        pokemon: Option<&UnknownPokemon<P>>,
    ) -> Self {
        let (((background, origin, small), data_pos, hb), position) = Self::attributes(ctx, index);
        Self {
            alive: false,
            position,
            origin,
            background: Some(background),
            small,
            data: pokemon
                .map(|pokemon| PokemonStatusData {
                    active: true,
                    name: pokemon.name().to_owned(),
                    level: Self::level(pokemon.level),
                    health: String::new(),
                })
                .unwrap_or_default(),
            data_pos,
            health: (
                HealthBar::with_size(
                    dex,
                    pokemon.map(|pokemon| pokemon.hp).unwrap_or_default() * HealthBar::WIDTH,
                ),
                hb,
            ),
            exp: ExperienceBar::new(),
        }
    }

    const TOP_SINGLE: Vec2 = vec2(14.0, 18.0);

    const BOTTOM_SINGLE: Vec2 = vec2(127.0, 75.0);
    const BOTTOM_MANY_WITH_BOTTOM_RIGHT: Vec2 = vec2(240.0, 113.0);

    // const OPPONENT_HEIGHT: f32 = 29.0;
    const OPPONENT_HEALTH_OFFSET: Vec2 = vec2(24.0, Self::HEALTH_Y);

    const OPPONENT_POSES: PokemonStatusPos = PokemonStatusPos {
        name: 8.0,
        level: 86.0,
    };

    const EXP_OFFSET: Vec2 = vec2(32.0, 33.0);

    fn attributes(
        ctx: &BattleGuiData,
        index: BattleGuiPositionIndex,
    ) -> (
        (
            ((Option<Texture>, Texture), Vec2, bool),
            PokemonStatusPos,
            Vec2,
        ),
        BattleGuiPosition,
    ) {
        (
            match index.position {
                BattleGuiPosition::Top => {
                    if index.size == 1 {
                        (
                            (
                                (Some(ctx.padding.clone()), ctx.smallui.clone()), // Background
                                Self::TOP_SINGLE,
                                true,
                            ),
                            Self::OPPONENT_POSES,         // Text positions
                            Self::OPPONENT_HEALTH_OFFSET, // Health Bar Pos
                        )
                    } else {
                        let texture = ctx.smallui.clone();
                        let mut pos = Vec2::ZERO;
                        pos.y += index.index as f32 * texture.height() as f32;
                        (
                            (
                                (None, texture), // Background
                                pos,             // Panel
                                true,
                            ),
                            Self::OPPONENT_POSES,
                            Self::OPPONENT_HEALTH_OFFSET, // Health Bar Pos
                        )
                    }
                }
                BattleGuiPosition::Bottom => {
                    if index.size == 1 {
                        (
                            (
                                (None, ctx.largeui.clone()),
                                Self::BOTTOM_SINGLE,
                                false,
                                // Some(ExperienceBar::new(/*Self::BOTTOM_SINGLE + Self::EXP_OFFSET*/),),
                            ),
                            PokemonStatusPos {
                                name: 17.0,
                                level: 95.0,
                            },
                            vec2(33.0, Self::HEALTH_Y),
                        )
                    } else {
                        let texture = ctx.smallui.clone();
                        let mut pos = Self::BOTTOM_MANY_WITH_BOTTOM_RIGHT;
                        pos.x -= texture.width() as f32;
                        pos.y -= (index.index + 1) as f32 * (texture.height() as f32 + 1.0);
                        (
                            ((None, texture), pos, true),
                            Self::OPPONENT_POSES,
                            Self::OPPONENT_HEALTH_OFFSET,
                        )
                    }
                }
            },
            index.position,
        )
    }

    fn level(level: Level) -> (String, Level) {
        (Self::level_fmt(level), level)
    }

    fn level_fmt(level: Level) -> String {
        format!("Lv{}", level)
    }

    pub fn update_hp(&mut self, delta: f32) {
        self.health.0.update(delta);
    }

    pub fn update_exp<P: Deref<Target = Pokemon>, M, I, G>(
        &mut self,
        delta: f32,
        pokemon: &OwnablePokemon<P, M, I, G, Nature, Health>,
    ) {
        if self.data.active {
            if self.small {
                self.exp.update_exp(pokemon.level, pokemon, true)
            } else if self.exp.update(delta) {
                self.data.level.1 += 1;
                self.data.level.0 = Self::level_fmt(self.data.level.1);
                let base = Pokemon::base_hp(
                    pokemon.pokemon.base.hp,
                    pokemon.ivs.hp,
                    pokemon.evs.hp,
                    self.data.level.1,
                );
                self.data.update_health(pokemon.hp(), base);
            }
            self.health.0.resize(pokemon.percent_hp(), false);
            self.health.0.update(delta);
        }
    }

    pub fn health_moving(&self) -> bool {
        self.health.0.is_moving()
    }

    pub fn exp_moving(&self) -> bool {
        (self.exp.moving() && !self.small) || self.health.0.is_moving()
    }

    pub fn update_gui<
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        pokemon: Option<&OwnedPokemon<P, M, I>>,
        previous: Option<Level>,
        reset: bool,
    ) {
        self.data.active = if let Some(pokemon) = pokemon {
            self.data.update(
                previous.unwrap_or(pokemon.level),
                pokemon,
                reset,
                &mut self.health.0,
                &mut self.exp,
                !self.small,
            );
            true
        } else {
            false
        };
    }

    pub fn update_gui_view<
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        pokemon: Option<&dyn GuiPokemonView<P, M, I>>,
        previous: Option<Level>,
        reset: bool,
    ) {
        self.data.active = if let Some(pokemon) = pokemon {
            self.data.update_view(
                previous.unwrap_or(pokemon.level()),
                pokemon.base(),
                reset,
                &mut self.health.0,
            );
            true
        } else {
            false
        };
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext, offset: f32, bounce: f32) {
        if self.alive && self.data.active {
            let should_bounce =
                !self.data.health.is_empty() || matches!(self.position, BattleGuiPosition::Top);
            let pos = vec2(
                self.origin.x + offset + if should_bounce { 0.0 } else { bounce },
                self.origin.y + if should_bounce { bounce } else { 0.0 },
            );

            if let Some(background) = self.background.as_ref() {
                if let Some(padding) = &background.0 {
                    padding.draw(ctx, pos.x + 8.0, pos.y + 21.0, Default::default());
                }
                background.1.draw(ctx, pos.x, pos.y, Default::default());
            }

            let x2 = pos.x + self.data_pos.level;
            let y = pos.y + 2.0;

            draw_text_left(
                ctx,
                eng,
                &0,
                &self.data.name,
                pos.x + self.data_pos.name,
                y,
                DrawParams::color(TextColor::BLACK),
            );

            draw_text_right(
                ctx,
                eng,
                &0,
                &self.data.level.0,
                x2,
                y,
                DrawParams::color(TextColor::BLACK),
            );

            if !self.small {
                self.exp.draw(ctx, pos + Self::EXP_OFFSET);
                draw_text_right(
                    ctx,
                    eng,
                    &0,
                    &self.data.health,
                    x2,
                    y + 18.0,
                    DrawParams::color(TextColor::BLACK),
                );
            }

            self.health.0.draw(ctx, pos + self.health.1);
        }
    }
}

impl PokemonStatusData {
    pub fn update_view<P: Deref<Target = Pokemon>>(
        &mut self,
        previous: Level,
        pokemon: &dyn BasePokemonView<P>,
        reset: bool,
        health: &mut HealthBar,
    ) {
        if &self.name != pokemon.name() {
            self.name = pokemon.name().to_owned();
        }
        if pokemon.level() == previous {
            health.resize(pokemon.percent_hp(), reset);
        }
        if reset {
            self.level = PokemonStatusGui::level(pokemon.level());
        }
    }

    pub fn update<P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>>(
        &mut self,
        previous: Level,
        pokemon: &OwnedPokemon<P, M, I>,
        reset: bool,
        health: &mut HealthBar,
        exp: &mut ExperienceBar,
        exp_active: bool,
    ) {
        self.update_view(previous, pokemon, reset, health);
        if exp_active {
            exp.update_exp(previous, pokemon, reset);
        }
    }

    fn update_health(&mut self, current: Health, max: Health) {
        self.health.clear();
        use std::fmt::Write;
        if let Err(err) = write!(self.health, "{}/{}", current, max) {
            warn!("Could not write to health text with error {}", err);
        }
    }
}

impl Entity for PokemonStatusGui {
    fn spawn(&mut self) {
        self.alive = true;
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn alive(&self) -> bool {
        self.alive
    }
}
