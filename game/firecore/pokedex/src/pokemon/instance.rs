use deps::smallvec::SmallVec;

use crate::item::Item;
use crate::item::ItemRef;
use crate::item::script::ItemActionKind;
use crate::item::script::ItemCondition;
use crate::itemdex;
use crate::moves::MoveRef;
use crate::moves::PokemonMove;
use crate::moves::persistent::PersistentMove;
use crate::moves::script::MoveAction;
use crate::{pokemon::{
		PokemonId,
		Level,
		Pokemon,
		PokemonRef,
		saved::{
			SavedPokemon,
			PokemonData
		},
		data::StatSet,
	},
	moves::instance::MoveInstanceSet,
	moves::saved::to_instance,
};

use super::Health;
use super::types::effective::Effective;

pub type PokemonInstanceParty = SmallVec<[PokemonInstance; 6]>;

#[derive(Clone)]
pub struct PokemonInstance {
	
	pub pokemon: PokemonRef, 
	
	pub data: PokemonData,

	pub persistent: Vec<PersistentMove>,

	pub item: Option<ItemRef>,
	pub moves: MoveInstanceSet,
	pub base: BaseStatSet,
	pub current_hp: Health,
	
}

impl PokemonInstance {

	pub fn new(pokemon: &SavedPokemon) -> Option<Self> {

		crate::pokedex().get(&pokemon.id).map(|pokemon_data| {
			let stats = get_stats(pokemon_data, pokemon.data.ivs, pokemon.data.evs, pokemon.data.level);

			Self {

				data: pokemon.data.clone(),

				persistent: Vec::new(),

				item: pokemon.item.as_ref().map(|id| itemdex().get(id).map(|item| item)).flatten(),

				moves: pokemon.moves.as_ref().map(|moves| to_instance(moves)).unwrap_or(pokemon_data.moves_from_level(pokemon.data.level)),
	
				base: stats,
				
				current_hp: pokemon.current_hp.unwrap_or(stats.hp),
	
				pokemon: pokemon_data,
				
			}
		})		

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
				moves.push(crate::movedex().get(&pokemon_move.move_id).unwrap())
			}
		}
		moves
	}

	pub fn move_effective(&self, pokemon_move: &PokemonMove) -> Effective {
		let primary = pokemon_move.pokemon_type.effective(self.pokemon.data.primary_type);
		if let Some(secondary) = self.pokemon.data.secondary_type {
			primary * pokemon_move.pokemon_type.effective(secondary)
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

	pub fn run_persistent_moves(&mut self) {
		for persistent in &mut self.persistent {
			for action in &persistent.actions {
				match action {
				    MoveAction::Damage(damage) => {
						self.current_hp = self.current_hp.saturating_sub(match *damage {
						    crate::moves::script::DamageKind::PercentCurrent(percent) => (self.current_hp as f32 * percent) as Health,
						    crate::moves::script::DamageKind::PercentMax(percent) => (self.base.hp as f32 * percent) as Health,
						    crate::moves::script::DamageKind::Constant(damage) => damage,
						});
					}
				    MoveAction::Status(chance, effect) => {
						if self.data.status.is_none() {
							if *chance >= super::POKEMON_RANDOM.gen_range(1..11) as u8 {
								self.data.status = Some(*effect);
							}
						}
					}
				    _ => (),
				}
			}
		}
	}
	
}

impl super::GeneratePokemon for PokemonInstance {

    fn generate(id: PokemonId, min: Level, max: Level, ivs: Option<StatSet>) -> Self {

		let pokemon = crate::pokedex().get(&id).unwrap();

        let level = if min == max {
			max
		} else {
			super::POKEMON_RANDOM.gen_range(min as u32..max as u32 + 1) as u8
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

			persistent: Vec::new(),

			item: None,

			moves: pokemon.moves_from_level(level),

			current_hp: base.hp,

			base,
			
			pokemon,
			
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