use deps::rhai::INT;
use deps::rhai::plugin::*;
use deps::rhai::{Dynamic, Engine};

use crate::{
    types::{PokemonType, Effective},
    pokemon::{
        instance::PokemonInstance,
        stat::{StatType, BaseStat},
    },
    moves::{
        Move,
        MoveCategory,
        usage::MoveResult,
        Power,
    },
};


#[derive(Clone, Copy)]
pub struct DamageResult {
    damage: INT,
    effective: Effective,
}

impl DamageResult {
    fn damage(&mut self) -> INT {
        self.damage
    }
    fn set_damage(&mut self, damage: INT) {
        self.damage = damage;
    }
    fn effective(&mut self) -> Effective {
        self.effective
    }
}

impl PokemonInstance {
    fn get_damage_rhai(user: &mut Self, use_type: PokemonType, power: INT, target_def: INT, effective: Effective) -> DamageResult {
        let (damage, effective) = user.get_damage_stat(effective, power as Power, user.base.get(StatType::Attack), target_def as BaseStat, user.pokemon.unwrap().primary_type == use_type);
        DamageResult { damage: damage as INT, effective }
    }
    fn effective_rhai(&mut self, pokemon_type: PokemonType, category: MoveCategory) -> Effective {
        self.effective(pokemon_type, category)
    }
    fn defense_rhai(&mut self, category: MoveCategory) -> INT {
        self.base.get(category.defense()) as INT
    }

    fn current_hp(&mut self) -> INT {
        self.current_hp as INT
    }
    fn primary_type(&mut self) -> PokemonType {
        self.pokemon.unwrap().primary_type
    }
    // fn get_ref(&mut self) -> &Self {
    //     self
    // }
}

pub fn engine() -> Engine {

	let mut engine = Engine::new_raw();

	engine

		// .register_type_with_name::<PokemonType>("Type")
		// .register_fn("effective", PokemonType::effective)

        // .register_type::<Effective>()

        .register_type_with_name::<DamageResult>("Damage")
        .register_get("damage", DamageResult::damage)
        .register_get("effective", DamageResult::effective)
        .register_set("damage", DamageResult::set_damage)

		.register_type_with_name::<MoveCategory>("Category")

		.register_type_with_name::<PokemonInstance>("Pokemon")
		.register_fn("damage",  PokemonInstance::get_damage_rhai)
        .register_fn("effective", PokemonInstance::effective_rhai)
        .register_fn("defense", PokemonInstance::defense_rhai)

        // .register_fn("ref", PokemonInstance::get_ref)
        .register_get("current_hp", PokemonInstance::current_hp)
        .register_get("primary_type", PokemonInstance::primary_type)

		.register_type::<Move>()
        .register_get("category", Move::get_category)
        .register_get("type", Move::get_type)

        // .register_type_with_name::<MoveTargetInstance>("MoveTarget")
        // .register_static_module("MoveTarget", deps::rhai::exported_module!(move_target_instance).into())

        .register_type_with_name::<MoveResult>("MoveResult")
        .register_static_module("MoveResult", deps::rhai::exported_module!(move_result).into())

	;

	engine
}

impl Move {
    fn get_category(&mut self) -> MoveCategory {
        self.category
    }
    fn get_type(&mut self) -> PokemonType {
        self.pokemon_type
    }
}


#[allow(non_snake_case, non_upper_case_globals)]
#[deps::rhai::export_module]
mod move_result {
    use deps::rhai::INT;

    use crate::pokemon::Health;
    use crate::moves::usage::MoveResult;

    use super::DamageResult;

    pub const fn Damage(damage: DamageResult) -> MoveResult { MoveResult::Damage(damage.damage as Health, damage.effective) }
    // pub const fn Status(effect: StatusEffect) -> MoveResult { MoveResult::Status(effect) }
    pub const fn Drain(damage: DamageResult, heal: INT) -> MoveResult { MoveResult::Drain(damage.damage as Health, heal as Health, damage.effective) }
    pub const Todo: MoveResult = MoveResult::Todo;
}