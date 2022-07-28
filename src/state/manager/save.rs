use std::ops::Deref;

use worldcli::pokedex::trainer::InitTrainer;

use crate::{
    pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex},
    saves::{GameWorldState, Player},
};

#[derive(Debug)]
pub enum PlayerSave {
    Uninit(Player),
    Init(String, GameWorldState, InitTrainer),
    None,
}

impl Default for PlayerSave {
    fn default() -> Self {
        Self::None
    }
}

impl PlayerSave {
    pub fn init(
        &mut self,
        random: &mut impl rand::Rng,
        pokedex: &Dex<Pokemon>,
        movedex: &Dex<Move>,
        itemdex: &Dex<Item>,
    ) -> bool {
        match self {
            PlayerSave::Uninit(..) => {
                let (version, world, trainer) = match std::mem::take(self) {
                    PlayerSave::Uninit(Player {
                        version,
                        world,
                        trainer,
                    }) => (version, world, trainer),
                    _ => unreachable!(),
                };

                match trainer.clone().init(random, pokedex, movedex, itemdex) {
                    Some(init) => {
                        *self = PlayerSave::Init(version, world, init);
                        true
                    }
                    None => {
                        *self = PlayerSave::Uninit(Player {
                            version,
                            world,
                            trainer,
                        });
                        false
                    }
                }
            }
            PlayerSave::Init(..) => true,
            PlayerSave::None => false,
        }
    }

    pub fn cloned_uninit(&self) -> Option<Player> {
        match self {
            PlayerSave::Uninit(player) => Some(player.clone()),
            PlayerSave::Init(version, world, trainer) => Some(Player {
                version: version.clone(),
                world: world.clone(),
                trainer: trainer.clone().uninit(),
            }),
            PlayerSave::None => None,
        }
    }

    pub fn as_mut(&mut self) -> Option<(&mut GameWorldState, &mut InitTrainer)> {
        match self {
            PlayerSave::Init(.., w, t) => Some((w, t)),
            _ => None,
        }
    }
}
