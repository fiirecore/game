use rand::{prelude::SmallRng, SeedableRng};
use std::{ops::Deref, rc::Rc};

use worldcli::worldlib::{character::player::InitPlayerCharacter, serialized::SerializedWorld};

use crate::{
    command::CommandProcessor,
    pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex},
    state::StateMessage,
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

use worldcli::map::manager::WorldManager;

use self::{command::WorldCommands, start::StartMenu};

mod command;
mod start;

pub struct WorldWrapper {
    alive: bool,
    pub manager: WorldManager,
    menu: StartMenu,
    commands: CommandProcessor,
    random: SmallRng,
}

impl WorldWrapper {
    pub(crate) fn new(
        gfx: &mut Graphics,
        dex: Rc<PokedexClientData>,
        sender: EventWriter<StateMessage>,
        commands: CommandProcessor,
        world: SerializedWorld,
        debug_font: Font,
    ) -> Result<Self, String> {
        Ok(Self {
            alive: false,
            manager: WorldManager::new(gfx, world, debug_font)?,
            menu: StartMenu::new(dex, sender),
            commands,
            random: SmallRng::seed_from_u64(0),
            // events,
        })
    }

    pub fn seed(&mut self, seed: u64) {
        self.manager.seed(seed);
        self.random = SmallRng::seed_from_u64(seed);
    }

    pub fn update<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        player: &mut InitPlayerCharacter<P, M, I>,
        pokedex: &impl Dex<Pokemon, Output = P>,
        movedex: &impl Dex<Move, Output = M>,
        itemdex: &impl Dex<Item, Output = I>,
        delta: f32,
    ) {
        if pressed(app, plugins, Control::Start) && !player.character.input_lock.active() {
            self.menu.spawn();
        }

        self.manager.update(app, plugins, player, itemdex, delta);

        while let Some(result) = self.commands.commands.borrow_mut().pop() {
            match Self::process(result) {
                Ok(command) => match command {
                    // WorldCommands::Battle(_) => todo!(),
                    // WorldCommands::Script(_) => todo!(),
                    WorldCommands::Warp(location) => {
                        self.manager.try_teleport(player, location);
                    }
                    WorldCommands::Wild(toggle) => {
                        player.state.wild.encounters = match toggle {
                            Some(set) => set,
                            None => !player.state.wild.encounters,
                        }
                    }
                    WorldCommands::NoClip(toggle) => {
                        player.character.noclip = match toggle {
                            Some(set) => set,
                            None => !player.character.noclip,
                        }
                    }
                    WorldCommands::DebugDraw => {
                        player.state.debug_draw = !player.state.debug_draw;
                    }
                    WorldCommands::Unfreeze => {
                        player.character.input_lock.clear();
                        player.character.locked.clear();
                    }
                    WorldCommands::GivePokemon(pokemon) => {
                        if let Some(pokemon) =
                            pokemon.init(&mut self.random, pokedex, movedex, itemdex)
                        {
                            player.give_pokemon(pokemon);
                        }
                        // player.give_pokemon(pokemon);
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
                        match stack.init(itemdex) {
                            Some(stack) => player.trainer.bag.insert(stack),
                            None => info!("Could not initialize item!"),
                        }
                        // player.trainer.bag.insert_saved(stack);
                    }
                    WorldCommands::Tile => match self.manager.get(&player.location) {
                        Some(map) => match map.tile(player.character.position.coords) {
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
                                if let Some(pokemon) = player.trainer.party.get(index) {
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
                                for (index, pokemon) in player.trainer.party.iter().enumerate() {
                                    info!(
                                        "Pokemon #{}: Lv. {} {}",
                                        index, pokemon.level, pokemon.pokemon.name,
                                    )
                                }
                            }
                        },
                    },
                    WorldCommands::ClearBattle => player.state.battle.battling = None,
                },
                Err(err) => self.commands.errors.borrow_mut().push(err),
            }
        }
    }

    pub fn draw<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &self,
        gfx: &mut Graphics,
        player: &InitPlayerCharacter<P, M, I>,
    ) {
        let mut draw = gfx.create_draw();
        draw.set_size(crate::WIDTH, crate::HEIGHT);
        draw.clear(Color::BLACK);
        self.manager.draw(&mut draw, player);
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
        player: &mut InitPlayerCharacter<P, M, I>,
    ) {
        self.menu.ui(app, plugins, egui, &mut player.trainer);
        self.manager.ui(app, plugins, egui, player);
    }
}

impl Entity for WorldWrapper {
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
