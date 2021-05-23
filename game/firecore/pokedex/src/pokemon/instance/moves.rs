use std::collections::BTreeMap;

use deps::rhai::{Engine, Scope};

use crate::{
    pokemon::{
        instance::{
			PokemonInstance,
			MoveResult,
			TurnResult,
			// HitResult,
		},
        Health,
        POKEMON_RANDOM,
        types::{
            PokemonType,
            Effective,
        },
        status::StatusEffect,
    },
    moves::{
		Move,
        MoveCategory,
        Power,
		result::{
			MoveUseType,
			DamageKind,
		},
		target::MoveTargetInstance,
    },
};

use super::PokemonTarget;

impl PokemonInstance {

	// To - do: multiple targets, uses PP on use
	pub fn use_own_move(&self, engine: &mut Engine, move_index: usize, targets: Vec<PokemonTarget>) -> TurnResult {
		let pokemon_move = &self.moves[move_index].move_ref.unwrap();
		let mut results = BTreeMap::new();

		for target in targets {
			self.use_move_on_target(engine, &mut results, pokemon_move, target);
		}

		TurnResult { pokemon_move, results }
		 // check if target is in move target enum
	}

    pub fn use_move_on_target(&self, engine: &mut Engine, results: &mut BTreeMap<MoveTargetInstance, Option<MoveResult>>, pokemon_move: &'static Move, target: PokemonTarget) {

		let hit = pokemon_move.accuracy.map(|accuracy| {
			let hit: u8 = POKEMON_RANDOM.gen_range(0, 100);
			hit < accuracy
		}).unwrap_or(true);

		if hit {
			match &pokemon_move.use_type {
				MoveUseType::Damage(kind) => {
					results.insert(target.instance, Some(MoveResult::Damage(self.damage_kind(*kind, pokemon_move.category, pokemon_move.pokemon_type, &target.pokemon))));
				}
				MoveUseType::Status(chance, effect) => {
					if let Some(effect) = target.pokemon.can_afflict(*chance, effect) {
						results.insert(target.instance, Some(MoveResult::Status(*effect)));
					}
				}
				MoveUseType::Drain(kind, percent) => {
					let damage = self.damage_kind(*kind, pokemon_move.category, pokemon_move.pokemon_type, &target.pokemon);
					let heal = (damage as f32 * percent) as Health;
					results.insert(target.instance, Some(MoveResult::Drain(damage, heal)));
				}
				// MoveUseType::Linger(..) => {
				// 	results.insert(target.instance, Some(MoveResult::Todo));
				// }
				MoveUseType::Todo => {
					results.insert(target.instance, Some(MoveResult::Todo));
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
										results.insert(target.instance, Some(hit));
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
			results.insert(target.instance, None);
		}
	}

	pub fn damage_kind(&self, kind: DamageKind, category: MoveCategory, pokemon_type: PokemonType, target: &PokemonInstance) -> Health {
		match kind {
			DamageKind::Power(power) => {
				self.get_damage(target, power, category, pokemon_type)
			}
			DamageKind::PercentCurrent(percent) => {
				(target.current_hp as f32 * percent) as Health
			}
			DamageKind::PercentMax(percent) => {
				(target.base.hp as f32 * percent) as Health
			}
			DamageKind::Constant(damage) => damage,
		}
	}

	pub fn can_afflict<'a>(&self, chance: u8, effect: &'a StatusEffect) -> Option<&'a StatusEffect> {
		if self.status.is_none() {
			if chance >= POKEMON_RANDOM.gen_range(1, 11) {
				Some(effect)
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn get_damage(&self, target: &PokemonInstance, power: Power, category: MoveCategory, pokemon_type: PokemonType) -> Health {
		let effective = target.effective(pokemon_type);
		if effective == Effective::Ineffective {
			return 0;
		}
		let effective = effective.multiplier() as f64;
		let (atk, def) = match category {
			MoveCategory::Physical => (self.base.atk as f64, target.base.def as f64),
			MoveCategory::Special => (self.base.sp_atk as f64, target.base.sp_def as f64),
			MoveCategory::Status => (0.0, 0.0),
		};
		(
			(((((2.0 * self.level as f64 / 5.0 + 2.0).floor() * atk * power as f64 / def).floor() / 50.0).floor() * effective) + 2.0)
			* (POKEMON_RANDOM.gen_range(85, 101u8) as f64 / 100.0)
			* (if pokemon_type == self.pokemon.unwrap().primary_type { 1.5 } else { 1.0 })
		) as Health
	}

}