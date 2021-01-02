use crate::engine::game_context::GameContext;
use crate::game::battle::battle::Battle;

use super::pokemon_gui::OpponentPokemonGui;
use super::pokemon_gui::PlayerPokemonGui;

/*

pub trait BattleGuiComponent {
	
	//fn update_gui(&mut self, battle_manager: &mut BattleManager);

	// fn on health change

	// fn on ...
	
}

*/

pub trait BattleGuiButton {

	fn on_use(&mut self, context: &mut GameContext, battle: &mut Battle);

}

pub trait BattleActivatable {

	fn focus(&mut self);

	fn unfocus(&mut self);

	fn in_focus(&mut self) -> bool;

	fn input(&mut self, context: &mut GameContext, battle: &mut Battle, pp_gui: &mut PlayerPokemonGui, op_gui: &mut OpponentPokemonGui);

	fn next(&self) -> u8;

}

