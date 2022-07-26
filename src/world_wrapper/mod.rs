use rand::{prelude::SmallRng, SeedableRng};
use std::ops::Deref;

use crate::{
    command::CommandProcessor,
    pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex},
    state::StateMessage, saves::GameWorldState, random::GameWorldRandoms,
};

use event::EventWriter;

use crate::engine::{
    controls::{pressed, Control},
    graphics::{Color, CreateDraw, Font, Graphics},
    log::{self, info},
    utils::Entity,
    App, Plugins,
};

use crate::pokengine::PokedexClientData;

use worldcli::{map::manager::WorldManager, worldlib::{script::default::DefaultWorldScriptEngine, map::manager::{Maps, WorldMapData}, serialized::SerializedTextures, state::map::MapState}, pokedex::trainer::InitTrainer};

use self::{command::WorldCommands, start::StartMenu};

mod command;
mod start;

pub struct WorldWrapper<D: Deref<Target = PokedexClientData> + Clone> {
    alive: bool,
    pub manager: WorldManager<DefaultWorldScriptEngine>,
    menu: StartMenu<D>,
    commands: CommandProcessor,
    #[deprecated]
    random: SmallRng,
}

impl<D: Deref<Target = PokedexClientData> + Clone> WorldWrapper<D> {
    pub(crate) fn new(
        gfx: &mut Graphics,
        dex: D,
        sender: EventWriter<StateMessage>,
        commands: CommandProcessor,
        world: WorldMapData,
        scripting: DefaultWorldScriptEngine,
        textures: SerializedTextures,
        debug_font: Font,
    ) -> Result<Self, String> {
        Ok(Self {
            alive: false,
            manager: WorldManager::new(gfx, world, scripting, textures, debug_font)?,
            menu: StartMenu::new(dex, sender),
            commands,
            random: SmallRng::seed_from_u64(0),
            // events,
        })
    }

    pub fn update<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        state: &mut GameWorldState,
        randoms: &mut GameWorldRandoms,
        trainer: &mut InitTrainer<P, M, I>,
        pokedex: &impl Dex<Pokemon, Output = P>,
        movedex: &impl Dex<Move, Output = M>,
        itemdex: &impl Dex<Item, Output = I>,
        delta: f32,
    ) {
        if pressed(app, plugins, Control::Start) && !state.map.player.character.input_lock.active() {
            self.menu.spawn();
        }

        #[cfg(debug_assertions)]
        self.test_battle(app, state, trainer, pokedex, movedex, itemdex);

        self.manager.update(app, plugins, state, randoms, trainer, itemdex, delta);

        while let Some(result) = self.commands.commands.borrow_mut().pop() {
            match Self::process(result) {
                Ok(command) => match command {
                    // WorldCommands::Battle(_) => todo!(),
                    // WorldCommands::Script(_) => todo!(),
                    WorldCommands::Warp(location) => {
                        self.manager.try_teleport(&mut state.map, randoms, trainer, location);
                    }
                    WorldCommands::Wild(toggle) => {
                        state.map.player.character.capabilities.encounters = match toggle {
                            Some(set) => set,
                            None => !state.map.player.character.capabilities.encounters,
                        }
                    }
                    WorldCommands::NoClip(toggle) => {
                        state.map.player.character.capabilities.noclip = match toggle {
                            Some(set) => set,
                            None => !state.map.player.character.capabilities.noclip,
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
                        if let Some(pokemon) =
                            pokemon.init(&mut self.random, pokedex, movedex, itemdex)
                        {
                            trainer.party.push(pokemon);
                        }
                        // player.give_pokemon(pokemon);
                    }
                    WorldCommands::HealPokemon(index) => match index {
                        Some(index) => {
                            if let Some(p) = trainer.party.get_mut(index) {
                                p.heal(None, None);
                            }
                        }
                        None => trainer
                            .party
                            .iter_mut()
                            .for_each(|p| p.heal(None, None)),
                    },
                    WorldCommands::GiveItem(stack) => {
                        match stack.init(itemdex) {
                            Some(stack) => trainer.bag.insert(stack),
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
                        if let Some(m) = movedex.try_get(&id) {
                            if let Some(pokemon) = trainer.party.get_mut(index) {
                                pokemon.moves.push(worldcli::pokedex::moves::owned::OwnedMove::from(m.clone()));
                            } else {
                                info!("No pokemon at index {}", index);
                            }
                        }
                    },
                },
                Err(err) => self.commands.errors.borrow_mut().push(err),
            }
        }
    }

    pub fn draw(
        &self,
        gfx: &mut Graphics,
        state: &MapState,
    ) {
        let mut draw = gfx.create_draw();
        draw.set_size(crate::WIDTH, crate::HEIGHT);
        draw.clear(Color::BLACK);
        self.manager.draw(&mut draw, state);
        gfx.render(&draw);
    }

    pub fn ui<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        egui: &crate::engine::egui::Context,
        state: &mut MapState,
        trainer: &mut InitTrainer<P, M, I>,
    ) {
        self.menu.ui(app, plugins, egui, trainer);
        self.manager.ui(app, plugins, egui, state);
    }

    #[cfg(debug_assertions)]
    fn test_battle<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        app: &mut App,
        state: &mut GameWorldState,
        trainer: &mut InitTrainer<P, M, I>,
        pokedex: &impl Dex<Pokemon, Output = P>,
        movedex: &impl Dex<Move, Output = M>,
        itemdex: &impl Dex<Item, Output = I>,
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
                        pokemon: rng.gen_range(0..pokedex.len() as u16),
                        level: 50,
                        ..Default::default()
                    };
                    let pokemon2 = SavedPokemon {
                        pokemon: rng.gen_range(0..pokedex.len() as u16),
                        level: 25,
                        ..Default::default()
                    };
                    trainer
                        .party
                        .push(pokemon1.init(&mut rng, pokedex, movedex, itemdex).unwrap());
                    trainer
                        .party
                        .push(pokemon2.init(&mut rng, pokedex, movedex, itemdex).unwrap());
                }

                let pokemon3 = SavedPokemon {
                    pokemon: rng.gen_range(0..pokedex.len() as u16),
                    level: 25,
                    ..Default::default()
                };
                let pokemon4 = SavedPokemon {
                    pokemon: rng.gen_range(0..pokedex.len() as u16),
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
}

impl<D: Deref<Target = PokedexClientData> + Clone> Entity for WorldWrapper<D> {
    fn spawn(&mut self) {
        self.alive = true;
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn alive(&self) -> bool {
        self.alive
    }
}
