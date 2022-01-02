use std::rc::Rc;

use crossbeam_channel::Receiver;
use firecore_battle_gui::pokedex::{
    engine::{
        error::ImageError,
        controls::{pressed, Control},
        log::info,
        utils::Entity,
        Context, EngineContext,
    },
    gui::{bag::BagGui, party::PartyGui},
    PokedexClientData,
};
use rand::{prelude::SmallRng, SeedableRng};
use worldlib::{
    character::player::PlayerCharacter,
    events::{split, Sender},
    positions::{Location, Position},
    serialized::SerializedWorld,
};

use crate::state::game::GameActions;

use self::command::WorldCommands;

use super::{gui::StartMenu, map::manager::GameWorldMapManager};

mod command;

pub struct WorldManager {
    manager: GameWorldMapManager,

    menu: StartMenu,

    sender: Sender<GameActions>,
    commands: Sender<WorldCommands>,
    receiver: Receiver<WorldCommands>,

    random: SmallRng,
    // events: EventReciver<WorldEvents>,
}

impl WorldManager {
    pub(crate) fn new(
        ctx: &mut Context,
        dex: Rc<PokedexClientData>,
        party: Rc<PartyGui>,
        bag: Rc<BagGui>,
        sender: Sender<GameActions>,
        world: SerializedWorld,
    ) -> Result<Self, ImageError> {
        let (actions, receiver) = split();
        // let events = EventReciver::default();
        Ok(Self {
            manager: GameWorldMapManager::new(ctx, sender.clone(), world)?,
            menu: StartMenu::new(dex, party, bag, sender.clone()),
            sender,
            commands: actions,
            receiver,
            random: SmallRng::seed_from_u64(0),
            // events,
        })
    }

    pub fn seed(&mut self, seed: u64) {
        self.manager.seed(seed);
        self.random = SmallRng::seed_from_u64(seed);
    }

    pub fn spawn(&self) -> (Location, Position) {
        self.manager.spawn()
    }

    pub fn start(&mut self, player: &mut PlayerCharacter) {
        self.manager.start(player)
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        eng: &mut EngineContext,
        delta: f32,
        player: &mut PlayerCharacter,
        console: bool,
    ) {
        if self.menu.alive() {
            self.menu.update(ctx, eng, delta, &mut player.trainer);
        } else {
            if pressed(ctx, eng, Control::Start) && !player.input_frozen && !console {
                self.menu.spawn();
            }

            for command in self.receiver.try_iter() {
                match command {
                    // WorldCommands::Battle(_) => todo!(),
                    // WorldCommands::Script(_) => todo!(),
                    WorldCommands::Warp(location) => {
                        self.manager.try_warp(player, location);
                    }
                    WorldCommands::Wild(toggle) => {
                        player.world.wild.encounters = match toggle {
                            Some(set) => set,
                            None => !player.world.wild.encounters,
                        }
                    }
                    WorldCommands::NoClip(toggle) => {
                        player.character.noclip = match toggle {
                            Some(set) => set,
                            None => !player.character.noclip,
                        }
                    }
                    WorldCommands::DebugDraw => {
                        player.world.debug_draw = !player.world.debug_draw;
                    }
                    WorldCommands::Unfreeze => {
                        player.input_frozen = false;
                        player.character.unfreeze();
                    }
                    WorldCommands::GivePokemon(pokemon) => {
                        player.give_pokemon(pokemon);
                    }
                    WorldCommands::HealPokemon(index) => match index {
                        Some(index) => {
                            if let Some(p) = player.trainer.party.get_mut(index) {
                                p.heal(None, None);
                            }
                        }
                        None => player
                            .trainer
                            .party
                            .iter_mut()
                            .for_each(|p| p.heal(None, None)),
                    },
                    WorldCommands::GiveItem(stack) => {
                        player.trainer.bag.insert_saved(stack);
                    }
                    WorldCommands::Tile => match self.manager.get(&player.location) {
                        Some(map) => {
                            info!(
                                "Palettes: {:?}, Tile: {:?}",
                                map.palettes,
                                map.tile(player.character.position.coords)
                            );
                        }
                        None => info!("Could not get current map!"),
                    },
                    WorldCommands::Party(command) => match command {
                        command::PartyCommand::Info(index) => match index {
                            Some(index) => {
                                let pokemon = player.trainer.party.get(index);
                                info!("Pokemon #{}: {:?}", index, pokemon,);
                            }
                            None => {
                                for (index, pokemon) in player.trainer.party.iter().enumerate() {
                                    info!(
                                        "Pokemon #{}: Lv. {}, #{}",
                                        index, pokemon.level, pokemon.pokemon
                                    )
                                }
                            }
                        },
                    },
                }
            }

            // {
            //     let mut events = self.events.0.borrow_mut();

            //     events.flush();

            //     drop(events);
            // }

            self.manager.update(ctx, eng, player, delta);
        }
    }

    pub fn post_battle(&mut self, player: &mut PlayerCharacter, winner: bool) {
        self.manager.post_battle(player, winner)
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext, player: &PlayerCharacter) {
        if !self.menu.fullscreen() {
            self.manager.draw(ctx, eng, player);
        }
        self.menu.draw(ctx, eng)
    }
}
