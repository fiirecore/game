use crate::{
    pokedex::{
        item::ItemId,
        pokemon::{stat::Stat, Level, PokemonId},
    },
    state::game::GameActions,
};

use crate::command::CommandProcessor;
use battlelib::pokedex::{
    item::{ItemStack, SavedItemStack},
    pokemon::{owned::SavedPokemon, stat::StatSet},
};
use firecore_world::positions::Location;
use worldlib::positions::LocationId;

use crate::command::CommandResult;

use super::WorldManager;

pub enum WorldCommands {
    // Battle(BattleCommand),
    // Script(ScriptCommand),
    GivePokemon(SavedPokemon),
    GiveItem(SavedItemStack),
    HealPokemon(Option<usize>),
    Warp(Location),
    Wild(Option<bool>),
    NoClip(Option<bool>),
    DebugDraw,
    Unfreeze,
    Tile,
}

pub enum PartyCommand {
    Info(Option<usize>),
}

pub enum BattleCommand {
    Random,
}

pub enum ScriptCommand {
    Clear,
    List,
}

impl WorldManager {
    fn error(&self, error: &'static str) {
        self.sender.send(GameActions::CommandError(error))
    }
}

impl CommandProcessor for WorldManager {
    fn process(&mut self, mut result: CommandResult) {
        fn on_off<E: Fn(Option<bool>) -> WorldCommands>(
            world: &WorldManager,
            f: E,
            arg: Option<&str>,
        ) {
            match arg {
                Some("on") => world.commands.send((f)(Some(true))),
                Some("off") => world.commands.send((f)(Some(false))),
                None => world.commands.send((f)(None)),
                _ => world.error("Please provide an on/off command"),
            }
        }

        match result.command {
            //         "help" => {
            //             info!("To - do: help list.");
            //             info!("To - do: show messages in game")
            //         }
            //         // "fly_temp" => {
            //         //     self.world_map.spawn();
            //         // }
            "heal" => self.commands.send(WorldCommands::HealPokemon(None)),
            "wild" => {
                on_off(self, WorldCommands::Wild, result.args.next());
            }
            "noclip" => {
                on_off(self, WorldCommands::NoClip, result.args.next());
            }
            "unfreeze" => {
                self.commands
                    .send(WorldCommands::Unfreeze);
            }
            "debugdraw" => {
                self.commands
                    .send(WorldCommands::DebugDraw);
            },
            "tile" => {
                self.commands.send(WorldCommands::Tile);
            }
            //         "party" => match result.args.next() {
            //             Some(arg) => match arg {
            //                 "info" => match result.args.next() {
            //                     Some(index) => match index.parse::<usize>() {
            //                         Ok(index) => {
            //                             if let Some(instance) = save.party.get(index) {
            //                                 info!(
            //                                     "Party Slot {}: Lv{} {}",
            //                                     index,
            //                                     instance.level,
            //                                     instance.name()
            //                                 );
            //                             } else {
            //                                 info!("Could not get pokemon at index {}", index);
            //                             }
            //                         }
            //                         Err(err) => warn!(
            //                             "Could not parse pokemon index for /party with error {}",
            //                             err
            //                         ),
            //                     },
            //                     None => {
            //                         for (slot, instance) in save.party.iter().enumerate() {
            //                             info!(
            //                                 "Party Slot {}: Lv{} {}",
            //                                 slot,
            //                                 instance.level,
            //                                 instance.name()
            //                             );
            //                         }
            //                     }
            //                 },
            //                 _ => (),
            //             },
            //             None => self.menu.spawn_party(dex, save),
            //         },
            //         "battle" => match result.args.next() {
            //             Some(arg) => match arg {
            //                 "random" => {
            //                     match result.args.next() {
            //                         Some(len) => match len.parse::<usize>() {
            //                             Ok(size) => random_wild_battle(
            //                                 &mut self.map.randoms.wild,
            //                                 pokedex.len() as _,
            //                                 size,
            //                             ),
            //                             Err(err) => {
            //                                 warn!("Could not parse battle length for second /battle argument \"{}\" with error {}", len, err);
            //                             }
            //                         },
            //                         None => random_wild_battle(
            //                             &mut self.randoms.wild,
            //                             pokedex.len() as _,
            //                             DEFAULT_RANDOM_BATTLE_SIZE,
            //                         ),
            //                     };
            //                 }
            //                 _ => warn!("Unknown /battle argument \"{}\".", arg),
            //             },
            //             None => warn!("Command /battle requires arguments TODO"),
            //         },
            //         "script" => match result.args.next() {
            //             Some(arg) => match arg {
            //                 "clear" => {
            //                     save.world.scripts.clear();
            //                     info!("Cleared used scripts in player data!");
            //                 }
            //                 "list" => {
            //                     if let Some(map) = self.world.get() {
            //                         info!("{:?}", map.scripts.iter().map(|s| &s.identifier));
            //                     }
            //                 }
            //                 _ => warn!("Unknown /script argument \"{}\".", arg),
            //             },
            //             None => warn!("/script requires arguments \"clear\" or \"list\"."),
            //         },
            "warp" | "tp" => {
                if let Some(map_or_index) = result
                    .args
                    .next()
                    .map(|a| a.parse::<LocationId>().ok())
                    .flatten()
                {
                    let location = if let Some(index) = result
                        .args
                        .next()
                        .map(|a| a.parse::<LocationId>().ok())
                        .flatten()
                    {
                        Location {
                            map: Some(map_or_index),
                            index,
                        }
                    } else {
                        Location::from(map_or_index)
                    };
                    self.commands.send(WorldCommands::Warp(location));
                } else {
                    self.error("Please provide a location!");
                }
            }
            "give" => match result.args.next() {
                Some(arg) => match arg {
                    "pokemon" => match result
                        .args
                        .next()
                        .map(|arg| arg.parse::<PokemonId>().ok())
                        // .map(|arg| arg.parse::<PokemonId>().map(Either::Left).ok().or_else(|| arg.parse::<TinyStr16>().map(Either::Right).ok()))
                        .flatten()
                    {
                        Some(id) => {
                            let level = result
                                .args
                                .next()
                                .map(|arg| arg.parse::<Level>().ok())
                                .flatten()
                                .unwrap_or(5);

                            let ivs = result
                                .args
                                .next()
                                .map(|arg| arg.parse::<Stat>().ok())
                                .flatten()
                                .map(StatSet::uniform)
                                .unwrap_or_default();

                            self.commands
                                .send(WorldCommands::GivePokemon(SavedPokemon::generate(
                                    &mut self.random,
                                    id,
                                    level,
                                    None,
                                    Some(ivs),
                                )));
                        }
                        None => self.error("Please provide a valid pokemon ID!"),
                    },
                    "item" => {
                        if let Some(item) = result
                            .args
                            .next()
                            .map(|item| item.parse::<ItemId>().ok())
                            .flatten()
                        {
                            let count = result
                                .args
                                .next()
                                .map(|count| count.parse::<usize>().ok())
                                .flatten()
                                .unwrap_or(1);

                            self.commands
                                .send(WorldCommands::GiveItem(ItemStack { item, count }));
                        }
                    }
                    _ => self.error("Please provide an item ID"),
                },
                None => self.error("Please provide an argument for /give: pokemon, item"),
            },
            _ => self.error("Unknown command."),
        }
    }
}
