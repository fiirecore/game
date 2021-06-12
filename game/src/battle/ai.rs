use firecore_battle::message::{ClientMessage, ServerMessage};
use pokedex::moves::target::MoveTargetInstance;

use crate::battle::{
    client::{BattleClient, BattleEndpoint},
    pokemon::{
        view::{BattlePartyKnown, BattlePartyUnknown, BattlePartyView},
        BattleMove,
    },
};

#[derive(Default)]
pub struct BattlePlayerAi {
    user: BattlePartyKnown,
    opponent: BattlePartyUnknown,

    messages: Vec<ClientMessage>,
}

impl BattleEndpoint for BattlePlayerAi {
    fn give_client(&mut self, message: ServerMessage) {
        match message {
            ServerMessage::User(_, user) => self.user = user,
            ServerMessage::Opponents(opponent) => self.opponent = opponent,
            ServerMessage::StartSelecting => {
                use crate::battle::BATTLE_RANDOM;

                for (active, pokemon) in self.user.active.iter().enumerate().map(|(i, a)| a.map(|a| (i, a))).flatten() {
                    if let Some(pokemon) = self.user.pokemon.get(pokemon) {
                        // crashes when moves run out
                        let moves: Vec<usize> = pokemon
                            .moves
                            .iter()
                            .enumerate()
                            .filter(|(_, instance)| instance.pp != 0)
                            .map(|(index, _)| index)
                            .collect();

                        self.messages.push(
                            ClientMessage::Move(
                                active, 
                                BattleMove::Move(
                                    moves[BATTLE_RANDOM.gen_range(0, moves.len())],
                                    MoveTargetInstance::opponent(
                                        self.opponent.id,
                                        BATTLE_RANDOM.gen_range(0, self.opponent.active.len()),
                                    ),
                                )
                            )
                        );
                    }
                }
            },

            ServerMessage::TurnQueue(actions) => {
                for instance in actions.iter() {
                    if let firecore_battle::pokemon::BattleClientAction::Move(.., instances) = &instance.action {
                        for (_, moves) in instances {
                            for moves in moves {
                                if let firecore_battle::pokemon::BattleClientMove::Faint(instance) = moves {
                                    if instance.team == self.user.id {
                                        let active = instance.index;
                                        if let Some(pokemon) = self.user.active_mut(active) {
                                            pokemon.set_hp(0.0);
                                        }
                                
                                        let available: Vec<usize> = self
                                            .user
                                            .pokemon
                                            .iter()
                                            .enumerate()
                                            .filter(|(index, pokemon)| active.ne(index) && !pokemon.fainted())
                                            .map(|(index, _)| index)
                                            .collect(); // To - do: use position()
                                
                                        let r = if available.is_empty() {
                                            deps::log::debug!("AI has no inactive pokemon!");
                                            return
                                        } else {
                                            available[crate::battle::BATTLE_RANDOM.gen_range(0, available.len())]
                                        };
                                
                                        self.user.replace(active, Some(r));
                                
                                        self.messages.push(ClientMessage::FaintReplace(active, r));
                                    }
                                }
                            }
                        }
                    }
                }
                self.messages.push(ClientMessage::FinishedTurnQueue);
            },
            ServerMessage::AskFinishedTurnQueue => self.messages.push(ClientMessage::FinishedTurnQueue),
            ServerMessage::FaintReplace(id, active, new) => {
                match id == self.user.id {
                    true => &mut self.user as &mut dyn BattlePartyView,
                    false => &mut self.opponent as _,
                }.replace(active, new)
            },
            ServerMessage::AddUnknown(_, index, unknown) => self.opponent.add(index, unknown),
        }
    }
}

impl BattleClient for BattlePlayerAi {
    fn give_server(&mut self) -> Option<ClientMessage> {
        self.messages.pop()
    }
}