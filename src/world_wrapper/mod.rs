use std::{sync::Arc, rc::Rc};

use crate::{command::CommandProcessor, random::GameWorldRandoms, saves::GameWorldState, settings::Settings};

use crate::engine::{
    controls::{pressed, Control},
    graphics::{Color, CreateDraw, Graphics},
    log::{self, info},
    App, Plugins,
};

use firecore_battle_client::pokengine::texture::{PokemonTextures, ItemTextures};
use worldcli::{
    map::{data::ClientWorldData, manager::WorldManager},
    pokedex::{moves::owned::OwnedMove, trainer::InitTrainer},
    worldlib::{
        character::CharacterState, map::manager::WorldMapManager,
        script::default::DefaultWorldScriptEngine,
        state::map::MapState,
    },
};

use self::{command::WorldCommands, start::StartMenu};

mod command;
mod start;

pub struct WorldWrapper {
    alive: bool,
    pub manager: WorldManager<DefaultWorldScriptEngine>,
    menu: StartMenu,
    commands: CommandProcessor,
    randoms: GameWorldRandoms,
}

pub enum WorldRequest {
    Save,
    Exit,
}

impl WorldWrapper {
    pub(crate) fn new(
        world: WorldMapManager<DefaultWorldScriptEngine>,
        data: ClientWorldData,
        settings: Rc<Settings>,
        pokemon: Arc<PokemonTextures>,
        items: Arc<ItemTextures>,
        commands: CommandProcessor,
    ) -> Self {
        Self {
            alive: false,
            manager: WorldManager {
                world,
                data,
                warper: Default::default(),
                input: Default::default(),
            },
            menu: StartMenu::new(settings, pokemon, items),
            commands,
            randoms: Default::default(),
            // events,
        }
    }

    pub fn seed(&mut self, seed: u64) {
        self.randoms.seed(seed);
    }

    pub fn start(&mut self, state: &mut GameWorldState, trainer: &mut InitTrainer) {
        self.manager
            .start(state, &mut self.randoms, trainer)
    }

    pub fn update(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        state: &mut GameWorldState,
        trainer: &mut InitTrainer,
        delta: f32,
    ) {
        if pressed(app, plugins, Control::Start) && !state.map.player.character.input_lock.active()
        {
            self.menu.spawn();
        }

        #[cfg(debug_assertions)]
        self.test_battle(app, state, trainer);

        self.manager
            .update(app, plugins, state, &mut self.randoms, trainer, delta);

        while let Some(result) = self.commands.commands.borrow_mut().pop() {
            match Self::process(result) {
                Ok(command) => match command {
                    // WorldCommands::Battle(_) => todo!(),
                    // WorldCommands::Script(_) => todo!(),
                    WorldCommands::Warp(location) => {
                        self.manager.try_teleport(
                            state,
                            &mut self.randoms,
                            trainer,
                            location,
                        );
                    }
                    WorldCommands::Wild(toggle) => {
                        if state
                            .map
                            .player
                            .character
                            .capabilities
                            .contains(&CharacterState::ENCOUNTERS)
                        {
                            state
                                .map
                                .player
                                .character
                                .capabilities
                                .remove(&CharacterState::ENCOUNTERS);
                        } else {
                            state
                                .map
                                .player
                                .character
                                .capabilities
                                .insert(CharacterState::ENCOUNTERS);
                        }
                    }
                    WorldCommands::NoClip(toggle) => {
                        if state
                            .map
                            .player
                            .character
                            .capabilities
                            .contains(&CharacterState::NOCLIP)
                        {
                            state
                                .map
                                .player
                                .character
                                .capabilities
                                .remove(&CharacterState::NOCLIP);
                        } else {
                            state
                                .map
                                .player
                                .character
                                .capabilities
                                .insert(CharacterState::NOCLIP);
                        }
                    }
                    WorldCommands::DebugDraw => {
                        state.map.debug_mode = !state.map.debug_mode;
                    }
                    WorldCommands::Unfreeze => {
                        state.map.player.character.input_lock.clear();
                        state.map.player.character.locked.clear();
                    }
                    WorldCommands::CancelScript => {
                        state.scripts.stop();
                    }
                    WorldCommands::GivePokemon(pokemon) => {
                        match pokemon.init(
                            &mut self.randoms.general,
                            &self.manager.world.pokedex,
                            &self.manager.world.movedex,
                            &self.manager.world.itemdex,
                        ) {
                            Some(pokemon) => trainer.party.push(pokemon),
                            None => info!("Could not initialize pokemon!"),
                        }
                        // player.give_pokemon(pokemon);
                    }
                    WorldCommands::HealPokemon(index) => match index {
                        Some(index) => {
                            if let Some(p) = trainer.party.get_mut(index) {
                                p.heal(None, None);
                            }
                        }
                        None => trainer.party.iter_mut().for_each(|p| p.heal(None, None)),
                    },
                    WorldCommands::GiveItem(stack) => {
                        match stack.init(&self.manager.world.itemdex) {
                            Some(stack) => {
                                trainer.bag.insert(stack);
                                self.manager.update_capabilities(&mut state.map.player.character, trainer);
                            },
                            None => info!("Could not initialize item!"),
                        }
                        // player.trainer.bag.insert_saved(stack);
                    }
                    WorldCommands::Tile => match self.manager.get(&state.map.location) {
                        Some(map) => match map.tile(state.map.player.character.position.coords) {
                            Some(tile) => {
                                info!(
                                    "Palette {}, Tile {}",
                                    tile.palette(&map.palettes),
                                    tile.id()
                                );
                            }
                            None => info!("Could not get tile!"),
                        },
                        None => info!("Could not get current map!"),
                    },
                    WorldCommands::Party(command) => match command {
                        command::PartyCommand::Info(index) => match index {
                            Some(index) => {
                                if let Some(pokemon) = trainer.party.get(index) {
                                    info!(
                                        "Pokemon #{}: Lv. {} {}",
                                        index, pokemon.level, pokemon.pokemon.name
                                    );
                                    log::debug!(" - Moves: {}", pokemon.moves.iter().count());
                                } else {
                                    info!("No pokemon at index {}", index);
                                }
                            }
                            None => {
                                for (index, pokemon) in trainer.party.iter().enumerate() {
                                    info!(
                                        "Pokemon #{}: Lv. {} {}",
                                        index, pokemon.level, pokemon.pokemon.name,
                                    )
                                }
                            }
                        },
                    },
                    WorldCommands::ClearBattle => state.map.player.battle.battling = None,
                    WorldCommands::GiveMove(id, index) => {
                        if let Some(m) = self.manager.world.movedex.try_get(&id) {
                            if let Some(pokemon) = trainer.party.get_mut(index) {
                                pokemon.moves.push(OwnedMove::from(m.clone()));
                                self.manager.update_capabilities(&mut state.map.player.character, trainer);
                            } else {
                                info!("No pokemon at index {}", index);
                            }
                        }
                    }
                    WorldCommands::ToggleCapability(capability) => {
                        match state.map.player.character.capabilities.contains(&capability) {
                            true => state.map.player.character.capabilities.remove(&capability),
                            false => state.map.player.character.capabilities.insert(capability),
                        };
                    },
                    WorldCommands::Capabilities => {
                        info!("Capabilities: {:?}", state.map.player.character.capabilities.iter());
                    },
                },
                Err(err) => self.commands.errors.borrow_mut().push(err),
            }
        }
    }

