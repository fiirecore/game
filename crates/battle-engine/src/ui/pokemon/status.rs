use pokengine::{
    engine::{egui, graphics::Texture, math::Vec2},
    gui::health::HealthBar,
    pokedex::pokemon::{owned::OwnedPokemon, Level},
};

use battle::pokemon::remote::InitUnknownPokemon;

use crate::{
    ui::{exp_bar::ExperienceBar, BattleGuiPosition, BattleGuiPositionIndex},
    view::GuiPokemonView,
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

impl PokemonStatusGui {
    pub const BATTLE_OFFSET: f32 = 24.0 * 5.0;

    const HEALTH_Y: f32 = 15.0;

    // pub fn new<'d>(
    //     ctx: &BattleGuiData,
    //     dex: &PokedexClientData,
    //     index: BattleGuiPositionIndex,
    // ) -> Self {
    //     let (((background, origin, small), data_pos, hb), position) = Self::attributes(ctx, index);

    //     Self {
    //         alive: false,

    //         position,

    //         origin,

    //         small,
    //         background: Some(background),
    //         data_pos,
    //         health: (HealthBar::default(), hb),
    //         exp: ExperienceBar::new(),
    //         data: Default::default(),
    //     }
    // }

    // pub fn with_known(
    //     ctx: &BattleGuiData,
    //     dex: &PokedexClientData,
    //     index: BattleGuiPositionIndex,
    //     pokemon: Option<&OwnedPokemon>,
    // ) -> Self {
    //     let (((background, origin, small), data_pos, hb), position) = Self::attributes(ctx, index);
    //     Self {
    //         alive: false,
    //         position,
    //         origin,
    //         small,
    //         background: Some(background),
    //         data: pokemon
    //             .map(|pokemon| PokemonStatusData {
    //                 active: true,
    //                 name: pokemon.name().to_owned(),
    //                 level: Self::level(pokemon.level),
    //                 health: format!("{}/{}", pokemon.hp(), pokemon.max_hp()),
    //             })
    //             .unwrap_or_default(),
    //         data_pos,
    //         health: (
    //             HealthBar::default(),
    //             // HealthBar::with_size(
    //             //     dex,
    //             //     pokemon
    //             //         .map(|pokemon| HealthBar::width(pokemon.percent_hp()))
    //             //         .unwrap_or_default(),
    //             // ),
    //             hb,
    //         ),
    //         exp: ExperienceBar::new(),
    //     }
    // }

    // pub fn with_unknown(
    //     ctx: &BattleGuiData,
    //     dex: &PokedexClientData,
    //     index: BattleGuiPositionIndex,
    //     pokemon: Option<&InitUnknownPokemon>,
    // ) -> Self {
    //     let (((background, origin, small), data_pos, hb), position) = Self::attributes(ctx, index);
    //     Self {
    //         alive: false,
    //         position,
    //         origin,
    //         background: Some(background),
    //         small,
    //         data: pokemon
    //             .map(|pokemon| PokemonStatusData {
    //                 active: true,
    //                 name: pokemon.name().to_owned(),
    //                 level: Self::level(pokemon.level),
    //                 health: String::new(),
    //             })
    //             .unwrap_or_default(),
    //         data_pos,
    //         health: (HealthBar::default(), hb),
    //         exp: ExperienceBar::new(),
    //     }
    // }

    // const TOP_SINGLE: Vec2 = Vec2::new(14.0, 18.0);

    // const BOTTOM_SINGLE: Vec2 = Vec2::new(127.0, 75.0);
    // const BOTTOM_MANY_WITH_BOTTOM_RIGHT: Vec2 = Vec2::new(240.0, 113.0);

    // // const OPPONENT_HEIGHT: f32 = 29.0;
    // const OPPONENT_HEALTH_OFFSET: Vec2 = Vec2::new(24.0, Self::HEALTH_Y);

    // const OPPONENT_POSES: PokemonStatusPos = PokemonStatusPos {
    //     name: 8.0,
    //     level: 86.0,
    // };

    // const EXP_OFFSET: Vec2 = Vec2::new(32.0, 33.0);

    // fn attributes(
    //     ctx: &BattleGuiData,
    //     index: BattleGuiPositionIndex,
    // ) -> (
    //     (
    //         ((Option<Texture>, Texture), Vec2, bool),
    //         PokemonStatusPos,
    //         Vec2,
    //     ),
    //     BattleGuiPosition,
    // ) {
    //     (
    //         match index.position {
    //             BattleGuiPosition::Top => {
    //                 if index.size == 1 {
    //                     (
    //                         (
    //                             (Some(ctx.padding.clone()), ctx.smallui.clone()), // Background
    //                             Self::TOP_SINGLE,
    //                             true,
    //                         ),
    //                         Self::OPPONENT_POSES,         // Text positions
    //                         Self::OPPONENT_HEALTH_OFFSET, // Health Bar Pos
    //                     )
    //                 } else {
    //                     let texture = ctx.smallui.clone();
    //                     let mut pos = Vec2::ZERO;
    //                     pos.y += index.index as f32 * texture.height() as f32;
    //                     (
    //                         (
    //                             (None, texture), // Background
    //                             pos,             // Panel
    //                             true,
    //                         ),
    //                         Self::OPPONENT_POSES,
    //                         Self::OPPONENT_HEALTH_OFFSET, // Health Bar Pos
    //                     )
    //                 }
    //             }
    //             BattleGuiPosition::Bottom => {
    //                 if index.size == 1 {
    //                     (
    //                         (
    //                             (None, ctx.largeui.clone()),
    //                             Self::BOTTOM_SINGLE,
    //                             false,
    //                             // Some(ExperienceBar::new(/*Self::BOTTOM_SINGLE + Self::EXP_OFFSET*/),),
    //                         ),
    //                         PokemonStatusPos {
    //                             name: 17.0,
    //                             level: 95.0,
    //                         },
    //                         Vec2::new(33.0, Self::HEALTH_Y),
    //                     )
    //                 } else {
    //                     let texture = ctx.smallui.clone();
    //                     let mut pos = Self::BOTTOM_MANY_WITH_BOTTOM_RIGHT;
    //                     pos.x -= texture.width() as f32;
    //                     pos.y -= (index.index + 1) as f32 * (texture.height() as f32 + 1.0);
    //                     (
    //                         ((None, texture), pos, true),
    //                         Self::OPPONENT_POSES,
    //                         Self::OPPONENT_HEALTH_OFFSET,
    //                     )
    //                 }
    //             }
    //         },
    //         index.position,
    //     )
    // }

    // fn level(level: Level) -> (String, Level) {
    //     (Self::level_fmt(level), level)
    // }

    // fn level_fmt(level: Level) -> String {
    //     format!("Lv{}", level)
    // }

    // pub fn update_hp(&mut self, delta: f32) {
    //     self.health.0.update(delta);
    // }

    pub fn ui(ui: &mut egui::Ui, hashnum: usize, pokemon: &impl GuiPokemonView) {
        egui::Grid::new(("StatusGrid", hashnum)).show(ui, |ui| {
            ui.label(pokemon.name());
            ui.label(format!("{}", pokemon.level()));
            ui.end_row();
            ui.label(format!("{:.3}% HP", pokemon.percent_hp() * 100.0));
        });
    }
}
