use game::{
    graphics::{byte_texture, draw_text_left, draw_text_right, position},
    gui::health::HealthBar,
    pokedex::{
        battle::view::UnknownPokemon,
        pokemon::{instance::PokemonInstance, stat::StatSet, Level, Health},
    },
    tetra::{graphics::Texture, math::Vec2, Context},
    text::TextColor,
    util::Entity,
};

use crate::{
    ui::{exp_bar::ExperienceBar, BattleGuiPosition, BattleGuiPositionIndex},
    view::PokemonView,
};

static mut PLAYER: Option<Texture> = None;

fn large_ui(ctx: &mut Context) -> &'static Texture {
    unsafe {
        PLAYER.get_or_insert(byte_texture(
            ctx,
            include_bytes!("../../../assets/gui/large.png"),
        ))
    }
}

static mut OPPONENT_PADDING: Option<Texture> = None;

fn padding(ctx: &mut Context) -> &'static Texture {
    unsafe {
        OPPONENT_PADDING.get_or_insert(byte_texture(
            ctx,
            include_bytes!("../../../assets/gui/padding.png"),
        ))
    }
}

static mut OPPONENT: Option<Texture> = None;

fn small_ui(ctx: &mut Context) -> &'static Texture {
    unsafe {
        OPPONENT.get_or_insert(byte_texture(
            ctx,
            include_bytes!("../../../assets/gui/small.png"),
        ))
    }
}

pub struct PokemonStatusGui {
    alive: bool,

    position: BattleGuiPosition,

    origin: Vec2<f32>,

    background: (Option<Texture>, Texture),
    small: bool,
    data_pos: PokemonStatusPos,
    health: (HealthBar, Vec2<f32>),
    data: PokemonStatusData,
    exp: ExperienceBar,
}

#[derive(Default)]
struct PokemonStatusData {
    active: bool,
    name: String,
    level: (String, Level),
    health: String,
}

struct PokemonStatusPos {
    name: f32,
    level: f32,
}

impl PokemonStatusGui {
    pub const BATTLE_OFFSET: f32 = 24.0 * 5.0;

    const HEALTH_Y: f32 = 15.0;

    pub fn new(ctx: &mut Context, index: BattleGuiPositionIndex) -> Self {
        let (((background, origin, small), data_pos, hb), position) = Self::attributes(ctx, index);

        Self {
            alive: false,

            position,

            origin,

            small,
            background,
            data_pos,
            health: (HealthBar::new(ctx), hb),
            exp: ExperienceBar::new(),
            data: Default::default(),
        }
    }

    pub fn with_known(
        ctx: &mut Context,
        index: BattleGuiPositionIndex,
        pokemon: Option<&PokemonInstance>,
    ) -> Self {
        let (((background, origin, small), data_pos, hb), position) = Self::attributes(ctx, index);
        Self {
            alive: false,
            position,
            origin,
            small,
            background,
            data: pokemon.map(|pokemon| PokemonStatusData {
                active: true,
                name: pokemon.name().to_owned(),
                level: Self::level(pokemon.level()),
                health: format!("{}/{}", pokemon.hp(), pokemon.max_hp()),
            }).unwrap_or_default(),
            data_pos,
            health: (
                HealthBar::with_size(
                    ctx,
                    pokemon
                        .map(|pokemon| HealthBar::width(pokemon.hp(), pokemon.max_hp()))
                        .unwrap_or_default(),
                ),
                hb,
            ),
            exp: ExperienceBar::new(),
        }
    }

    pub fn with_unknown(
        ctx: &mut Context,
        index: BattleGuiPositionIndex,
        pokemon: Option<&UnknownPokemon>,
    ) -> Self {
        let (((background, origin, small), data_pos, hb), position) = Self::attributes(ctx, index);
        Self {
            alive: false,
            position,
            origin,
            background,
            small,
            data: pokemon.map(|pokemon| PokemonStatusData {
                active: true,
                name: pokemon.name().to_owned(),
                level: Self::level(pokemon.level()),
                health: String::new(),
            }).unwrap_or_default(),
            data_pos,
            health: (
                HealthBar::with_size(
                    ctx,
                    pokemon.map(|pokemon| pokemon.hp()).unwrap_or_default() * HealthBar::WIDTH,
                ),
                hb,
            ),
            exp: ExperienceBar::new(),
        }
    }

    const TOP_SINGLE: Vec2<f32> = Vec2::new(14.0, 18.0);

    const BOTTOM_SINGLE: Vec2<f32> = Vec2::new(127.0, 75.0);
    const BOTTOM_MANY_WITH_BOTTOM_RIGHT: Vec2<f32> = Vec2::new(240.0, 113.0);

    // const OPPONENT_HEIGHT: f32 = 29.0;
    const OPPONENT_HEALTH_OFFSET: Vec2<f32> = Vec2::new(24.0, Self::HEALTH_Y);

    const OPPONENT_POSES: PokemonStatusPos = PokemonStatusPos {
        name: 8.0,
        level: 86.0,
    };

