use std::fmt::Display;
use crate::io::data::player::PlayerData;
use crate::pokemon::moves::MoveCategory;
use crate::pokemon::moves::PokemonMove;
use crate::pokemon::party::PokemonParty;
use crate::pokemon::pokedex::texture::Side;
use crate::pokemon::pokedex::texture::pokemon_texture;
use crate::util::graphics::Texture;
use crate::entity::Entity;
use crate::gui::battle::battle_gui::BattleGui;
use crate::gui::battle::battle_text;
use crate::util::graphics::draw_bottom;
use super::battle_info::BattleType;
use super::battle_pokemon::BattlePokemon;
use super::transitions::managers::battle_closer_manager::BattleCloserManager;

pub struct Battle {
	
	pub player_pokemon: Vec<BattlePokemon>,
	pub opponent_pokemon: Vec<BattlePokemon>,

	pub player_active: usize,
	pub opponent_active: usize,

	pub player_move: PokemonMove,
	pub opponent_move: PokemonMove,

	pub player_textures: Vec<Texture>,
	pub opponent_textures: Vec<Texture>,

	//pub battle_events: BattleEventManager,

	pub pmove_queued: bool,
	pub omove_queued: bool,
	pub faint_queued: bool,

	//pub move_finished: bool,
	pub faint: bool,

	try_run: bool,
	battle_type: BattleType,
	
}

impl Default for Battle {
	fn default() -> Self {
		
		Self {
		
			player_pokemon: Vec::new(),
			opponent_pokemon: Vec::new(),

			player_active: 0,
			opponent_active: 0,

			player_move: PokemonMove::default(),
			opponent_move: PokemonMove::default(),

			player_textures: Vec::new(),
			opponent_textures: Vec::new(),

			pmove_queued: false,
			omove_queued: false,
			faint_queued: false,

			//move_finished: false,
			faint: false,
			try_run: false,

			battle_type: BattleType::Wild,
		
		}
		
	}
}

impl Battle {
	
	pub fn new(battle_type: BattleType, player_pokemon: &PokemonParty, opponent_pokemon: &PokemonParty) -> Self {
		
		let mut player_active = 0;

		Self {
			
			player_pokemon: player_pokemon.pokemon.iter().map(|pokemon| {
				if let Some(hp) = pokemon.current_hp {
					if hp == 0 {
						player_active += 1;
					}
				}
				BattlePokemon::new(pokemon)
			}
			).collect(),
			opponent_pokemon: opponent_pokemon.pokemon.iter().map(|pokemon| {
				BattlePokemon::new(pokemon)
			}).collect(),
			
			player_active: player_active,

			battle_type: battle_type,

			..Battle::default()
			
		}
		
	}

	fn load_textures(&mut self) {
		for i in &self.opponent_pokemon {
			self.opponent_textures.push(pokemon_texture(&i.data.name, Side::Front));
		}
		for i in &self.player_pokemon {
			self.player_textures.push(pokemon_texture(&i.data.name, Side::Back));
		}
	}

	pub fn load(&mut self) {
		self.load_textures();
	}

	pub fn update(&mut self, delta: f32, battle_gui: &mut BattleGui, battle_closer_manager: &mut BattleCloserManager) {
		if self.try_run {
			if self.battle_type == BattleType::Wild {
				battle_closer_manager.spawn();
			}
		}
		if self.pmove_queued || self.omove_queued || self.faint_queued {
			if battle_gui.opponent_pokemon_gui.health_bar.get_width() == 0 {
				battle_gui.update_gui(&self);
			}
			if self.player().base.speed > self.opponent().base.speed {
				battle_text::pmove(delta, self, battle_gui);
			} else {
				battle_text::omove(delta, self, battle_gui);
			}
		} else if self.faint {
			if self.player().faint() {
				for pkmn_index in 0..self.player_pokemon.len() {
					if self.player_pokemon[pkmn_index].current_hp != 0 {
						self.faint = false;
						self.player_active = pkmn_index;
						battle_gui.update_gui(&self);
						break;
					}
				}
				if self.faint {
					battle_closer_manager.spawn();
				}
			} else {
				for pkmn_index in 0..self.opponent_pokemon.len() {

					// Calculate and give exp to player

					let gain = ((self.opponent().training.base_exp * self.opponent().level as usize) as f32 * match self.battle_type {
						BattleType::Wild => 1.0,
						_ => 1.5,
					} / 7.0) as usize;
					self.player_mut().exp += gain;
					let max_exp = self.player().training.growth_rate.level_exp(self.player().level);
					if self.player().exp > max_exp {
						let player = self.player_mut();
						player.level += 1;
						player.exp -= max_exp;
						macroquad::prelude::info!("{} levelled up to Lv. {}", &player.data.name, player.level);
					} else {
						macroquad::prelude::info!("{} gained {} exp. {} is needed to level up!", &self.player().data.name, gain, max_exp - self.player().exp);
					}



					if self.opponent_pokemon[pkmn_index].current_hp != 0 {
						self.faint = false;
						self.opponent_active = pkmn_index;
						battle_gui.update_gui(&self);
						break;
					}
				}
				if self.faint {
					battle_closer_manager.spawn();
				}
			}
		} else if !(battle_gui.player_panel.battle_panel.is_alive() || battle_gui.player_panel.fight_panel.is_alive()) {
			//self.finished = false;
			battle_gui.player_panel.start();
		}
	}
	
