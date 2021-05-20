use game::{
	util::Entity,
	pokedex::pokemon::{
		Level,
		instance::PokemonInstance,
	},
	macroquad::prelude::{Vec2, const_vec2, Texture2D},
	gui::health::HealthBar,
	text::TextColor,
	graphics::{byte_texture, draw, draw_text_left, draw_text_right},
};

use crate::ui::{
	BattleGuiPosition,
	BattleGuiPositionIndex,
	exp_bar::ExperienceBar,
};

static mut PLAYER: Option<Texture2D> = None;

fn player_texture() -> Texture2D {
	unsafe { *PLAYER.get_or_insert(byte_texture(include_bytes!("../../../assets/gui/player.png"))) }
}

static mut OPPONENT_PADDING: Option<Texture2D> = None;

fn opponent_padding() -> Texture2D {
	unsafe { *OPPONENT_PADDING.get_or_insert(byte_texture(include_bytes!("../../../assets/gui/opponent_padding.png"))) }
}

static mut OPPONENT: Option<Texture2D> = None;

fn opponent_texture() -> Texture2D {
	unsafe { *OPPONENT.get_or_insert(byte_texture(include_bytes!("../../../assets/gui/opponent.png"))) }
}

pub struct PokemonStatusGui {

	alive: bool,

	origin: Vec2,

	background: (Option<Texture2D>, Texture2D),
	name: Option<String>,
	level: Option<(String, Level)>,
	data_pos: PokemonStatusPos,
	health: (HealthBar, Vec2),
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

	pub fn new(index: BattleGuiPositionIndex) -> Self {

		let ((background, origin, exp), data_pos, hb) = Self::attributes(index);

		Self {

			alive: false,

			origin,

			background,
			name: None,
			level: None,
			data_pos,
			health: (HealthBar::new(), hb),
			health_text: None,
			exp,

		}

	}

	pub fn with(index: BattleGuiPositionIndex, pokemon: &PokemonInstance) -> Self {

		let ((background, origin, exp), data_pos, hb) = Self::attributes(index);
		Self {
			alive: false,
			origin,
			background,
			name: Some(pokemon.name().to_string()),
			level: Some(Self::level(pokemon.data.level)),
			data_pos,
			health: (HealthBar::with_size(HealthBar::width(pokemon.current_hp, pokemon.base.hp)), hb),
			health_text: exp.is_some().then(|| format!("{}/{}", pokemon.current_hp, pokemon.base.hp)),
			exp: exp.map(|mut exp| {
				exp.update_exp(pokemon.data.level, pokemon, true);
				exp
			}),			
		}
	}

	const TOP_SINGLE: Vec2 = const_vec2!([14.0, 18.0]);

	const BOTTOM_SINGLE: Vec2 = const_vec2!([127.0, 75.0]);
	const BOTTOM_MANY_WITH_BOTTOM_RIGHT: Vec2 = const_vec2!([240.0, 113.0]);

	// const OPPONENT_HEIGHT: f32 = 29.0;
	const OPPONENT_HEALTH_OFFSET: Vec2 = const_vec2!([24.0, Self::HEALTH_Y]);

	const OPPONENT_POSES: PokemonStatusPos = PokemonStatusPos {
		name: 8.0,
		level: 86.0,
	};

	const EXP_OFFSET: Vec2 = const_vec2!([32.0, 33.0]);


