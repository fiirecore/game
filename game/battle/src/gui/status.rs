use game::{
	util::Entity,
	pokedex::pokemon::instance::PokemonInstance,
	macroquad::prelude::{Vec2, const_vec2, Texture2D},
	gui::health::HealthBar,
	text::TextColor,
	graphics::{byte_texture, draw, draw_text_left, draw_text_right},
};

use crate::gui::{BattleGuiPosition, BattleGuiPositionIndex};

use super::exp_bar::ExperienceBar;

static mut PLAYER: Option<Texture2D> = None;

fn player_texture() -> Texture2D {
	unsafe { *PLAYER.get_or_insert(byte_texture(include_bytes!("../../assets/gui/player_pokemon.png"))) }
}

#[deprecated(note = "use texture that combines many and single to take up less space")]
static mut OPPONENT_SINGLE: Option<Texture2D> = None;

fn single_opponent_texture() -> Texture2D {
	unsafe { *OPPONENT_SINGLE.get_or_insert(byte_texture(include_bytes!("../../assets/gui/opponent_single.png"))) }
}

#[deprecated]
static mut OPPONENT_MANY: Option<Texture2D> = None;

fn many_opponent_texture() -> Texture2D {
	unsafe { *OPPONENT_MANY.get_or_insert(byte_texture(include_bytes!("../../assets/gui/opponent_many.png"))) }
}


pub struct PokemonStatusGui {

	alive: bool,

	origin: Vec2,

	background: Texture2D,
	data: Option<PokemonStatusData>,
	data_pos: PokemonStatusPos,
	health: (HealthBar, Vec2),
	health_text: Option<String>,
	exp: Option<ExperienceBar>,

}

struct PokemonStatusPos {
	name: f32,
	level: f32,
}

struct PokemonStatusData {
	name: String,
	level: String,
}


impl From<&PokemonInstance> for PokemonStatusData {
    fn from(pokemon: &PokemonInstance) -> Self {
        Self {
			name: pokemon.name(),
			level: format!("Lv{}", pokemon.data.level),
		}
    }
}

impl PokemonStatusGui {

	pub const BATTLE_OFFSET: f32 = 24.0 * 5.0;

	const HEALTH_Y: f32 = 17.0;

	pub fn new(index: BattleGuiPositionIndex) -> Self {

		let ((background, origin, exp), data_pos, hb) = Self::attributes(index);

		Self {

			alive: false,

			origin,

			background,
			data: None,
			data_pos,
			health: (HealthBar::new(hb), hb),
			health_text: None,
			exp,

		}

	}

	pub fn with(index: BattleGuiPositionIndex, pokemon: &PokemonInstance) -> Self {

		let ((background, origin, exp), data_pos, hb) = Self::attributes(index);

		let mut health = HealthBar::new(origin + hb);
		health.update_bar(pokemon.current_hp, pokemon.base.hp, true);


		Self {
			alive: false,
			origin,
			background,
			data: Some(PokemonStatusData::from(pokemon)),
			data_pos,
			health: (health, hb),
			health_text: if exp.is_some() { Some(format!("{}/{}", pokemon.current_hp, pokemon.base.hp)) } else { None },
			exp,			
		}
	}

	const TOP_SINGLE: Vec2 = const_vec2!([14.0, 18.0]);

	const BOTTOM_SINGLE: Vec2 = const_vec2!([127.0, 75.0]);
	const BOTTOM_MANY_WITH_BOTTOM_RIGHT: Vec2 = const_vec2!([240.0, 113.0]);

	// const OPPONENT_HEIGHT: f32 = 29.0;
	const OPPONENT_HEALTH_OFFSET: Vec2 = const_vec2!([39.0, Self::HEALTH_Y]);

	const OPPONENT_POSES: PokemonStatusPos = PokemonStatusPos {
		name: 8.0,
		level: 86.0,
	};

	const EXP_OFFSET: Vec2 = const_vec2!([32.0, 33.0]);


	fn attributes(index: BattleGuiPositionIndex) -> ((Texture2D, Vec2, Option<ExperienceBar>), PokemonStatusPos, Vec2) {
		match index.position {
			BattleGuiPosition::Top => {
				if index.size == 1 {
					(
						(
							single_opponent_texture(), // Background
							Self::TOP_SINGLE, // Panel
							None
						),
						Self::OPPONENT_POSES, // Text positions
						Self::OPPONENT_HEALTH_OFFSET, // Health Bar Pos
					)
				} else {
					let texture = many_opponent_texture();
					let mut pos = Vec2::default();
					pos.y += index.index as f32 * texture.height();
					(
						(
							texture, // Background
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
							player_texture(),
							Self::BOTTOM_SINGLE,
							Some(ExperienceBar::new(Self::BOTTOM_SINGLE + Self::EXP_OFFSET)),
						),
						PokemonStatusPos {
							name: 17.0,
							level: 95.0,
						},
						const_vec2!([48.0, Self::HEALTH_Y])
					)
				} else {
					let texture = many_opponent_texture();
					let mut pos = Self::BOTTOM_MANY_WITH_BOTTOM_RIGHT;
					pos.x -= texture.width();
					pos.y -= (index.index + 1) as f32 * (texture.height() + 1.0);
					(
						(
							texture,
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

	pub fn update(&mut self, delta: f32) {
		if self.alive {
			self.health.0.update(delta);
		}		
	}

	#[deprecated]
	pub fn render(&self) {
		if self.alive {
			if let Some(data) = &self.data {
				draw(self.background, self.origin.x, self.origin.y);

				let x2 = self.origin.x + self.data_pos.level;
				let y = self.origin.y + 2.0;
				
				if let Some(health_text) = self.health_text.as_ref() {
					draw_text_right(0, health_text, TextColor::Black, x2, y + 18.0);
				}

				draw_text_left(0, &data.name, TextColor::Black, self.origin.x + self.data_pos.name, y);
				draw_text_right(0, &data.level, TextColor::Black, x2, y);

				if let Some(exp) = self.exp.as_ref() {
					exp.render();
				}

				self.health.0.render();
			}
		}		
	}

	pub fn update_gui(&mut self, pokemon: Option<&PokemonInstance>, reset: bool) -> bool {
		let mut damage = false;
		self.data = pokemon.map(|pokemon| {
			damage = self.health.0.update_bar(pokemon.current_hp, pokemon.base.hp, reset);
			if let Some(exp) = self.exp.as_mut() {
				exp.update_exp(pokemon, reset);
				self.health_text = Some(format!("{}/{}", pokemon.current_hp, pokemon.base.hp));
			}
			PokemonStatusData::from(pokemon)
		});
		damage
	}

	// Deprecate maybe?
	pub fn render_offset(&self, offset: f32, bounce: f32) {
		if self.alive {
			if let Some(data) = &self.data {
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

				draw(self.background, pos.x, pos.y);

				let x2 = pos.x + self.data_pos.level;
				let y = pos.y + 2.0;

				if let Some(health_text) = self.health_text.as_ref() {
					draw_text_right(0, health_text, TextColor::Black, x2, y + 18.0);
				}

				draw_text_left(0, &data.name, TextColor::Black, pos.x + self.data_pos.name, y);
				draw_text_right(0, &data.level, TextColor::Black, x2, y);

				if let Some(exp) = self.exp.as_ref() {
					exp.render_position(pos + Self::EXP_OFFSET);
				}
				
				self.health.0.render_position(pos + self.health.1);
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