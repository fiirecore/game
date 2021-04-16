use std::sync::atomic::Ordering::Relaxed;

use game::{
	data::{get, get_mut, DIRTY, save, player::PlayerSaves},
	input::{pressed, Control},
	scene::SceneState,
	macroquad::prelude::{info, warn, is_key_down, is_key_pressed, KeyCode},
	gui::party::PokemonPartyGui,
	battle::BattleData,
};

use world::map::manager::WorldManager;
use battle::manager::BattleManager;

use crate::scene::Scene;


pub struct GameScene {

	state: SceneState,

	// server: Option<crate::net::Server>,
	// players: game::hash::HashMap<PlayerId, LocalPlayerData>,
	
	world_manager: WorldManager,
	battle_manager: BattleManager,

	party_gui: PokemonPartyGui,
	// pub pokemon_textures: PokemonTextures,
	battle_data: Option<BattleData>,

	battling: bool,

}

impl GameScene {

	pub async fn load(&mut self) {
		match postcard::from_bytes(include_bytes!("../../../build/data/world.bin")) {
			Ok(world) => {
				self.world_manager.load(world);
			}
			Err(err) => {
				panic!("Could not load world file with error {}", err);
			}
		}
	}

	pub fn data_dirty(&mut self, player_data: &mut PlayerSaves) {
		self.save_data(player_data);
		DIRTY.store(false, Relaxed);
	}

    pub fn save_data(&mut self, player_data: &mut PlayerSaves) {
        self.world_manager.save_data(player_data.get_mut());
		info!("Saving player data!");
		if let Err(err) = save(player_data) {
			warn!("Could not save player data with error: {}", err);
		}
    }
	
}

impl Scene for GameScene {

	fn new() -> Self {
		Self {

			state: SceneState::Continue,

			// server: None,
			// players: game::hash::HashMap::new(),

			world_manager: WorldManager::new(),
			battle_manager: BattleManager::new(),
			party_gui: PokemonPartyGui::new(),

			// pokemon_textures: PokemonTextures::default(),

			battle_data: None,

			battling: false,
		}
	}

	fn on_start(&mut self) {
		// unsafe { self.server = crate::net::SERVER.take(); }
		// if let Some(data) = get::<PlayerSaves>() {
		// 	let save = data.get();
		// 	if let Some(server) = self.server.as_mut() {
		// 		server.sender.send(Packet::new(postcard::to_allocvec(&ClientMessage::Join(
		// 			firecore_game::network::message::FullPlayerData {
		// 				map: save.location.map,
		// 				index: save.location.index,
		// 				data: PlayerData {
		// 					pos: PlayerPos {
		// 						coords: save.location.position.local.coords,
		// 						direction: save.location.position.local.direction,
		// 					},
		// 				}
		// 			}
		// 		)).unwrap())).unwrap();
		// 	}
		// }
		self.world_manager.on_start();
	}
	
	fn update(&mut self, delta: f32) {

		// Speed game up if spacebar is held down

		let delta = delta *  if is_key_down(KeyCode::Space) {
			4.0
		} else {
			1.0
		};

		// if let Some(server) = self.server.as_mut() {
		// 	if let Some(id) = server.id {
		// 		while let Some(packet) = server.socket.receive().ok().flatten() {
		// 			match postcard::from_bytes(packet.payload()) {
		// 				Ok(message) => match message {
		// 					ServerMessage::MapPlayers(map) => {
		// 						info!("Received players on map! ({})", map.len());
		// 						self.players = map.into_iter().map(|(id, data)| (id, LocalPlayerData::from(data))).collect();
		// 					}
							
		// 				    ServerMessage::Connect(_) => {
		// 						info!("Received unknown connect message!");
		// 					}
		// 				    ServerMessage::SpawnPlayer(id, data) => {
		// 						info!("Spawned player #{}", id);
		// 						self.players.insert(id, LocalPlayerData::from(data));
		// 					}
		// 				    ServerMessage::MovePlayer(id, pos) => {
		// 						if let Some(player) = self.players.get_mut(&id) {
		// 							player.pos = firecore_game::util::Position {
		// 							    coords: pos.coords,
		// 							    direction: pos.direction,
		// 								..Default::default()
		// 							}
		// 						}
		// 					}
		// 				    ServerMessage::DespawnPlayer(id) => {
		// 						self.players.remove(&id);
		// 					}
		// 				}
		// 				Err(err) => {

		// 				}
		// 			}
		// 		}
		// 	}
		// }

		// save player data if asked to

		if DIRTY.load(Relaxed) {
			if let Some(mut saves) = get_mut::<PlayerSaves>() {
				self.data_dirty(&mut saves);
			}	
		}

		if !self.battling {

			self.world_manager.update(delta, &mut self.battle_data);

			if self.battle_data.is_some() {
				if let Some(player_saves) = get::<PlayerSaves>() {
					if self.battle_manager.battle(&player_saves.get().party, self.battle_data.take().unwrap()) {
						self.battling = true;
					}
				}
			}

		} else {

			self.battle_manager.update(delta, &mut self.party_gui);
			
			if self.battle_manager.is_finished() {
				if let Some(mut player_saves) = get_mut::<PlayerSaves>() {
					let save = player_saves.get_mut();
					if let Some(data) = self.battle_manager.update_data(save) {
						world::battle::update_world(save, data.0, data.1);
					}
				}				
				self.battling = false;
				self.world_manager.map_start(true);
			}

		}

		self.party_gui.update(delta);

	}
	
	fn render(&self) {
		if !self.battling {
			self.world_manager.render();
			
			// if let Some(server) = &self.server {
			// 	if let Some(selfid) = server.id {
			// 		for (id, player) in &self.players {
			// 			if selfid.ne(id) {
			// 				println!("Rendering player #{}", id);
			// 				let x = ((player.pos.coords.x + self.world_manager.render_coords.offset.x) << 4) as f32 - self.world_manager.render_coords.focus.x + player.pos.offset.x;
    		// 				let y = ((player.pos.coords.y - 1 + self.world_manager.render_coords.offset.y) << 4) as f32 - self.world_manager.render_coords.focus.y + player.pos.offset.y;
			// 				game::macroquad::prelude::draw_rectangle(x, y, 16.0, 32.0, game::macroquad::prelude::RED);
			// 			}
			// 		}
			// 	}
			// }
		} else {
			if self.battle_manager.world_active() {
				self.world_manager.render();
			}
			self.battle_manager.render();
		}
		self.party_gui.render();
	}
	
	fn input(&mut self, delta: f32) {
		if self.party_gui.is_alive() {
			self.party_gui.input();
			if pressed(Control::Start) || is_key_pressed(KeyCode::Escape) {
				self.party_gui.despawn();
				if !self.battling {
					if let Some(mut saves) = get_mut::<PlayerSaves>() {
						self.party_gui.on_finish(&mut saves.get_mut().party)
					}
				}
			}
		} else if !self.battling {
			self.world_manager.input(delta, &mut self.battle_data, &mut self.party_gui, &mut self.state);
		} else {
			self.battle_manager.input(&mut self.party_gui);
		}
	}

	fn quit(&mut self) {
		if let Some(mut player_data) = get_mut::<PlayerSaves>() {
			self.save_data(&mut player_data);
		}
		// if let Some(server) = self.server.as_mut() {
		// 	// server.sender.send(Packet::new(postcard::to_allocvec(&ClientMessage::Disconnect).unwrap()));
		// }
		self.state = SceneState::Continue;
	}
	
	fn state(&self) -> SceneState {
		self.state
	}
	
}