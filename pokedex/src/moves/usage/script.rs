use deps::rhai::INT;
use deps::rhai::plugin::*;
use deps::rhai::{Dynamic, Engine};

use crate::{moves::{
        Move,
        MoveCategory,
        usage::MoveResult,
        Power,
    }, pokemon::{Health, instance::PokemonInstance, stat::{StatType, BaseStat}}, types::{PokemonType, Effective}};

use super::DamageResult;

impl DamageResult<INT> {
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

impl From<DamageResult<Health>> for DamageResult<INT> {
    fn from(result: DamageResult<Health>) -> Self {
        Self {
            damage: result.damage as _,
            effective: result.effective,
            crit: result.crit,
        }
    }
}

impl Into<DamageResult<Health>> for DamageResult<INT> {
    fn into(self) -> DamageResult<Health> {
        DamageResult {
            damage: self.damage as _,
            effective: self.effective,
            crit: self.crit,
        }
    }
}

impl PokemonInstance {
    fn get_damage_rhai(user: &mut Self, use_type: PokemonType, power: INT, target_def: INT, effective: Effective, crit_chance: f32) -> DamageResult<INT> {
        user.move_power_damage_stat(effective, power as Power, user.base.get(StatType::Attack), target_def as BaseStat, user.pokemon.primary_type == use_type, crit_chance).map(DamageResult::from).unwrap_or(DamageResult { damage: 0, effective: Effective::Ineffective, crit: false})
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
        self.pokemon.primary_type
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

        .register_type_with_name::<DamageResult<INT>>("Damage")
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
        .register_get("crit_chance", Move::get_crit_chance)

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
    fn get_crit_chance(&mut self) -> f32 {
        self.crit_chance
    }
}


#[allow(non_snake_case, non_upper_case_globals)]
#[deps::rhai::export_module]
mod move_result {
    use deps::rhai::INT;

    use crate::moves::usage::MoveResult;

    use super::DamageResult;

    pub fn Damage(damage: DamageResult<INT>) -> MoveResult { MoveResult::Damage(damage.into()) }
    // pub const fn Status(effect: StatusEffect) -> MoveResult { MoveResult::Status(effect) }
    pub fn Drain(damage: DamageResult<INT>, heal: INT) -> MoveResult { MoveResult::Drain(damage.into(), heal as _) }
}