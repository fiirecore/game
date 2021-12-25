use std::rc::Rc;

use battlelib::pokedex::Initializable;
use crossbeam_channel::Receiver;
use firecore_battle_gui::pokedex::{
    engine::{
        error::ImageError,
        input::controls::{pressed, Control},
        log::info,
        utils::Entity,
        Context,
    },
    gui::{bag::BagGui, party::PartyGui},
    PokedexClientData,
};
use rand::{prelude::SmallRng, SeedableRng};
use worldlib::{
    character::player::PlayerCharacter,
    events::{split, Sender},
    serialized::SerializedWorld,
};

use crate::{saves::PlayerData, state::game::GameActions};

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

    pub fn start(&mut self, player: &mut PlayerCharacter) {
        self.manager.start(player)
    }

    pub fn update(&mut self, ctx: &mut Context, delta: f32, save: &mut PlayerData, console: bool) {
        if self.menu.alive() {
            self.menu
                .update(ctx, delta, &mut save.character.trainer.party, &mut save.bag);
        } else {
            if pressed(ctx, Control::Start) && !save.character.input_frozen && !console {
                self.menu.spawn();
            }

            for command in self.receiver.try_iter() {
                match command {
                    // WorldCommands::Battle(_) => todo!(),
                    // WorldCommands::Script(_) => todo!(),
                    WorldCommands::Warp(location) => {
                        self.manager.try_warp(&mut save.character, location);
                    }
                    WorldCommands::Wild(toggle) => {
                        save.character.world.wild.encounters = match toggle {
                            Some(set) => set,
                            None => !save.character.world.wild.encounters,
                        }
                    }
                    WorldCommands::NoClip(toggle) => {
                        save.character.character.noclip = match toggle {
                            Some(set) => set,
                            None => !save.character.character.noclip,
                        }
                    }
                    WorldCommands::DebugDraw => {
                        save.character.world.debug_draw = !save.character.world.debug_draw;
                    }
                    WorldCommands::Unfreeze => {
                        save.character.input_frozen = false;
                        save.character.unfreeze();
                    }
                    WorldCommands::GivePokemon(pokemon) => {
                        save.character.trainer.party.try_push(pokemon);
                    }
                    WorldCommands::HealPokemon(index) => match index {
                        Some(index) => {
                            if let Some(p) = save.character.trainer.party.get_mut(index) {
                                p.heal(None, None);
                            }
                        }
                        None => save
                            .character
                            .trainer
                            .party
                            .iter_mut()
                            .for_each(|p| p.heal(None, None)),
                    },
                    WorldCommands::GiveItem(item) => {
                        if let Some(stack) = item.init(crate::dex::itemdex()) {
                            save.bag.insert(stack);
                        }
                    }
                    WorldCommands::Tile => {
                        info!(
                            "{:?}",
                            self.manager
                                .get(&save.character.location)
                                .map(|map| map.tile(save.character.position.coords))
                                .flatten()
                        );
                    }
                    WorldCommands::Party(command) => match command {
                        command::PartyCommand::Info(index) => match index {
                            Some(index) => {
                                let pokemon = save.character.trainer.party.get(index);
                                info!(
                                    "Pokemon #{}: {:?}",
                                    index,
                                    pokemon,
                                );
                            }
                            None => {
                                for (index, pokemon) in
                                    save.character.trainer.party.iter().enumerate()
                                {
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

            self.manager
                .update(ctx, &mut save.character, &mut save.bag, delta);
        }
    }

    pub fn post_battle(&mut self, player: &mut PlayerCharacter, winner: bool, trainer: bool) {
        self.manager.post_battle(player, winner, trainer)
    }

    pub fn draw(&self, ctx: &mut Context, save: &PlayerData) {
        if !self.menu.fullscreen() {
            self.manager.draw(ctx, &save.character);
        }
        self.menu.draw(ctx, save)
    }
}