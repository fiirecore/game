use firecore_battle::message::{ClientMessage, ServerMessage};
use pokedex::moves::{
    instance::MoveInstance,
    target::MoveTargetInstance,
};

use crate::battle::{
    client::{BattleClient, BattleEndpoint},
    pokemon::{
        view::{BattlePartyKnown, BattlePartyUnknown, BattlePartyView},
        BattleMove,
        BattleClientMove,
        BattleClientAction,
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
                                    MoveTargetInstance::opponent(BATTLE_RANDOM.gen_range(0, self.opponent.active.len())),
                                )
                            )
                        );
                    }
                }
            },

            ServerMessage::TurnQueue(actions) => {
                for instance in actions.iter() {
                    if let BattleClientAction::Move(.., instances) = &instance.action {
                        for (_, moves) in instances {
                            for moves in moves {
                                if let BattleClientMove::Faint(instance) = moves {
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
                                            .filter(|(index, pokemon)| !self.user.active.iter().any(|u| u == &Some(*index)) && !pokemon.fainted())
                                            .map(|(index, _)| index)
                                            .collect(); // To - do: use position()
                                        
                                        if !available.is_empty() {
                                            let r = available[crate::battle::BATTLE_RANDOM.gen_range(0, available.len())];
                                            self.user.replace(active, Some(r));
                                
                                            self.messages.push(ClientMessage::FaintReplace(active, r));
                                        }
                                
                                    }
                                }
                            }
                        }
                    }
                }
                self.messages.push(ClientMessage::FinishedTurnQueue);
            },
            ServerMessage::AskFinishedTurnQueue => self.messages.push(ClientMessage::FinishedTurnQueue),
            ServerMessage::FaintReplace(pokemon, new) => match pokemon.team == self.user.id {
                true => &mut self.user as &mut dyn BattlePartyView,
                false => &mut self.opponent as _,
            }.replace(pokemon.index, new),
            ServerMessage::AddUnknown(index, unknown) => self.opponent.add(index, unknown),
            ServerMessage::PokemonRequest(index, instance) => self.opponent.add_instance(index, instance),
            ServerMessage::Winner(..) => (),
            ServerMessage::AddMove(pokemon, _, move_ref) => if pokemon.team == self.user.id {
                if let Some(pokemon) = self.user.pokemon.get_mut(pokemon.index) {
                    if let Err(_)  = pokemon.moves.try_push(MoveInstance::new(move_ref)) {}
                }
            }
        }
    }
}

impl BattleClient for BattlePlayerAi {
    fn give_server(&mut self) -> Option<ClientMessage> {
        self.messages.pop()
    }
}