use deps::random::{Random, RandomState, GLOBAL_STATE};
use pokedex::{
    battle::{
        party::knowable::{BattlePartyKnown, BattlePartyUnknown},
        BattleMove,
    },
    moves::target::{MoveTarget, MoveTargetInstance},
};

use crate::{
    client::{
        action::{BattleClientAction, BattleClientMove},
        BattleClient, BattleEndpoint,
    },
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

                        let move_index = moves[AI_RANDOM.gen_range(0, moves.len())];

                        let target = match &pokemon.moves[move_index].move_ref.target {
                            MoveTarget::Any => MoveTargetInstance::Any(
                                false,
                                AI_RANDOM.gen_range(0, self.opponent.active.len()),
                            ),
                            MoveTarget::Ally => {
                                let index = AI_RANDOM.gen_range(1, self.user.active.len());
                                let index = if index >= active { index + 1 } else { index };
                                MoveTargetInstance::Ally(index)
                            }
                            MoveTarget::Allies => MoveTargetInstance::Allies,
                            MoveTarget::UserOrAlly => MoveTargetInstance::UserOrAlly(
                                AI_RANDOM.gen_range(0, self.user.active.len()),
                            ),
                            MoveTarget::User => MoveTargetInstance::User,
                            MoveTarget::Opponent => MoveTargetInstance::Opponent(
                                AI_RANDOM.gen_range(0, self.opponent.active.len()),
                            ),
                            MoveTarget::AllOpponents => MoveTargetInstance::AllOpponents,
                            MoveTarget::RandomOpponent => MoveTargetInstance::RandomOpponent,
                            MoveTarget::AllOtherPokemon => MoveTargetInstance::AllOtherPokemon,
                            MoveTarget::Todo => MoveTargetInstance::Todo,
                            MoveTarget::UserAndAllies => MoveTargetInstance::UserAndAllies,
                            MoveTarget::AllPokemon => MoveTargetInstance::AllPokemon,
                        };

                        self.messages.push(ClientMessage::Move(
                            active,
                            BattleMove::Move(move_index, target),
                        ));
                    }
                }
            }

            ServerMessage::TurnQueue(actions) => {
                for instance in actions {
                    if let BattleClientAction::Move(.., instances) = &instance.action {
                        for (.., moves) in instances {
                            for moves in moves {
                                match moves {
                                    BattleClientMove::Faint(instance) => {
                                        if instance.team == self.user.id {
                                            let active = instance.index;

                                            if let Some(pokemon) = self
                                                .user
                                                .active
                                                .get(active)
                                                .copied()
                                                .flatten()
                                                .map(|index| self.user.pokemon.get_mut(index))
                                                .flatten()
                                            {
                                                pokemon.current_hp = 0;
                                            }

                                            let available: Vec<usize> = self
                                                .user
                                                .pokemon
                                                .iter()
                                                .enumerate()
                                                .filter(|(index, pokemon)| {
                                                    !self
                                                        .user
                                                        .active
                                                        .iter()
                                                        .any(|u| u == &Some(*index))
                                                        && !pokemon.fainted()
                                                })
                                                .map(|(index, _)| index)
                                                .collect(); // To - do: use position()

                                            if !available.is_empty() {
                                                let r = available
                                                    [AI_RANDOM.gen_range(0, available.len())];
                                                self.user.active[active] = Some(r);

                                                self.messages
                                                    .push(ClientMessage::FaintReplace(active, r));
                                            }
                                        }
                                    }
                                    BattleClientMove::SetExp(.., level) => {
                                        match instance.pokemon.team == self.user.id {
                                            false => {
                                                if let Some(active) = self
                                                    .opponent
                                                    .active
                                                    .get(instance.pokemon.index)
                                                    .copied()
                                                    .flatten()
                                                {
                                                    if let Some(pokemon) = self
                                                        .opponent
                                                        .pokemon
                                                        .get_mut(active)
                                                        .map(Option::as_mut)
                                                        .flatten()
                                                    {
                                                        pokemon.level = *level;
                                                        // Ai does not learn moves
                                                    }
                                                }
                                            }
                                            true => {
                                                if let Some(active) = self
                                                    .user
                                                    .active
                                                    .get(instance.pokemon.index)
                                                    .copied()
                                                    .flatten()
                                                {
                                                    if let Some(pokemon) =
                                                        self.user.pokemon.get_mut(active)
                                                    {
                                                        pokemon.level = *level;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
                self.messages.push(ClientMessage::FinishedTurnQueue);
            }
            ServerMessage::FaintReplace(pokemon, new) => match pokemon.team == self.user.id {
                true => self.user.active[pokemon.index] = new,
                false => self.opponent.active[pokemon.index] = new,
            },
            ServerMessage::AddUnknown(index, unknown) => self.opponent.add_unknown(index, unknown),
            ServerMessage::PokemonRequest(index, instance) => {
                self.opponent.add_instance(index, instance)
            }
            ServerMessage::Winner(..) => (),
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
