use game::{
    graphics::{byte_texture, draw_text_left, draw_text_right, position},
    gui::health::HealthBar,
    pokedex::{
        battle::view::UnknownPokemon,
        pokemon::{instance::PokemonInstance, stat::StatSet, Level},
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
    name: Option<String>,
    level: Option<(String, Level)>,
    data_pos: PokemonStatusPos,
    health: (HealthBar, Vec2<f32>),
    health_text: Option<String>,
    exp: Option<ExperienceBar>,
}

struct PokemonStatusPos {
    name: f32,
    level: f32,
}

impl PokemonStatusGui {
    pub const BATTLE_OFFSET: f32 = 24.0 * 5.0;

    const HEALTH_Y: f32 = 15.0;

    pub fn new(ctx: &mut Context, index: BattleGuiPositionIndex) -> Self {
        let (((background, origin, exp), data_pos, hb), position) = Self::attributes(ctx, index);

        Self {
            alive: false,

            position,

            origin,

            background,
            name: None,
            level: None,
            data_pos,
            health: (HealthBar::new(ctx), hb),
            health_text: None,
            exp,
        }
    }

    pub fn with_known(
        ctx: &mut Context,
        index: BattleGuiPositionIndex,
        pokemon: Option<&PokemonInstance>,
    ) -> Self {
        let (((background, origin, exp), data_pos, hb), position) = Self::attributes(ctx, index);
        Self {
            alive: false,
            position,
            origin,
            background,
            name: pokemon.map(|pokemon| pokemon.name().to_string()),
            level: pokemon.map(|pokemon| Self::level(pokemon.level())),
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
            health_text: exp.is_some().then(|| {
                pokemon
                    .map(|pokemon| format!("{}/{}", pokemon.hp(), pokemon.max_hp()))
                    .unwrap_or_default()
            }),
            exp: exp.map(|mut exp| {
                if let Some(pokemon) = pokemon {
                    exp.update_exp(pokemon.level, pokemon, true);
                }
                exp
            }),
        }
    }

    pub fn with_unknown(
        ctx: &mut Context,
        index: BattleGuiPositionIndex,
        pokemon: Option<&UnknownPokemon>,
    ) -> Self {
        let (((background, origin, _), data_pos, hb), position) = Self::attributes(ctx, index);
        Self {
            alive: false,
            position,
            origin,
            background,
            name: pokemon.map(|pokemon| pokemon.name().to_owned()),
            level: pokemon.as_ref().map(|pokemon| Self::level(pokemon.level())),
            data_pos,
            health: (
                HealthBar::with_size(
                    ctx,
                    pokemon.map(|pokemon| pokemon.hp()).unwrap_or_default() * HealthBar::WIDTH,
                ),
                hb,
            ),
            health_text: None,
            exp: None,
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
            ((Option<Texture>, Texture), Vec2<f32>, Option<ExperienceBar>),
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
                                Self::TOP_SINGLE,                                    // Panel
                                None,
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
                                None,
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
                                Some(
                                    ExperienceBar::new(/*Self::BOTTOM_SINGLE + Self::EXP_OFFSET*/),
                                ),
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
                            ((None, texture), pos, None),
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
        if let Some(exp) = self.exp.as_mut() {
            if exp.update(delta) {
                if let Some(level) = self.level.as_mut() {
                    level.1 += 1;
                    level.0 = Self::level_fmt(level.1);
                    let base = StatSet::hp(
                        pokemon.pokemon().base.hp,
                        pokemon.ivs.hp,
                        pokemon.evs.hp,
                        level.1,
                    );
                    self.health_text = Some(format!("{}/{}", pokemon.hp(), base));
                    self.health.0.resize(pokemon.percent_hp(), false);
                }
            }
            self.health.0.update(delta);
        }
    }

    pub fn health_moving(&self) -> bool {
        self.health.0.is_moving()
    }

    pub fn exp_moving(&self) -> bool {
        self.exp
            .as_ref()
            .map(|exp| exp.moving())
            .unwrap_or_default()
            || self.health.0.is_moving()
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
        self.name = pokemon.map(|(previous, pokemon)| {
            if pokemon.level() == previous {
                self.health.0.resize(pokemon.hp(), reset);
            }
            if let Some(exp) = self.exp.as_mut() {
                if let Some(pokemon) = pokemon.instance() {
                    exp.update_exp(previous, pokemon, reset);
                    if pokemon.level == previous {
                        self.health_text = Some(format!("{}/{}", pokemon.hp(), pokemon.max_hp()));
                    }
                }
            }
            if reset {
                self.level = Some(Self::level(pokemon.level()));
            }
            pokemon.name().to_string()
        });
    }

    pub fn draw(&self, ctx: &mut Context, offset: f32, bounce: f32) {
        if self.alive {
            if let Some(name) = &self.name {
                let should_bounce =
                    self.health_text.is_some() || matches!(self.position, BattleGuiPosition::Top);
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

                if let Some(health_text) = self.health_text.as_ref() {
                    draw_text_right(ctx, &0, health_text, &TextColor::Black, x2, y + 18.0);
                }

                draw_text_left(
                    ctx,
                    &0,
                    name,
                    &TextColor::Black,
                    pos.x + self.data_pos.name,
                    y,
                );
                if let Some((level, _)) = &self.level {
                    draw_text_right(ctx, &0, level, &TextColor::Black, x2, y);
                }

                if let Some(exp) = self.exp.as_ref() {
                    exp.draw(ctx, pos + Self::EXP_OFFSET);
                }

                self.health.0.draw(ctx, pos + self.health.1);
            }
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
