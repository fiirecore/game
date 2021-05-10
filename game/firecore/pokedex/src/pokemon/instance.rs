use deps::vec::ArrayVec;

use crate::{
	pokemon::{
		PokemonId,
		Level,
		Pokemon,
		PokemonRef,
		types::PokemonType,
		saved::{
			SavedPokemon,
			PokemonData
		},
		data::StatSet,
	},
	moves::{
		MoveRef,
		saved::to_instance,
		instance::{MoveInstanceSet, MoveInstance},
		persistent::PersistentMoveInstance,
	},
	item::{
		Item,
		ItemRef,
		itemdex,
		script::{ItemCondition, ItemActionKind},
	}
};

use super::Health;
use super::types::effective::Effective;

pub type PokemonInstanceParty = ArrayVec<[PokemonInstance; 6]>;

#[derive(Clone)]
pub struct PokemonInstance {
	
	pub pokemon: PokemonRef, 
	
	pub data: PokemonData,

	pub persistent: Option<PersistentMoveInstance>,

	pub item: Option<ItemRef>,
	pub moves: MoveInstanceSet,
	pub base: BaseStatSet,
	pub current_hp: Health,
	
}

impl PokemonInstance {

	pub fn new(pokemon: &SavedPokemon) -> Option<Self> {

		super::pokedex().get(&pokemon.id).map(|pokemon_data| {
			let stats = get_stats(pokemon_data, pokemon.data.ivs, pokemon.data.evs, pokemon.data.level);

			Self {

				data: pokemon.data.clone(),

				persistent: None,

				item: pokemon.item.as_ref().map(|id| itemdex().get(id).map(|item| item)).flatten(),

				moves: pokemon.moves.as_ref().map(|moves| to_instance(moves)).unwrap_or(pokemon_data.generate_moves(pokemon.data.level)),
	
				base: stats,
				
				current_hp: pokemon.current_hp.unwrap_or(stats.hp),
	
				pokemon: pokemon_data,
				
			}
		})		

	}

	pub fn generate(id: PokemonId, min: Level, max: Level, ivs: Option<StatSet>) -> Self {
		let pokemon = super::pokedex().get(&id).unwrap();

        let level = if min == max {
			max
		} else {
			super::POKEMON_RANDOM.gen_range(min, max + 1) as u8
		};

		let ivs = ivs.unwrap_or(StatSet::random());
		let evs = StatSet::default();

		let base = get_stats(pokemon, ivs, evs, level);

		Self {

			data: PokemonData {
				nickname: None,
				level: level,
				gender: pokemon.generate_gender(),
				ivs: ivs,
				evs: evs,
				experience: 0,
				friendship: 70,
				status: None,
			},

			persistent: None,

			item: None,

			moves: pokemon.generate_moves(level),

			current_hp: base.hp,

			base,
			
			pokemon,
			
		}
	}

	pub fn add_exp(&mut self, experience: super::Experience) -> Option<(Level, Option<Vec<MoveRef>>)> {

		// add exp to pokemon

		self.data.experience += experience * 5;

		// get the maximum exp a player can have at their level

		let max_exp = self.pokemon.training.growth_rate.max_exp(self.data.level);

		// level the pokemon up if they reach a certain amount of exp (and then subtract the exp by the maximum for the previous level)
		if self.data.experience > max_exp {
			self.data.level += 1;
			self.data.experience -= max_exp;

			// Get the moves the pokemon learns at the level it just gained.

			let mut moves = self.moves_at_level();

			// Add moves if the player's pokemon does not have a full set of moves;

			while self.moves.len() < 4 && !moves.is_empty() {
				self.moves.push(MoveInstance::new(moves.remove(0)));
			}
			
			Some((
				self.data.level,
				if !moves.is_empty() {
					Some(moves)
				} else {
					None
				}
			))
		} else {
			None
		}
	}

	pub fn generate_with_level(id: PokemonId, level: Level, ivs: Option<StatSet>) -> Self {
		Self::generate(id, level, level, ivs)
	}