    pub fn draw(&self, gfx: &mut Graphics, state: &MapState) {
        let mut draw = gfx.create_draw();
        draw.set_size(crate::WIDTH, crate::HEIGHT);
        draw.clear(Color::BLACK);
        self.manager.draw(&mut draw, state);
        gfx.render(&draw);
    }

    pub fn ui(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        egui: &crate::engine::egui::Context,
        state: &mut MapState,
        trainer: &mut InitTrainer,
    ) -> Option<WorldRequest> {
        self.manager.ui(app, plugins, egui, state);
        self.menu.ui(app, plugins, egui, trainer)
    }

    #[cfg(debug_assertions)]
    fn test_battle(
        &mut self,
        app: &mut App,
        state: &mut GameWorldState,
        trainer: &mut InitTrainer,
    ) {
        {
            use crate::engine::notan::prelude::KeyCode;
            use crate::pokedex::pokemon::owned::SavedPokemon;
            use rand::Rng;
            use worldcli::worldlib::map::battle::*;
            if app.keyboard.was_pressed(KeyCode::F1) {
                let mut rng = rand::thread_rng();
                if trainer.party.is_empty() {
                    let pokemon1 = SavedPokemon {
                        pokemon: rng.gen_range(0..self.manager.world.pokedex.len() as u16),
                        level: 50,
                        ..Default::default()
                    };
                    let pokemon2 = SavedPokemon {
                        pokemon: rng.gen_range(0..self.manager.world.pokedex.len() as u16),
                        level: 25,
                        ..Default::default()
                    };
                    trainer.party.push(
                        pokemon1
                            .init(
                                &mut rng,
                                &self.manager.world.pokedex,
                                &self.manager.world.movedex,
                                &self.manager.world.itemdex,
                            )
                            .unwrap(),
                    );
                    trainer.party.push(
                        pokemon2
                            .init(
                                &mut rng,
                                &self.manager.world.pokedex,
                                &self.manager.world.movedex,
                                &self.manager.world.itemdex,
                            )
                            .unwrap(),
                    );
                }

                let pokemon3 = SavedPokemon {
                    pokemon: rng.gen_range(0..self.manager.world.pokedex.len() as u16),
                    level: 25,
                    ..Default::default()
                };
                let pokemon4 = SavedPokemon {
                    pokemon: rng.gen_range(0..self.manager.world.pokedex.len() as u16),
                    level: 50,
                    ..Default::default()
                };

                state.map.player.battle.battling = Some(BattleEntry {
                    id: BattleId::Wild,
                    party: vec![pokemon3, pokemon4],
                    active: 2,
                    trainer: None,
                });
            }
        }
    }
    pub fn spawn(&mut self) {
        self.alive = true;
    }

    pub fn despawn(&mut self) {
        self.alive = false;
    }

    pub fn alive(&self) -> bool {
        self.alive
    }
}
