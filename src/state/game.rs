use std::{rc::Rc, sync::atomic::Ordering::Relaxed};

use game::{
	util::Entity,
	storage::{PLAYER_SAVES, save, data_mut, player::{SHOULD_SAVE, PlayerSaves}},
	gui::{
		party::PartyGui,
		bag::BagGui,
	},
	game::{GameStateAction, GameState},
	battle_glue::BattleEntry,
	tetra::{
		State, Context, Result,
		time::get_delta_time,
		input::{Key, is_key_down},
	},
	log::{info, warn},
};

use game::world::map::manager::WorldManager;
use game::battle::manager::BattleManager;

use crate::state::{MainState, MainStates};

mod console;

pub struct GameStateManager {

	action: Option<GameStateAction>,

	state: GameStates,
	
	world: WorldManager,
	battle: BattleManager,
	
	battle_entry: Option<BattleEntry>,

	console: console::Console,

}

pub enum GameStates {
	World,
	Battle,
}

impl Default for GameStates {
    fn default() -> Self {
        Self::World
    }
}

impl GameStateManager {

	pub fn new(ctx: &mut Context) -> Self {

		let party = Rc::new(PartyGui::new(ctx));
		let bag = Rc::new(BagGui::new(ctx));


		Self {

			action: None,

			state: GameStates::default(),
			
			world: WorldManager::new(ctx, party.clone(), bag.clone()),
			battle: BattleManager::new(ctx, party, bag),

			battle_entry: None,

			console: console::Console::default(),

		}
	}

	pub fn load(&mut self, ctx: &mut Context) {
		match game::deps::ser::deserialize(include_bytes!("../../build/data/world.bin")) {
			Ok(world) => self.world.load(ctx, world),
			Err(err) => panic!("Could not load world file with error {}", err),
		}
	}

	pub fn data_dirty(&mut self, saves: &mut PlayerSaves) {
		self.save_data(saves);
		SHOULD_SAVE.store(false, Relaxed);
	}

    pub fn save_data(&mut self, saves: &mut PlayerSaves) {
        self.world.save_data(saves.get_mut());
		info!("Saving player data!");
		if let Err(err) = save(saves) {
			warn!("Could not save player data with error: {}", err);
		}
    }
	
}

impl State for GameStateManager {

	fn begin(&mut self, ctx: &mut Context) -> Result {
		self.world.load_with_data();
		self.world.on_start(ctx, &mut self.battle_entry);
		Ok(())
	}

	fn end(&mut self, _: &mut Context) -> Result {
		if let Some(mut saves) = unsafe{PLAYER_SAVES.as_mut()} {
			self.save_data(&mut saves);
		}
		Ok(())
	}

    fn update(&mut self, ctx: &mut Context) -> Result {

		if let Some(command) = self.console.update(ctx) {
			match self.state {
				GameStates::World => self.world.process(command),
				GameStates::Battle => self.battle.process(command),
			}
		}
		
		// Speed game up if spacebar is held down

		let delta = get_delta_time(ctx).as_secs_f32() * if is_key_down(ctx, Key::Space) {
			4.0
		} else {
			1.0
		};

		if SHOULD_SAVE.load(Relaxed) {
			if let Some(mut saves) = unsafe{PLAYER_SAVES.as_mut()} {
				self.data_dirty(&mut saves);
			}	
		}
		match self.state {
			GameStates::World => {
				self.world.update(ctx, delta, self.console.alive(), &mut self.battle_entry, &mut self.action);
				if let Some(entry) = self.battle_entry.take() {
					if self.battle.battle(entry) {
						self.state = GameStates::Battle;
					}
				}
			}
			GameStates::Battle => {
				self.battle.update(ctx, delta, self.console.alive());
				if self.battle.finished {
					let save = data_mut();
					if let Some((winner, trainer)) = self.battle.update_data(save) {
						game::world::battle::update_world(&mut self.world, save, winner, trainer);
					}			
					self.state = GameStates::World;
					self.world.map_start(ctx, true);
				}
			}
		}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result {
		match self.state {
			GameStates::World => self.world.draw(ctx),
			GameStates::Battle => {
				if self.battle.world_active() {
					self.world.draw(ctx);
				}
				self.battle.draw(ctx);
			}
		}
		self.console.draw(ctx);
        Ok(())
    }
}

impl MainState for GameStateManager {
	fn next(&mut self) -> Option<MainStates> {
        self.action.take().map(|action| match action {
			GameStateAction::ExitToMenu => MainStates::Menu,
		})
	}
}