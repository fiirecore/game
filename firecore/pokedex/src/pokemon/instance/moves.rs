use deps::rhai::{Engine, Scope};

use crate::{
	RANDOM,
	types::{
		PokemonType,
		Effective,
	},
    pokemon::{
        instance::PokemonInstance,
        Health,
		stat::BaseStat,
        status::StatusEffect,
    },
    moves::{
		Move,
        MoveCategory,
        Power,
		usage::{
			MoveUseType,
			MoveResult,
			DamageKind,
			MoveResults,
			TurnResult,
			pokemon::PokemonTarget,
		},
    },
};

impl PokemonInstance {

	// To - do: uses PP on use
	pub fn use_own_move(&self, engine: &Engine, move_index: usize, targets: Vec<PokemonTarget>) -> TurnResult {
		let pokemon_move = self.moves[move_index].move_ref;
		let mut results = MoveResults::new();

		for target in targets {
			self.use_move_on_target(engine, &mut results, pokemon_move.value(), target);
		}

		TurnResult { pokemon_move, results }
		 // check if target is in move target enum
	}

    pub fn use_move_on_target(&self, engine: &Engine, results: &mut MoveResults, pokemon_move: &'static Move, target: PokemonTarget) {

		let hit = pokemon_move.accuracy.map(|accuracy| {
			let hit: u8 = RANDOM.gen_range(0, 100);
			hit < accuracy
		}).unwrap_or(true);

		if hit {
			match &pokemon_move.usage {
				MoveUseType::Damage(kind) => {
					let (damage, effective, crit) = self.damage_kind(
						*kind, 
						pokemon_move.category, 
						pokemon_move.pokemon_type, 
						&target.pokemon
					);
					results.insert(target.active, Some(MoveResult::Damage(damage, effective, crit)));
				}
				MoveUseType::Status(chance, effect) => {
					if let Some(effect) = target.pokemon.can_afflict(*chance, effect) {
						results.insert(target.active, Some(MoveResult::Status(*effect)));
					}
				}
				MoveUseType::Drain(kind, percent) => {
					let (damage, effective, crit) = self.damage_kind(*kind, pokemon_move.category, pokemon_move.pokemon_type, &target.pokemon);
					let heal = (damage as f32 * percent) as Health;
					results.insert(target.active, Some(MoveResult::Drain(damage, heal, effective, crit)));
				}
				MoveUseType::StatStage(stat, stage) => {
					if target.pokemon.base.can_change_stage(*stat, *stage) {
						results.insert(target.active, Some(MoveResult::StatStage(*stat, *stage)));
					}
				}
				// MoveUseType::Linger(..) => {
				// 	results.insert(target.instance, Some(MoveResult::Todo));
				// }
				MoveUseType::Todo => {
					results.insert(target.active, Some(MoveResult::Todo));
				}
				MoveUseType::Script(script) => {
					let mut scope = Scope::new();
					scope.push("move", pokemon_move.clone());
					scope.push("user", self.clone());
					scope.push("target", target.pokemon.clone());
					// scope.push("target_instance", target.instance.clone());

					match engine.eval_with_scope::<deps::rhai::Array>(&mut scope, script) {
						Ok(hits) => {
							for hit in hits {
								match hit.try_cast::<MoveResult>() {
									Some(hit) => {
										results.insert(target.active, Some(hit));
									},
									None => panic!("Could not get hit result from returned array for move {}", pokemon_move),
								}
							}
						}
						Err(err) => panic!("{}", err),
					}
				}
			}
		} else {
			results.insert(target.active, None);
		}
	}

	pub fn damage_kind(&self, kind: DamageKind, category: MoveCategory, pokemon_type: PokemonType, target: &PokemonInstance) -> (Health, Effective, bool) {
		match kind {
			DamageKind::Power(power) => {
				self.get_damage(target, power, category, pokemon_type)
			}
			DamageKind::PercentCurrent(percent) => {
				(
					(target.hp() as f32 * percent) as Health,
					Effective::Ineffective,
					false,
				)
			}
			DamageKind::PercentMax(percent) => {
				(
					(target.max_hp() as f32 * percent) as Health,
					Effective::Ineffective,
					false
				)
			}
			DamageKind::Constant(damage) => (
				damage,
				Effective::Ineffective,
				false
			),
		}
	}

	pub fn can_afflict<'a>(&self, chance: u8, effect: &'a StatusEffect) -> Option<&'a StatusEffect> {
		if self.status.is_none() {
			if chance >= RANDOM.gen_range(1, 11) {
				Some(effect)
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn get_damage(&self, target: &PokemonInstance, power: Power, category: MoveCategory, use_type: PokemonType) -> (Health, Effective, bool) {
		let effective = target.effective(use_type, category);
		let (atk, def) = category.stats();
		let (atk, def) = (self.base.get(atk), target.base.get(def));
		self.get_damage_stat(effective, power, atk, def, self.pokemon.value().primary_type == use_type)
	}

	pub fn get_damage_stat(&self, effective: Effective, power: Power, attack: BaseStat, defense: BaseStat, same_type_as_user: bool) -> (Health, Effective, bool) {
		if effective == Effective::Ineffective {
			return (0, effective, false);
		}
		let damage = (
			(((((2.0 * self.level as f64 / 5.0 + 2.0).floor() * attack as f64 * power as f64 / defense as f64).floor() / 50.0).floor() * effective.multiplier() as f64) + 2.0)
			* (RANDOM.gen_range(85, 101u8) as f64 / 100.0)
			* (if same_type_as_user { 1.5 } else { 1.0 })
		) as Health;
		let crit = RANDOM.gen_range(0u8, 16) == 0;
		let damage = (damage * 3) >> 1;
		(damage, effective, crit)
	}

}