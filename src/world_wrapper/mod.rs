use rand::{prelude::SmallRng, SeedableRng};
use std::rc::Rc;

use worldcli::{worldlib::{
    character::player::PlayerCharacter,
    events::{split, Sender, Receiver},
    serialized::SerializedWorld,
}, WorldMetaAction};

use crate::engine::{
    controls::{pressed, Control},
    error::ImageError,
    log::info,
    utils::Entity,
    Context, EngineContext,
};

use crate::pokengine::{
    gui::{bag::BagGui, party::PartyGui},
    PokedexClientData,
};

use worldcli::map::manager::WorldManager;

use crate::state::game::GameActions;

use self::{command::WorldCommands, start::StartMenu};

mod command;
mod start;

pub struct WorldWrapper {
    pub manager: WorldManager,

    menu: StartMenu,

    sender: Sender<GameActions>,
    commands: Vec<WorldCommands>,
    receiver: Receiver<WorldMetaAction>,

    random: SmallRng,
    // events: EventReciver<WorldEvents>,
}

impl WorldWrapper {
    pub(crate) fn new(
        ctx: &mut Context,
        dex: Rc<PokedexClientData>,
        sender: Sender<GameActions>,
        world: SerializedWorld,
    ) -> Result<Self, ImageError> {
        let (actions, receiver) = split();
        Ok(Self {
            manager: WorldManager::new(ctx, actions.clone(), world)?,
            menu: StartMenu::new(dex, sender.clone()),
            sender,
            commands: Vec::new(),
            receiver,
            random: SmallRng::seed_from_u64(0),
            // events,
        })
    }

    pub fn seed(&mut self, seed: u64) {
        self.manager.seed(seed);
        self.random = SmallRng::seed_from_u64(seed);
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

            self.manager.update(ctx, eng, player, delta);

            for action in self.receiver.try_iter() {
                match action {
                    WorldMetaAction::Battle(e) => self.sender.send(GameActions::Battle(e)),
                }
            }

            while !self.commands.is_empty() {
                match self.commands.remove(0) {
                    // WorldCommands::Battle(_) => todo!(),
                    // WorldCommands::Script(_) => todo!(),
                    WorldCommands::Warp(location) => {
                        self.manager.try_teleport(player, location);
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
                            match map.tile(player.character.position.coords) {
                                Some(tile) => {
                                    info!("Palette {}, Tile {}", tile.palette(&map.palettes), tile.id());
                                },
                                None => info!("Could not get tile!"),
                            }
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
        }
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext, player: &PlayerCharacter) {
        if !self.menu.fullscreen() {
            self.manager.draw(ctx, eng, player);
        }
        self.menu.draw(ctx, eng)
    }
}
