use log::{info, warn};
use pokedex::item::{ItemId, ItemStack, StackSize};
use worldlib::positions::{Location, LocationId};

use crate::{
    game::{
        battle_glue::BattleEntryRef,
        storage::{data, data_mut},
    },
    state::game::command::CommandResult,
    world::{
        battle::{random_wild_battle, DEFAULT_RANDOM_BATTLE_SIZE},
        map::WILD_ENCOUNTERS,
    },
};

use super::WorldManager;

impl WorldManager {
    pub fn process(&mut self, mut result: CommandResult, battle: BattleEntryRef) {
        match result.command {
            "help" => {
                info!("To - do: help list.");
                info!("To - do: show messages in game")
            }
            // "fly_temp" => {
            //     self.world_map.spawn();
            // }
            "heal" => {
                data_mut().party.iter_mut().for_each(|pokemon| {
                    pokemon.heal();
                });
                info!("Healed player pokemon.");
            }
            "wild" => {
                use std::sync::atomic::Ordering::Relaxed;
                let wild = !WILD_ENCOUNTERS.load(Relaxed);
                WILD_ENCOUNTERS.store(wild, Relaxed);
                info!("Wild Encounters: {}", wild);
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
                                if let Some(instance) = data().party.get(index) {
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
                            for (slot, instance) in data().party.iter().enumerate() {
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
                None => self.menu.spawn_party(),
            },
            "battle" => match result.args.next() {
                Some(arg) => match arg {
                    "random" => {
                        match result.args.next() {
                            Some(len) => match len.parse::<usize>() {
                                Ok(size) => random_wild_battle(battle, size),
                                Err(err) => {
                                    warn!("Could not parse battle length for second /battle argument \"{}\" with error {}", len, err);
                                }
                            },
                            None => random_wild_battle(battle, DEFAULT_RANDOM_BATTLE_SIZE),
                        };
                    }
                    _ => warn!("Unknown /battle argument \"{}\".", arg),
                },
                None => warn!("Command /battle requires arguments TODO"),
            },
            "script" => match result.args.next() {
                Some(arg) => match arg {
                    "clear" => {
                        data_mut().world.scripts.clear();
                        info!("Cleared used scripts in player data!");
                    }
                    "list" => {
                        if let Some(map) = self.world.get() {
                            info!("{:?}", map.scripts.iter().map(|s|&s.identifier));
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
                    self.warp_to_location(location);
                } else {
                    warn!("Invalid warp command syntax!")
                }
            }
            "give" => {
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
                    data_mut().bag.add_item(ItemStack::new(&item, count));
                }
            }
            _ => warn!("Unknown world command \"{}\".", result),
        }
    }
}
