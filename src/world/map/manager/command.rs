use log::{info, warn};
use pokedex::{Dex, context::PokedexClientContext, item::{ItemId, ItemStack, StackSize}, pokemon::{Level, PokemonId, owned::SavedPokemon, stat::{Stat, StatSet}}};
use saves::PlayerData;
use worldlib::positions::{Location, LocationId};

use crate::{
    game::battle_glue::BattleEntryRef,
    state::game::command::CommandResult,
    world::battle::{random_wild_battle, DEFAULT_RANDOM_BATTLE_SIZE},
};

use super::WorldManager;

impl WorldManager {
    pub fn process<'d>(
        &mut self,
        mut result: CommandResult,
        dex: &PokedexClientContext<'d>,
        save: &mut PlayerData<'d>,
        battle: BattleEntryRef,
    ) {
        match result.command {
            "help" => {
                info!("To - do: help list.");
                info!("To - do: show messages in game")
            }
            // "fly_temp" => {
            //     self.world_map.spawn();
            // }
            "heal" => {
                save.party.iter_mut().for_each(|pokemon| {
                    pokemon.heal(None, None);
                });
                info!("Healed player pokemon.");
            }
            "wild" => {
                self.world.wild.encounters = !self.world.wild.encounters;
                info!("Wild Encounters: {}", self.world.wild.encounters);
            }
            "noclip" => {
                self.world.player.noclip = !self.world.player.noclip;
                info!("Toggled no clip! ({})", self.world.player.noclip);
            }
            "unfreeze" => {
                let player = &mut self.world.player;
                player.unfreeze();
                player.input_frozen = false;
                info!("Unfroze player!");
            }
            "party" => match result.args.next() {
                Some(arg) => match arg {
                    "info" => match result.args.next() {
                        Some(index) => match index.parse::<usize>() {
                            Ok(index) => {
                                if let Some(instance) = save.party.get(index) {
                                    info!(
                                        "Party Slot {}: Lv{} {}",
                                        index,
                                        instance.level,
                                        instance.name()
                                    );
                                } else {
                                    info!("Could not get pokemon at index {}", index);
                                }
                            }
                            Err(err) => warn!(
                                "Could not parse pokemon index for /party with error {}",
                                err
                            ),
                        },
                        None => {
                            for (slot, instance) in save.party.iter().enumerate() {
                                info!(
                                    "Party Slot {}: Lv{} {}",
                                    slot,
                                    instance.level,
                                    instance.name()
                                );
                            }
                        }
                    },
                    _ => (),
                },
                None => self.menu.spawn_party(dex, save),
            },
            "battle" => match result.args.next() {
                Some(arg) => match arg {
                    "random" => {
                        match result.args.next() {
                            Some(len) => match len.parse::<usize>() {
                                Ok(size) => random_wild_battle(
                                    &mut self.randoms.wild,
                                    dex.pokedex.len() as _,
                                    battle,
                                    size,
                                ),
                                Err(err) => {
                                    warn!("Could not parse battle length for second /battle argument \"{}\" with error {}", len, err);
                                }
                            },
                            None => random_wild_battle(
                                &mut self.randoms.wild,
                                dex.pokedex.len() as _,
                                battle,
                                DEFAULT_RANDOM_BATTLE_SIZE,
                            ),
                        };
                    }
                    _ => warn!("Unknown /battle argument \"{}\".", arg),
                },
                None => warn!("Command /battle requires arguments TODO"),
            },
            "script" => match result.args.next() {
                Some(arg) => match arg {
                    "clear" => {
                        save.world.scripts.clear();
                        info!("Cleared used scripts in player data!");
                    }
                    "list" => {
                        if let Some(map) = self.world.get() {
                            info!("{:?}", map.scripts.iter().map(|s| &s.identifier));
                        }
                    }
                    _ => warn!("Unknown /script argument \"{}\".", arg),
                },
                None => warn!("/script requires arguments \"clear\" or \"list\"."),
            },
            "warp" => {
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
                        Location::new(Some(map_or_index), index)
                    } else {
                        Location::new(None, map_or_index)
                    };
                    self.warp_to_location(save, location);
                } else {
                    warn!("Invalid warp command syntax!")
                }
            }
            "give" => match result.args.next() {
                Some(arg) => match arg {
                    "pokemon" => match result.args.next().map(|arg| arg.parse::<PokemonId>().ok()).flatten() {
                        Some(id) => {
                            let level = match result.args.next().map(|arg| arg.parse::<Level>().ok()).flatten() {
                                Some(level) => level,
                                None => 5,
                            };
                            let ivs = match result.args.next().map(|arg| arg.parse::<Stat>().ok()).flatten() {
                                Some(iv) => StatSet::uniform(iv),
                                None => Default::default(),
                            };
                            let mut random = rand::thread_rng();
                            match SavedPokemon::generate(&mut random, id, level, None, Some(ivs)).init(&mut random, dex.pokedex, dex.movedex, dex.itemdex) {
                                Some(p) => if let Err(..) = save.party.try_push(p) {
                                    warn!("Party full!");
                                },
                                None => warn!("Cannot create pokemon!"),
                            }
                            
                        }
                        None => warn!("Please provide a pokemon ID!"),
                    }
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
                                .map(|count| count.parse::<StackSize>().ok())
                                .flatten()
                                .unwrap_or(1);
                            match dex.itemdex.try_get(&item) {
                                Some(item) => {
                                    save.bag.add_item(ItemStack::new(item, count));
                                }
                                None => warn!("Could not get item with id {}", item),
                            }
                        }
                    }
                    _ => warn!("Please provide either \"pokemon\" or \"item\" when using /give!"),
                },
                None => warn!("Please provide either \"pokemon\" or \"item\" when using /give!"),
            },
            _ => warn!("Unknown world command \"{}\".", result),
        }
    }
}
