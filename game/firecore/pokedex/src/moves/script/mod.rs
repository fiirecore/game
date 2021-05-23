use deps::rhai::INT;
use deps::rhai::plugin::*;
use deps::rhai::{Dynamic, Engine};

use crate::pokemon::instance::MoveResult;
use crate::{
    pokemon::{
        types::PokemonType,
        instance::PokemonInstance,
    },
    moves::{
        Move,
        MoveCategory,
    },
};


// mod condition;
// pub use condition::*;
// mod action;
// pub use action::*;

// #[derive(Debug, Clone)]
// pub struct HitResult {
//     pub target: MoveTargetInstance,
//     pub result: MoveResult, // maybe needs to be option?
// }

// impl HitResult {
//     pub const fn new(target: MoveTargetInstance, result: MoveResult) -> Self {
//         Self {
//             target,
//             result,
//         }
//     }
// }

impl PokemonInstance {
    fn get_damage_rhai(user: &mut Self, target: Self, power: INT, category: MoveCategory, pokemon_type: PokemonType) -> INT {
        user.get_damage(&target, power as super::Power, category, pokemon_type) as INT
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

		.register_type_with_name::<PokemonType>("Type")
		.register_fn("effective", PokemonType::effective)

		.register_type_with_name::<MoveCategory>("Category")

		.register_type_with_name::<PokemonInstance>("Pokemon")
		.register_fn("damage",  PokemonInstance::get_damage_rhai)

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

// impl deps::rhai::RegisterNativeFunction for Fn(&mut PokemonInstance, &mut PokemonInstance, INT, MoveCategory, PokemonType) -> Clone + SendSync + 'static {
//     fn into_callable_function(self) -> CallableFunction {
//         todo!()
//     }

//     fn param_types() -> Box<[TypeId]> {
//         Box::new([std::any::TypeId::of::<&mut PokemonInstance>(), std::any::TypeId::of::<&mut PokemonInstance>(), std::any::TypeId::of::<INT>(), std::any::TypeId::of::<MoveCategory>(), std::any::TypeId::of::<PokemonType>()])
//     }
// }

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
    use crate::pokemon::instance::MoveResult;

    pub const fn Damage(damage: INT) -> MoveResult { MoveResult::Damage(damage as Health) }
    // pub const fn Status(effect: StatusEffect) -> MoveResult { MoveResult::Status(effect) }
    pub const fn Drain(damage: INT, heal: INT) -> MoveResult { MoveResult::Drain(damage as Health, heal as Health) }
    pub const Todo: MoveResult = MoveResult::Todo;
}

// #[deps::rhai::export_module]
// mod move_target_instance {

//     use crate::moves::target::MoveTargetInstance;

//     pub const fn Opponent(index: i64) -> MoveTargetInstance { MoveTargetInstance::Opponent(index as usize) }
//     pub const fn Team(index: i64) -> MoveTargetInstance { MoveTargetInstance::Team(index as usize) }

//     pub const User: MoveTargetInstance = MoveTargetInstance::User;
//     // pub const AllButUser: MoveTargetInstance = MoveTargetInstance::AllButUser;
//     // pub const Opponents: MoveTargetInstance = MoveTargetInstance::Opponents;
// }