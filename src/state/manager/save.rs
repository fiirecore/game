use std::ops::Deref;

use worldcli::worldlib::character::player::InitPlayerCharacter;

use crate::{
    pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex},
    saves::Player,
};

#[derive(Debug)]
pub enum PlayerSave<
    P: Deref<Target = Pokemon> + Clone,
    M: Deref<Target = Move> + Clone,
    I: Deref<Target = Item> + Clone,
> {
    Uninit(Player),
    Init(InitPlayerCharacter<P, M, I>, String),
    None,
}

impl<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    > Default for PlayerSave<P, M, I>
{
    fn default() -> Self {
        Self::None
    }
}

impl<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    > PlayerSave<P, M, I>
{
    pub fn init(
        &mut self,
        random: &mut impl rand::Rng,
        pokedex: &impl Dex<Pokemon, Output = P>,
        movedex: &impl Dex<Move, Output = M>,
        itemdex: &impl Dex<Item, Output = I>,
    ) -> bool {
        match self {
            PlayerSave::Uninit(..) => {
                let (player, version) = match std::mem::take(self) {
                    PlayerSave::Uninit(Player { player, version }) => (player, version),
                    _ => unreachable!(),
                };

                match player.clone().init(random, pokedex, movedex, itemdex) {
                    Some(init) => {
                        *self = PlayerSave::Init(init, version);
                        true
                    }
                    None => {
                        *self = PlayerSave::Uninit(Player { player, version });
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
            PlayerSave::Init(player, version) => Some(Player {
                version: version.clone(),
                player: player.clone().uninit(),
            }),
            PlayerSave::None => None,
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut InitPlayerCharacter<P, M, I>> {
        match self {
            PlayerSave::Init(p, ..) => Some(p),
            _ => None,
        }
    }

}
