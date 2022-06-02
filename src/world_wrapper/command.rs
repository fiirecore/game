use std::ops::Deref;

use crate::{
    pokedex::{
        item::{ItemId, ItemStack, SavedItemStack},
        pokemon::{stat::Stat, Level, PokemonId, owned::SavedPokemon, stat::StatSet},
    },
    pokengine::PokedexClientData,
};

use worldcli::worldlib::positions::{Location, LocationId};

use super::WorldWrapper;

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
    Party(PartyCommand),
    ClearBattle,
}

pub enum PartyCommand {
    Info(Option<usize>),
}

// pub enum BattleCommand {
//     Random,
// }

// pub enum ScriptCommand {
//     Clear,
//     List,
// }

impl<
D: Deref<Target = PokedexClientData> + Clone,> WorldWrapper<D> {
    pub fn process(result: String) -> Result<WorldCommands, &'static str> {

        let mut args = result.split_ascii_whitespace();

        let (command, mut args) = if let Some(command) = args.next() {
            (command, args)
        } else {
            return Err("Could not parse command!");
        };

        fn on_off<E: Fn(Option<bool>) -> WorldCommands>(
            f: E,
            arg: Option<&str>,
        ) -> Result<WorldCommands, &'static str> {
            match arg {
                Some("on") => Ok((f)(Some(true))),
                Some("off") => Ok((f)(Some(false))),
                None => Ok((f)(None)),
                _ => Err("Please provide an on/off command"),
            }
        }

        match command {
            //         "help" => {
            //             info!("To - do: help list.");
            //             info!("To - do: show messages in game")
            //         }
            //         // "fly_temp" => {
            //         //     self.world_map.spawn();
            //         // }
            "heal" => Ok(WorldCommands::HealPokemon(None)),
            "wild" => {
                on_off(WorldCommands::Wild, args.next())
            }
            "noclip" => {
                on_off(WorldCommands::NoClip, args.next())
            }
            "unfreeze" => {
                Ok(WorldCommands::Unfreeze)
            }
            "debugdraw" => {
                Ok(WorldCommands::DebugDraw)
            }
            "tile" => {
                Ok(WorldCommands::Tile)
            }
            "party" => match args.next() {
                Some(arg) => match arg {
                    "info" => match args.next() {
                        Some(index) => match index.parse::<usize>() {
                            Ok(index) => Ok(WorldCommands::Party(PartyCommand::Info(Some(index)))),
                            Err(..) => Err("Cannot parse party index for /party info"),
                        },
                        None => Ok(WorldCommands::Party(PartyCommand::Info(None))),
                    },
                    _ => Err("Please provide a valid argument for /party"),
                },
                None => Err("Please provide an argument for /party"),
            },
            //         "party" => match args.next() {
            //             Some(arg) => match arg {
            //                 "info" => match args.next() {
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
            //         "battle" => match args.next() {
            //             Some(arg) => match arg {
            //                 "random" => {
            //                     match args.next() {
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
            //         "script" => match args.next() {
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
                if let Some(map_or_index) = args
                    .next().and_then(|a| a.parse::<LocationId>().ok())
                {
                    let location = if let Some(index) = args
                        .next().and_then(|a| a.parse::<LocationId>().ok())
                    {
                        Location {
                            map: Some(map_or_index),
                            index,
                        }
                    } else {
                        Location::from(map_or_index)
                    };
                    Ok(WorldCommands::Warp(location))
                } else {
                    Err("Please provide a location!")
                }
            }
            "clearbattle" => Ok(WorldCommands::ClearBattle),
            "give" => {
                match args.next() {
                    Some(arg) => match arg {
                        "pokemon" => match args
                            .next().and_then(|arg| arg.parse::<PokemonId>().ok())
                        {
                            Some(id) => {
                                let level = args
                                    .next().and_then(|arg| arg.parse::<Level>().ok())
                                    .unwrap_or(5);

                                let ivs = args
                                    .next().and_then(|arg| arg.parse::<Stat>().ok())
                                    .map(StatSet::uniform)
                                    .unwrap_or_default();

                                Ok(WorldCommands::GivePokemon(SavedPokemon {
                                    pokemon: id,
                                    level,
                                    ivs,
                                    ..Default::default()
                                }))
                            }
                            None => Err("Please provide a valid pokemon ID!"),
                        },
                        "item" => {
                            if let Some(item) = args
                                .next().and_then(|item| item.parse::<ItemId>().ok())
                            {
                                let count = args
                                    .next().and_then(|count| count.parse::<usize>().ok())
                                    .unwrap_or(1);

                                Ok(WorldCommands::GiveItem(ItemStack { item, count }))
                            } else {
                                Err("Invalid formatted item ID")
                            }
                        }
                        _ => Err("Please provide an item ID"),
                    },
                    None => Err("Please provide an argument for /give: pokemon, item"),
                }
            }
            _ => Err("Unknown command."),
        }
    }
}
