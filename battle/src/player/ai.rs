use deps::random::{GLOBAL_STATE, Random, RandomState};
use pokedex::{
    battle::{
        party::knowable::{BattlePartyKnown, BattlePartyUnknown},
        BattleMove,
    },
    moves::{instance::MoveInstance, target::MoveTargetInstance},
};

use crate::{
    client::{BattleClient, BattleClientAction, BattleClientMove, BattleEndpoint},
    message::{ClientMessage, ServerMessage},
};
                
static AI_RANDOM: Random = Random::new(RandomState::Static(&GLOBAL_STATE));

#[derive(Default)]
pub struct BattlePlayerAi<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> {
    user: BattlePartyKnown<ID>,
    opponent: BattlePartyUnknown<ID>,
    messages: Vec<ClientMessage>,
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> BattlePlayerAi<ID> {
    pub fn new(id: ID) -> Self {
        Self {
            user: BattlePartyKnown::default_with_id(id),
            opponent: BattlePartyUnknown::default_with_id(id),
            messages: Default::default(),
        }
    }
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> BattleEndpoint<ID>
    for BattlePlayerAi<ID>
{
    fn give_client(&mut self, message: ServerMessage<ID>) {
        match message {
            ServerMessage::User(_, user) => self.user = user,
            ServerMessage::Opponents(opponent) => self.opponent = opponent,
            ServerMessage::StartSelecting => {

                for (active, pokemon) in self
                    .user
                    .active
                    .iter()
                    .enumerate()
                    .map(|(i, a)| a.map(|a| (i, a)))
                    .flatten()
                {
                    if let Some(pokemon) = self.user.pokemon.get(pokemon) {
                        // crashes when moves run out
                        let moves: Vec<usize> = pokemon
                            .moves
                            .iter()
                            .enumerate()
                            .filter(|(_, instance)| instance.pp != 0)
                            .map(|(index, _)| index)
                            .collect();

                        self.messages.push(ClientMessage::Move(
                            active,
                            BattleMove::Move(
                                moves[AI_RANDOM.gen_range(0, moves.len())],
                                MoveTargetInstance::opponent(
                                    AI_RANDOM.gen_range(0, self.opponent.active.len()),
                                ),
                            ),
                        ));
                    }
                }
            }

            ServerMessage::TurnQueue(actions) => {
                for instance in actions.iter() {
                    if let BattleClientAction::Move(.., instances) = &instance.action {
                        for (_, moves) in instances {
                            for moves in moves {
                                if let BattleClientMove::Faint(instance) = moves {
                                    if instance.team == self.user.id {

                                        let active = instance.index;

                                        if let Some(pokemon) = self.user.active.get(active).copied().flatten().map(|index| self.user.pokemon.get_mut(index)).flatten() {
                                            pokemon.current_hp = 0;
                                        }

                                        let available: Vec<usize> = self
                                            .user
                                            .pokemon
                                            .iter()
                                            .enumerate()
                                            .filter(|(index, pokemon)| {
                                                !self.user.active.iter().any(|u| u == &Some(*index))
                                                    && !pokemon.fainted()
                                            })
                                            .map(|(index, _)| index)
                                            .collect(); // To - do: use position()

                                        if !available.is_empty() {
                                            let r = available[AI_RANDOM
                                                .gen_range(0, available.len())];
                                            self.user.active[active] = Some(r);

                                            self.messages
                                                .push(ClientMessage::FaintReplace(active, r));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self.messages.push(ClientMessage::FinishedTurnQueue);
            }
            ServerMessage::AskFinishedTurnQueue => {
                self.messages.push(ClientMessage::FinishedTurnQueue)
            }
            ServerMessage::FaintReplace(pokemon, new) => match pokemon.team == self.user.id {
                true => self.user.active[pokemon.index] = new,
                false => self.opponent.active[pokemon.index] = new,
            }
            ServerMessage::AddUnknown(index, unknown) => self.opponent.add_unknown(index, unknown),
            ServerMessage::PokemonRequest(index, instance) => {
                self.opponent.add_instance(index, instance)
            }
            ServerMessage::Winner(..) => (),
            ServerMessage::AddMove(pokemon, _, move_ref) => {
                if pokemon.team == self.user.id {
                    if let Some(pokemon) = self.user.pokemon.get_mut(pokemon.index) {
                        if let Err(_) = pokemon.moves.try_push(MoveInstance::new(move_ref)) {}
                    }
                }
            }
        }
    }
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> BattleClient<ID>
    for BattlePlayerAi<ID>
{
    fn give_server(&mut self) -> Option<ClientMessage> {
        self.messages.pop()
    }
}