	pub fn render(&self, offset: f32, ppp_y_o: u8) {
		draw_bottom(self.opponent_textures[self.opponent_active], 144.0 - offset, 74.0);
		draw_bottom(self.player_textures[self.player_active], 40.0 + offset, 113.0 + ppp_y_o as f32);
	}

	pub fn queue_player_move(&mut self, index: usize) {
		self.player_move = self.player_mut().moves[index].use_move();
	}

	pub fn queue_opponent_move(&mut self) {
		let index = macroquad::rand::gen_range(0, self.opponent().moves.len());
		self.opponent_move = self.opponent_mut().moves[index].use_move();
	}

	pub fn queue_faint(&mut self) {
		self.omove_queued = false;
		self.pmove_queued = false;
		self.faint_queued = true;
	}

	pub fn player_move(&mut self) {
		let damage = get_move_damage(&self.player_move, &self.player_pokemon[self.player_active], self.opponent());
		let opponent = &mut self.opponent_pokemon[self.opponent_active];
		if damage >= opponent.current_hp {
			opponent.current_hp = 0;
		} else {
			opponent.current_hp -= damage;
		}
	}

	pub fn opponent_move(&mut self) {
		let damage = get_move_damage(&self.opponent_move, &self.opponent_pokemon[self.opponent_active], &self.player_pokemon[self.player_active]);
		let player = &mut self.player_pokemon[self.player_active];
		if damage >= player.current_hp {
			player.current_hp = 0;
		} else {
			player.current_hp -= damage;
		}
	}

	pub fn update_data(&mut self, player_data: &mut PlayerData) {

		// Heal all pokemon in party (temporary)

		player_data.party.pokemon = self.player_pokemon.iter_mut().map(|pokemon| {
				pokemon.current_hp = pokemon.base.hp;
			pokemon.to_instance()
		}).collect();
		
	}

	pub fn player(&self) -> &BattlePokemon {
		&self.player_pokemon[self.player_active]
	}

	pub fn player_mut(&mut self) -> &mut BattlePokemon {
		&mut self.player_pokemon[self.player_active]
	}

	pub fn opponent(&self) -> &BattlePokemon {
		&self.opponent_pokemon[self.opponent_active]
	}

	pub fn opponent_mut(&mut self) -> &mut BattlePokemon {
		&mut self.opponent_pokemon[self.opponent_active]
	}

	pub fn run(&mut self) {
		self.try_run = true;
	}
	
}

fn get_move_damage(pmove: &PokemonMove, pokemon: &BattlePokemon, recieving_pokemon: &BattlePokemon) -> u16 { // Change to return MoveResult<>
	if if let Some(accuracy) = pmove.accuracy {
		let hit: u8 = macroquad::rand::gen_range(0, 100);
		let test = hit < accuracy;
		macroquad::prelude::debug!("{} accuracy: {} < {} = {}",  pmove, hit, accuracy, if test { "Hit! "} else { "Miss!" });
		test
	} else {
		true
	} {
		if let Some(power) = pmove.power {
			let effective = pmove.pokemon_type.unwrap_or_default().effective(recieving_pokemon.data.primary_type) as f64 * match recieving_pokemon.data.secondary_type {
				Some(ptype) => pmove.pokemon_type.unwrap_or_default().effective(ptype) as f64,
				None => 1.0,
			};
			match pmove.category {
				MoveCategory::Status => return 0,
				MoveCategory::Physical => {
					return ((((2.0 * pokemon.level as f64 / 5.0 + 2.0).floor() * pokemon.base.atk as f64 * power as f64 / recieving_pokemon.base.def as f64).floor() / 50.0).floor() * effective) as u16 + 2;
				},
				MoveCategory::Special => {
					return ((((2.0 * pokemon.level as f64 / 5.0 + 2.0).floor() * pokemon.base.sp_atk as f64 * power as f64 / recieving_pokemon.base.sp_def as f64).floor() / 50.0).floor() * effective) as u16+ 2;
				}
			}
		} else {
			return 0;
		}
	} else {
		macroquad::prelude::info!("{} missed!", pokemon);
		return 0;
	}	
}

impl Display for Battle {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} vs. {}", self.player(), self.opponent())
    }
}