    const EXP_OFFSET: Vec2<f32> = Vec2::new(32.0, 33.0);

    fn attributes(
        ctx: &mut Context,
        index: BattleGuiPositionIndex,
    ) -> (
        (
            ((Option<Texture>, Texture), Vec2<f32>, bool),
            PokemonStatusPos,
            Vec2<f32>,
        ),
        BattleGuiPosition,
    ) {
        (
            match index.position {
                BattleGuiPosition::Top => {
                    if index.size == 1 {
                        (
                            (
                                (Some(padding(ctx).clone()), small_ui(ctx).clone()), // Background
                                Self::TOP_SINGLE,
                                true,
                            ),
                            Self::OPPONENT_POSES,         // Text positions
                            Self::OPPONENT_HEALTH_OFFSET, // Health Bar Pos
                        )
                    } else {
                        let texture = small_ui(ctx).clone();
                        let mut pos = Vec2::zero();
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
                                (None, large_ui(ctx).clone()),
                                Self::BOTTOM_SINGLE,
                                false,
                                // Some(ExperienceBar::new(/*Self::BOTTOM_SINGLE + Self::EXP_OFFSET*/),),
                            ),
                            PokemonStatusPos {
                                name: 17.0,
                                level: 95.0,
                            },
                            Vec2::new(33.0, Self::HEALTH_Y),
                        )
                    } else {
                        let texture = small_ui(ctx).clone();
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

    pub fn update_exp(&mut self, delta: f32, pokemon: &PokemonInstance) {
        if self.data.active {
            if self.small {
                self.exp.update_exp(pokemon.level, pokemon, true)
            } else {
                if self.exp.update(delta) {
                    self.data.level.1 += 1;
                    self.data.level.0 = Self::level_fmt(self.data.level.1);
                    let base = StatSet::hp(
                        pokemon.pokemon().base.hp,
                        pokemon.ivs.hp,
                        pokemon.evs.hp,
                        self.data.level.1,
                    );
                    self.data.update_health(pokemon.hp(), base);
                }
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

    pub fn update_gui(&mut self, pokemon: Option<&dyn PokemonView>, reset: bool) {
        self.update_gui_ex(
            if let Some(pokemon) = pokemon {
                Some((pokemon.level(), pokemon))
            } else {
                None
            },
            reset,
        );
    }

    pub fn update_gui_ex(&mut self, pokemon: Option<(Level, &dyn PokemonView)>, reset: bool) {
        self.data.active = if let Some((previous, pokemon)) = pokemon {
            self.data.update(previous, pokemon, reset, &mut self.health.0, &mut self.exp, !self.small);
            true
        } else {
            false
        };
    }

    pub fn draw(&self, ctx: &mut Context, offset: f32, bounce: f32) {
        if self.alive {
            if self.data.active {
                let should_bounce =
                    !self.data.health.is_empty() || matches!(self.position, BattleGuiPosition::Top);
                let pos = Vec2::new(
                    self.origin.x + offset + if should_bounce { 0.0 } else { bounce },
                    self.origin.y + if should_bounce { bounce } else { 0.0 },
                );

                if let Some(padding) = &self.background.0 {
                    padding.draw(ctx, position(pos.x + 8.0, pos.y + 21.0));
                }
                self.background.1.draw(ctx, position(pos.x, pos.y));

                let x2 = pos.x + self.data_pos.level;
                let y = pos.y + 2.0;

                draw_text_left(
                    ctx,
                    &0,
                    &self.data.name,
                    &TextColor::Black,
                    pos.x + self.data_pos.name,
                    y,
                );

                draw_text_right(ctx, &0, &self.data.level.0, &TextColor::Black, x2, y);

                if !self.small {
                    self.exp.draw(ctx, pos + Self::EXP_OFFSET);
                    draw_text_right(ctx, &0, &self.data.health, &TextColor::Black, x2, y + 18.0);
                }

                self.health.0.draw(ctx, pos + self.health.1);
            }
        }
    }
}

impl PokemonStatusData {

    pub fn update(
        &mut self,
        previous: Level,
        pokemon: &dyn PokemonView,
        reset: bool,
        health: &mut HealthBar,
        exp: &mut ExperienceBar,
        exp_active: bool,
    ) {
        if &self.name != pokemon.name() {
            self.name = pokemon.name().to_owned();
        }
        if pokemon.level() == previous {
            health.resize(pokemon.hp(), reset);
        }
        if exp_active {
            if let Some(pokemon) = pokemon.instance() {
                exp.update_exp(previous, pokemon, reset);
                if pokemon.level == previous {
                }
            }
        }
        if reset {
            self.level = PokemonStatusGui::level(pokemon.level());
        }
    }

    fn update_health(&mut self, current: Health, max: Health) {
        self.health.clear();
        use std::fmt::Write;
        if let Err(err) = write!(self.health, "{}/{}", current, max) {
            game::log::warn!("Could not write to health text with error {}", err);
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