	fn attributes(index: BattleGuiPositionIndex) -> (((Option<Texture2D>, Texture2D), Vec2, Option<ExperienceBar>), PokemonStatusPos, Vec2) {
		match index.position {
			BattleGuiPosition::Top => {
				if index.size == 1 {
					(
						(
							(Some(opponent_padding()), opponent_texture()), // Background
							Self::TOP_SINGLE, // Panel
							None
						),
						Self::OPPONENT_POSES, // Text positions
						Self::OPPONENT_HEALTH_OFFSET, // Health Bar Pos
					)
				} else {
					let texture = opponent_texture();
					let mut pos = Vec2::ZERO;
					pos.y += index.index as f32 * texture.height();
					(
						(
							(None, texture), // Background
							pos, // Panel
							None
						),
						Self::OPPONENT_POSES,
						Self::OPPONENT_HEALTH_OFFSET, // Health Bar Pos
					)
				}				
			},
			BattleGuiPosition::Bottom => {
				if index.size == 1 {
					(
						(
							(None, player_texture()),
							Self::BOTTOM_SINGLE,
							Some(ExperienceBar::new(/*Self::BOTTOM_SINGLE + Self::EXP_OFFSET*/)),
						),
						PokemonStatusPos {
							name: 17.0,
							level: 95.0,
						},
						const_vec2!([33.0, Self::HEALTH_Y])
					)
				} else {
					let texture = opponent_texture();
					let mut pos = Self::BOTTOM_MANY_WITH_BOTTOM_RIGHT;
					pos.x -= texture.width();
					pos.y -= (index.index + 1) as f32 * (texture.height() + 1.0);
					(
						(
							(None, texture),
							pos,
							None
						),
						Self::OPPONENT_POSES,
						Self::OPPONENT_HEALTH_OFFSET
					)
				}
			}
		}
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
					let base = game::pokedex::pokemon::stat::calculate_hp(pokemon.pokemon.value().base.hp, pokemon.data.ivs.hp, pokemon.data.evs.hp, level.1);
					self.health_text = Some(format!("{}/{}", pokemon.current_hp, base));
					self.health.0.resize(pokemon.current_hp, base, false);
				}
			}
			self.health.0.update(delta);
		}
	}

	pub fn health_moving(&self) -> bool {
		self.health.0.is_moving()
	}

	pub fn exp_moving(&self) -> bool {
		self.exp.as_ref().map(|exp| exp.moving()).unwrap_or_default() || self.health.0.is_moving()
	}

	pub fn update_gui(&mut self, pokemon: Option<(Level, &PokemonInstance)>, reset: bool) {
		self.name = pokemon.map(|(previous, pokemon)| {
			if pokemon.data.level == previous {
				self.health.0.resize(pokemon.current_hp, pokemon.base.hp, reset);
			} 
			if let Some(exp) = self.exp.as_mut() {
				exp.update_exp(previous, pokemon, reset);
				if pokemon.data.level == previous {
					self.health_text = Some(format!("{}/{}", pokemon.current_hp, pokemon.base.hp));
				}
			}
			if reset {
				self.level = Some(Self::level(pokemon.data.level));
			}
			pokemon.name().to_string()
		});
	}

	pub fn render(&self, offset: f32, bounce: f32) {
		if self.alive {
			if let Some(name) = &self.name {
				let pos = Vec2::new(
					self.origin.x + offset + if self.health_text.is_some() {
						0.0
					} else {
						bounce
					},
					self.origin.y + if self.health_text.is_some() {
						bounce
					} else {
						0.0
					}
				);

				if let Some(padding) = self.background.0 {
					draw(padding, pos.x + 8.0, pos.y + 21.0);
				}
				draw(self.background.1, pos.x, pos.y);

				let x2 = pos.x + self.data_pos.level;
				let y = pos.y + 2.0;

				if let Some(health_text) = self.health_text.as_ref() {
					draw_text_right(0, health_text, TextColor::Black, x2, y + 18.0);
				}

				draw_text_left(0, name, TextColor::Black, pos.x + self.data_pos.name, y);
				if let Some((level, _)) = &self.level {
					draw_text_right(0, level, TextColor::Black, x2, y);
				}

				if let Some(exp) = self.exp.as_ref() {
					exp.render(pos + Self::EXP_OFFSET);
				}
				
				self.health.0.render(pos + self.health.1);
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

    fn is_alive(&self) -> bool {
        self.alive
    }
}