use serde::{Deserialize, Serialize};
use std::ops::Deref;

use pokedex::{
    item::{
        bag::{Bag, SavedBag},
        Item,
    },
    moves::Move,
    pokemon::{
        owned::{OwnedPokemon, SavedPokemon},
        Pokemon,
    },
    Dex,
};

use crate::{
    map::{movement::Elevation, warp::WarpDestination},
    positions::Location,
    state::WorldState,
};

use super::{
    npc::{trainer::NpcTrainer, NpcId},
    trainer::Trainer,
    Character,
};

pub type SavedPlayerCharacter = PlayerCharacter<SavedPokemon, SavedBag>;
pub type InitPlayerCharacter<P, M, I> = PlayerCharacter<OwnedPokemon<P, M, I>, Bag<I>>;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerCharacter<P, B: Default> {
    pub location: Location,
    pub character: Character,
    pub trainer: Trainer<P, B>,

    pub state: WorldState,

    pub cooldown: f32,
    pub rival: String,
}

impl<P, B: Default> PlayerCharacter<P, B> {
    pub fn new(name: impl Into<String>, rival: impl Into<String>) -> Self {
        Self {
            location: Default::default(),
            character: Character {
                name: name.into(),
                ..Default::default()
            },
            trainer: Default::default(),
            state: Default::default(),
            rival: rival.into(),
            cooldown: Default::default(),
        }
    }

    pub fn warp(&mut self, destination: WarpDestination) {
        self.character
            .position
            .from_destination(destination.position);
        self.character.actions.clear();
        self.location = destination.location;
        self.character.position.elevation = Elevation(0);
    }

    pub fn find_battle(
        &mut self,
        map: &Location,
        id: &NpcId,
        trainer: &NpcTrainer,
        character: &mut Character,
    ) -> bool {
        if !self.state.scripts.environment.running()
            && !self.state.battle.battled(map, id)
            && trainer.find_character(character, &mut self.character)
        {
            self.character.locked.increment();
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: f32) -> Option<super::DoMoveResult> {
        let i = match self.character.do_move(delta) {
            Some(action) => match action {
                super::DoMoveResult::Interact => {
                    if !self.character.input_lock.active() {
                        Some(super::DoMoveResult::Interact)
                    } else {
                        None
                    }
                }
                other => Some(other),
            },
            None => None,
        };

        if let text::MessageStates::Finished(cooldown) = &mut self.state.message {
            *cooldown -= delta;
            if *cooldown <= 0.0 {
                self.state.message = text::MessageStates::None;
                self.character.input_lock.decrement();
            }
        }

        i
    }

    pub fn give_pokemon(&mut self, pokemon: P) {
        self.trainer.party.push(pokemon);
    }
}

impl SavedPlayerCharacter {
    pub fn init<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        self,
        random: &mut impl rand::Rng,
        pokedex: &impl Dex<Pokemon, Output = P>,
        movedex: &impl Dex<Move, Output = M>,
        itemdex: &impl Dex<Item, Output = I>,
    ) -> Option<InitPlayerCharacter<P, M, I>> {
        Some(PlayerCharacter {
            location: self.location,
            character: self.character,
            trainer: self.trainer.init(random, pokedex, movedex, itemdex)?,
            state: self.state,
            rival: self.rival,
            cooldown: self.cooldown,
        })
    }
}

impl<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    > InitPlayerCharacter<P, M, I>
{
    pub fn uninit(self) -> SavedPlayerCharacter {
        SavedPlayerCharacter {
            location: self.location,
            character: self.character,
            trainer: self.trainer.uninit(),
            state: self.state,
            rival: self.rival,
            cooldown: self.cooldown,
        }
    }
}