	pub fn to_saved(self) -> SavedPokemon {
		SavedPokemon {
		    id: self.pokemon.data.id,
			data: self.data,
			item: self.item.map(|item| item.id),
		    moves: Some(crate::moves::instance::to_saved(self.moves)),
		    current_hp: Some(self.current_hp),
			owned_data: None,
		}
	}

	pub fn is_faint(&self) -> bool {
		return self.current_hp == 0;
	}

	pub fn name(&self) -> String {
		self.data.nickname.as_ref().map(|name| name.clone()).unwrap_or(self.pokemon.data.name.to_ascii_uppercase())
	}

	pub fn moves_at_level(&self) -> Vec<MoveRef> {
		let mut moves = Vec::new();
		for pokemon_move in &self.pokemon.moves {
			if pokemon_move.level == self.data.level {
				moves.push(crate::moves::movedex().get(&pokemon_move.move_id).unwrap())
			}
		}
		moves
	}

	pub fn effective(&self, pokemon_type: PokemonType) -> Effective {
		let primary = pokemon_type.effective(self.pokemon.data.primary_type);
		if let Some(secondary) = self.pokemon.data.secondary_type {
			primary * pokemon_type.effective(secondary)
		} else {
			primary
		}
	}

	pub fn use_held_item(&mut self) -> bool {
		if let Some(item) = self.item {
			if let Some(conditions) = item.script.conditions.as_ref() {
				for condition in conditions {
					match condition {
					    ItemCondition::BelowHealthPercent(percent) => {
							if (self.current_hp as f32 / self.base.hp as f32) >= *percent {
								return false;
							}
						}
					}
				}
				self.execute_item(item);
				self.item = None;
				true
			} else {
				false
			}
		} else {
			false
		}
	}

	pub fn execute_item(&mut self, item: &Item) {
		for action in &item.script.actions {
			match action {
			    ItemActionKind::CurePokemon(status) => {
					if let Some(effect) = self.data.status {
						if let Some(status) = status {
							if effect.status.eq(status) {
								self.data.status = None;
							}
						} else {
							self.data.status = None;
						}
					}
				}
			    ItemActionKind::HealPokemon(hp) => {
					self.current_hp += *hp;
					if self.current_hp > self.base.hp {
						self.current_hp = self.base.hp;
					}
				}
			}
		}
	}
	
}

impl std::fmt::Display for PokemonInstance {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Lv. {} {}", self.data.level, self.name())
	}
	
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default)]
pub struct BaseStatSet {

	pub hp: Health,
	pub atk: u16,
	pub def: u16,
	pub sp_atk: u16,
	pub sp_def: u16,
	pub speed: u16,

}

pub fn get_stats(pokemon: &Pokemon, ivs: StatSet, evs: StatSet, level: u8) -> BaseStatSet {
    BaseStatSet {
		hp: calculate_hp(pokemon.base.hp, ivs.hp, evs.hp, level),
		atk: calculate_stat(pokemon.base.atk, ivs.atk, evs.atk, level),
		def: calculate_stat(pokemon.base.def, ivs.def, evs.def, level),
		sp_atk: calculate_stat(pokemon.base.sp_atk, ivs.sp_atk, evs.sp_atk, level),
		sp_def: calculate_stat(pokemon.base.sp_def, ivs.sp_def, evs.sp_def, level),
		speed: calculate_stat(pokemon.base.speed, ivs.speed, evs.speed, level),
	}
}

pub fn calculate_stat(base_stat: u8, iv_stat: u8, ev_stat: u8, level: u8) -> u16 { //add item check
	let nature = 1.0;
   (((2.0 * base_stat as f64 + iv_stat as f64 + ev_stat as f64) * level as f64 / 100.0 + 5.0).floor() * nature).floor() as u16
}

pub fn calculate_hp(base_hp: u8, iv_hp: u8, ev_hp: u8, level: u8) -> u16 {
   ((2.0 * base_hp as f64 + iv_hp as f64 + ev_hp as f64) * level as f64 / 100.0 + level as f64 + 10.0).floor() as u16
}