use crate::{
    pokemon::{
        instance::{
			PokemonInstance,
			MoveResult,
			MoveResults,
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
        MoveCategory,
        Power,
        script::{
            MoveAction,
            MoveActionType,
            DamageKind,
        },
    },
};

impl PokemonInstance {

    pub fn use_move(&mut self, move_index: usize, target: &mut Self) -> Option<MoveResult> {
		if let Some(pokemon_move) = self.moves[move_index].decrement() {
			let pokemon_move = pokemon_move.value();
			let result = if pokemon_move.accuracy.map(|accuracy| {
				let hit: u8 = POKEMON_RANDOM.gen_range(0, 100);
				hit < accuracy
			}).unwrap_or(true) {
				match &pokemon_move.script {
					Some(script) => {
						let actions = match &script.action {
							MoveAction::Action(action) => Some(*action),
							MoveAction::Persistent(_) => {
								todo!("persistent moves");
								// target.persistent = Some(PersistentMoveInstance {
								// 	pokemon_move,
								// 	actions: &persistent.action,
								// 	remaining: persistent.length.map(|(min, max)| POKEMON_RANDOM.gen_range(min, max)),
								// 	should_do: persistent.on_move,
								// });
								// None
							}
						};
						if let Some(action) = actions {
							self.move_action(action, target)
						} else {
							MoveResults::None
						}
					}
					None => {
						match pokemon_move.power {
							Some(power) => {
								self.damage_kind(DamageKind::Move(power, pokemon_move.category, pokemon_move.pokemon_type), target);
								MoveResults::Damage
							},
							None => MoveResults::None,
						}
					}
				}
			} else {
				MoveResults::Miss
			};
			Some(MoveResult { move_ref: pokemon_move, result })
		} else {
			None
		}
	}

	pub fn use_move_on_self(&mut self, move_index: usize) -> Option<MoveResult> { // scuffed
		if let Some(pokemon_move) = self.moves[move_index].decrement() {
			let pokemon_move = pokemon_move.value();
			let result = if pokemon_move.accuracy.map(|accuracy| {
				let hit: u8 = POKEMON_RANDOM.gen_range(0, 100);
				hit < accuracy
			}).unwrap_or(true) {
				match &pokemon_move.script {
					Some(script) => {
						let actions = match &script.action {
							MoveAction::Action(action) => Some(*action),
							MoveAction::Persistent(_) => {
								todo!("persistent moves");
								// target.persistent = Some(PersistentMoveInstance {
								// 	pokemon_move,
								// 	actions: &persistent.action,
								// 	remaining: persistent.length.map(|(min, max)| POKEMON_RANDOM.gen_range(min, max)),
								// 	should_do: persistent.on_move,
								// });
								// None
							}
						};
						if let Some(action) = actions {
							match action {
								MoveActionType::Damage(damage) => {
									{
										let damage = match damage {
											DamageKind::Move(power, category, pokemon_type) => {
												self.get_damage(power, category, pokemon_type, self)
											}
											DamageKind::PercentCurrent(percent) => {
												(self.current_hp as f32 * percent) as Health
											}
											DamageKind::PercentMax(percent) => {
												(self.base.hp as f32 * percent) as Health
											}
											DamageKind::Constant(damage) => damage,
										};
										self.current_hp = self.current_hp.saturating_sub(damage);
									}
									MoveResults::Damage
								},
								MoveActionType::Status(chance, effect) => {
									self.chance_status(chance, effect)
								},
								MoveActionType::Drain(damage, percent) => {
									let damage = {
											let damage = match damage {
											DamageKind::Move(power, category, pokemon_type) => {
												self.get_damage(power, category, pokemon_type, self)
											}
											DamageKind::PercentCurrent(percent) => {
												(self.current_hp as f32 * percent) as Health
											}
											DamageKind::PercentMax(percent) => {
												(self.base.hp as f32 * percent) as Health
											}
											DamageKind::Constant(damage) => damage,
										};
										self.current_hp = self.current_hp.saturating_sub(damage);
										damage
									} as f32;
									self.current_hp += (damage * percent) as Health;
									if self.current_hp > self.base.hp {
										self.current_hp = self.base.hp;
									}
									MoveResults::Damage
								}
							}
						} else {
							MoveResults::None
						}
					}
					None => {
						match pokemon_move.power {
							Some(power) => {
								{
									let damage = match DamageKind::Move(power, pokemon_move.category, pokemon_move.pokemon_type) {
										DamageKind::Move(power, category, pokemon_type) => {
											self.get_damage(power, category, pokemon_type, self)
										}
										DamageKind::PercentCurrent(percent) => {
											(self.current_hp as f32 * percent) as Health
										}
										DamageKind::PercentMax(percent) => {
											(self.base.hp as f32 * percent) as Health
										}
										DamageKind::Constant(damage) => damage,
									};
									self.current_hp = self.current_hp.saturating_sub(damage);
								}
								MoveResults::Damage
							},
							None => MoveResults::None,
						}
					}
				}
			} else {
				MoveResults::Miss
			};
			Some(MoveResult { move_ref: pokemon_move, result })
		} else {
			None
		}
	}

	pub fn get_damage(&self, power: Power, category: MoveCategory, pokemon_type: PokemonType, target: &PokemonInstance) -> Health {
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
			* (if pokemon_type == self.pokemon.value().data.primary_type { 1.5 } else { 1.0 })
		) as Health
	}

	pub fn move_action(&mut self, action: MoveActionType, target: &mut PokemonInstance) -> MoveResults {
		match action {
			MoveActionType::Damage(damage) => {
				self.damage_kind(damage, target);
				MoveResults::Damage
			},
			MoveActionType::Status(chance, effect) => {
				target.chance_status(chance, effect)
			},
			MoveActionType::Drain(damage, percent) => {
				let damage = self.damage_kind(damage, target) as f32;
				self.current_hp += (damage * percent) as Health;
				if self.current_hp > self.base.hp {
					self.current_hp = self.base.hp;
				}
				MoveResults::Damage
			}
		}
	}

	pub fn damage_kind(&self, damage: DamageKind, target: &mut PokemonInstance) -> Health {
		let damage = match damage {
			DamageKind::Move(power, category, pokemon_type) => {
				self.get_damage(power, category, pokemon_type, target)
			}
			DamageKind::PercentCurrent(percent) => {
				(target.current_hp as f32 * percent) as Health
			}
			DamageKind::PercentMax(percent) => {
				(target.base.hp as f32 * percent) as Health
			}
			DamageKind::Constant(damage) => damage,
		};
		target.current_hp = target.current_hp.saturating_sub(damage);
		damage
	}

	fn chance_status(&mut self, chance: u8, effect: StatusEffect) -> MoveResults {
		if self.status.is_none() {
			if chance >= POKEMON_RANDOM.gen_range(1, 11) {
				self.status = Some(effect);
				MoveResults::Status(effect.status)
			} else {
				MoveResults::None
			}
		} else {
			MoveResults::None
		}
	}
}