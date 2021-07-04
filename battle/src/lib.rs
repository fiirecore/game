// #![feature(map_into_keys_values)] // for move queue fn

extern crate firecore_dependencies as deps;
extern crate firecore_pokedex as pokedex;

use {
    deps::{
        log::{debug, info, warn},
        random::{Random, RandomState, GLOBAL_STATE},
        rhai::Engine,
    },
    pokedex::{
        item::ItemUseType,
        moves::{
            target::{MoveTargetInstance, MoveTargetLocation},
            usage::{MoveResult, PokemonTarget},
        },
        pokemon::{instance::PokemonInstance, Health},
        types::Effective,
    },
};

use std::ops::Deref;

use pokedex::{
    battle::{ActionInstance, ActivePokemon, BattleMove, PokemonIndex},
    moves::usage::{DamageResult, NoHitResult},
};

use crate::{
    client::action::{BattleClientAction, BattleClientMove},
    data::*,
    message::{ClientMessage, ServerMessage},
    player::BattlePlayer,
    state::BattleState,
};

pub mod data;
pub mod state;

pub mod player;

pub mod client;
pub mod message;

pub static BATTLE_RANDOM: Random = Random::new(RandomState::Static(&GLOBAL_STATE));

pub struct Battle<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq + Ord> {
    ////////////// if using hashmap, only remaining player should be winner
    state: BattleState<ID>,

    data: BattleData,

    player1: BattlePlayer<ID>,
    player2: BattlePlayer<ID>,
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq + Ord> Battle<ID> {
    pub fn new(data: BattleData, player1: BattlePlayer<ID>, player2: BattlePlayer<ID>) -> Self {
        Self {
            state: BattleState::default(),
            data,
            player1,
            player2,
        }
    }

    pub fn data(&self) -> &BattleData {
        &self.data
    }

    pub fn begin(&mut self) {
        self.player1.client.send(ServerMessage::User(
            self.data.clone(),
            self.player1.party.as_known(),
        ));
        self.player2.client.send(ServerMessage::User(
            self.data.clone(),
            self.player2.party.as_known(),
        ));

        self.player1.party.reveal_active();
        self.player2.party.reveal_active();

        self.player1
            .client
            .send(ServerMessage::Opponents(self.player2.party.as_unknown()));
        self.player2
            .client
            .send(ServerMessage::Opponents(self.player1.party.as_unknown()));

        self.state = BattleState::StartSelecting;
    }

    fn _receive(&mut self, player1: bool) {
        let (user, other) = match player1 {
            true => (&mut self.player1, &mut self.player2),
            false => (&mut self.player2, &mut self.player1),
        };
        while let Some(message) = user.client.receive() {
            match message {
                ClientMessage::Move(active, bmove) => {
                    if let Some(pokemon) = user.party.active.get_mut(active) {
                        match pokemon {
                            ActivePokemon::Some(.., queued_move) => {
                                *queued_move = Some(bmove);
                                // party.client.confirm_move(active);
                            }
                            _ => warn!(
                                "Party {} could not add move #{:?} to pokemon #{}",
                                user.party.id, bmove, active
                            ),
                        }
                    }
                }
                ClientMessage::FaintReplace(active, index) => {
                    debug!(
                        "faint replace: {} replacing {} with {}",
                        user.party.name(),
                        active,
                        index
                    );
                    if !user.party.active_contains(index) {
                        if if let Some(pokemon) = user.party.pokemon.get(index) {
                            if !pokemon.pokemon.fainted() {
                                user.party.active[active] = ActivePokemon::Some(index, None);
                                if let Some(pokemon) = user.party.know(index) {
                                    other.client.send(ServerMessage::AddUnknown(index, pokemon));
                                }
                                other.client.send(ServerMessage::FaintReplace(
                                    PokemonIndex {
                                        team: user.party.id,
                                        index: active,
                                    },
                                    Some(index),
                                ));
                                false
                            } else {
                                true
                            }
                        // party.client.confirm_replace(active, index);
                        } else {
                            true
                        } {
                            warn!("Player {} tried to replace a fainted pokemon with an missing/fainted pokemon", user.party.name());
                        }
                    } else {
                        warn!("Player {} tried to replace a pokemon with a pokemon that is already active.", user.party.name());
                    }
                }
                ClientMessage::RequestPokemon(request) => {
                    if let Some(pokemon) = other.party.pokemon.get(request) {
                        if matches!(self.data.type_, BattleType::Wild)
                            || pokemon.requestable
                            || (pokemon.pokemon.fainted() && pokemon.known)
                        {
                            user.client.send(ServerMessage::PokemonRequest(
                                request,
                                pokemon.pokemon.deref().clone(),
                            ));
                        }
                    }
                }
                ClientMessage::Forfeit => Self::set_winner(other.party.id, user, other),
                ClientMessage::FinishedTurnQueue => (),
                ClientMessage::AddLearnedMove(pokemon, index, move_ref) => {
                    if let Some(pokemon) = user.party.pokemon.get_mut(pokemon) {
                        if pokemon.learnable_moves.contains(&move_ref) {
                            pokemon.pokemon.replace_move(index, move_ref);
                        }
                    }
                }
            }
        }
    }

    pub fn update(&mut self, engine: &Engine) {
        self._receive(true);
        self._receive(false);

        match self.state {
            BattleState::Setup => self.begin(),

            BattleState::StartWait => (),
            BattleState::StartSelecting => {
                self.player1.client.new_turn();
                self.player2.client.new_turn();
                self.player1.client.send(ServerMessage::StartSelecting);
                self.player2.client.send(ServerMessage::StartSelecting);
                self.state = BattleState::WaitSelecting;
            }
            BattleState::WaitSelecting => {
                if self.player1.party.ready_to_move() && self.player2.party.ready_to_move() {
                    self.state = BattleState::StartMoving;
                }
            }
            BattleState::StartMoving => {
                let queue =
                    pokedex::battle::move_queue(&mut self.player1.party, &mut self.player2.party);

                let player_queue = self.client_queue(engine, queue);

                // end queue calculations

                self.player2
                    .client
                    .send(ServerMessage::TurnQueue(player_queue.clone()));
                self.player1
                    .client
                    .send(ServerMessage::TurnQueue(player_queue));

                self.state = BattleState::WaitMoving;
            }
            BattleState::WaitMoving => {
                if self.player1.client.finished_turn() && self.player2.client.finished_turn() {
                    if self.player2.party.all_fainted() {
                        info!("{} wins!", self.player1.party.name());
                        Self::set_winner(
                            self.player1.party.id,
                            &mut self.player1,
                            &mut self.player2,
                        );
                    } else if self.player1.party.all_fainted() {
                        info!("{} wins!", self.player2.party.name());
                        Self::set_winner(
                            self.player2.party.id,
                            &mut self.player1,
                            &mut self.player2,
                        );
                    } else if !self.player1.party.needs_replace()
                        && !self.player2.party.needs_replace()
                    {
                        self.state = BattleState::StartSelecting;
                    }
                }
            }
            BattleState::End(..) => (),
        }
    }

    fn set_winner(winner: ID, player1: &mut BattlePlayer<ID>, player2: &mut BattlePlayer<ID>) {
        let party = player1
            .settings
            .request_party
            .then(|| Box::new(player1.party.cloned()));
        player1.client.send(ServerMessage::Winner(winner, party));
        let party = player2
            .settings
            .request_party
            .then(|| Box::new(player2.party.cloned()));
        player2.client.send(ServerMessage::Winner(winner, party));
    }

    pub fn finished(&self) -> bool {
        matches!(self.state, BattleState::End(..))
    }

    pub fn winner(&self) -> Option<&ID> {
        match &self.state {
            BattleState::End(.., winner) => Some(winner),
            _ => None,
        }
    }

    pub fn client_queue(
        &mut self,
        engine: &Engine,
        queue: Vec<ActionInstance<ID, BattleMove>>,
    ) -> Vec<ActionInstance<ID, BattleClientAction<ID>>> {
        let mut player_queue = Vec::with_capacity(queue.len());

        for instance in queue {
            let (user, other) = match instance.pokemon.team == self.player1.party.id {
                true => (&mut self.player1, &mut self.player2),
                false => (&mut self.player2, &mut self.player1),
            };
            if user.party.active(instance.pokemon.index).is_some() {
                match instance.action {
                    BattleMove::Move(move_index, target) => {
                        let user_pokemon = user.party.active(instance.pokemon.index).unwrap();

                        let targets = match target {
                            MoveTargetInstance::Any(user, index) => vec![match user {
                                true => match index == instance.pokemon.index {
                                    true => MoveTargetLocation::User,
                                    false => MoveTargetLocation::Team(index),
                                },
                                false => MoveTargetLocation::Opponent(index),
                            }],
                            MoveTargetInstance::Ally(index) => {
                                vec![MoveTargetLocation::Team(index)]
                            }
                            MoveTargetInstance::Allies => MoveTargetLocation::allies(
                                instance.pokemon.index,
                                user.party.active.len(),
                            ),
                            MoveTargetInstance::UserOrAlly(index) => {
                                vec![match index == instance.pokemon.index {
                                    true => MoveTargetLocation::User,
                                    false => MoveTargetLocation::Team(index),
                                }]
                            }
                            MoveTargetInstance::User => vec![MoveTargetLocation::User],
                            MoveTargetInstance::Opponent(index) => {
                                vec![MoveTargetLocation::Opponent(index)]
                            }
                            MoveTargetInstance::AllOpponents => {
                                MoveTargetLocation::opponents(other.party.active.len())
                            }
                            MoveTargetInstance::RandomOpponent => {
                                vec![MoveTargetLocation::Opponent(
                                    BATTLE_RANDOM.gen_range(0, other.party.active.len()),
                                )]
                            }
                            MoveTargetInstance::AllOtherPokemon => {
                                MoveTargetLocation::all_other_pokemon(
                                    instance.pokemon.index,
                                    user.party.active.len(),
                                    other.party.active.len(),
                                )
                            }
                            MoveTargetInstance::Todo => {
                                warn!(
                                    "Could not use move '{}' because it has no target implemented.",
                                    user_pokemon
                                        .pokemon
                                        .moves
                                        .get(move_index)
                                        .map(|m| m.move_ref.name.as_str())
                                        .unwrap_or("Unknown")
                                );
                                vec![]
                            }
                            MoveTargetInstance::UserAndAllies => MoveTargetLocation::user_and_allies(instance.pokemon.index, user.party.active.len()),
                            MoveTargetInstance::AllPokemon => MoveTargetLocation::all_pokemon(instance.pokemon.index, user.party.active.len(), other.party.active.len()),
                        };

                        let targets = targets
                            .into_iter()
                            .flat_map(|target| {
                                match target {
                                    MoveTargetLocation::Opponent(index) => {
                                        other.party.active(index)
                                    }
                                    MoveTargetLocation::Team(index) => user.party.active(index),
                                    MoveTargetLocation::User => Some(user_pokemon),
                                }
                                .map(|i| (target, i))
                            })
                            .map(|(target, pokemon)| PokemonTarget {
                                pokemon: &pokemon.pokemon,
                                active: target,
                            })
                            .collect();

                        let turn = user_pokemon
                            .pokemon
                            .use_own_move(engine, move_index, targets);

                        let mut target_results = Vec::with_capacity(turn.results.len());

                        for (target_instance, result) in turn.results {
                            let mut client_results = Vec::new();

                            {
                                let user = match instance.pokemon.team == self.player1.party.id {
                                    true => &mut self.player1,
                                    false => &mut self.player2,
                                };

                                let user = &mut user
                                    .party
                                    .active_mut(instance.pokemon.index)
                                    .unwrap()
                                    .pokemon;

                                for result in &result {
                                    match result {
                                        MoveResult::Drain(_, heal, ..) => {
                                            user.current_hp =
                                                (user.current_hp + *heal).min(user.base.hp());
                                            client_results.push(BattleClientMove::UserHP(
                                                user.current_hp as f32 / user.max_hp() as f32,
                                            ));
                                        }
                                        _ => (),
                                    }
                                }
                            }

                            for result in result {

                                let (user, other) =
                                    match instance.pokemon.team == self.player1.party.id {
                                        true => (&mut self.player1, &mut self.player2),
                                        false => (&mut self.player2, &mut self.player1),
                                    };

                                let (target_party, index) = match target_instance {
                                    MoveTargetLocation::Opponent(index) => (other, index),
                                    MoveTargetLocation::User => (user, instance.pokemon.index),
                                    MoveTargetLocation::Team(index) => (user, index),
                                };

                                if target_party.party.active(index).is_none() {
                                    debug!("{:?}", target_party.party);
                                }

                                if let Some(target) = target_party.party.active_mut(index) {
                                    fn on_damage<
                                        ID: Sized
                                            + Copy
                                            + core::fmt::Debug
                                            + core::fmt::Display
                                            + PartialEq,
                                    >(
                                        pokemon: &mut PokemonInstance,
                                        results: &mut Vec<BattleClientMove<ID>>,
                                        result: DamageResult<Health>,
                                    ) {
                                        pokemon.current_hp =
                                            pokemon.current_hp.saturating_sub(result.damage);
                                        results.push(BattleClientMove::TargetHP(
                                            pokemon.hp() as f32 / pokemon.max_hp() as f32,
                                            result.crit,
                                        ));
                                        if !matches!(result.effective, Effective::Effective) {
                                            results.push(BattleClientMove::Effective(result.effective));
                                        }
                                    }
    
                                    match result {
                                        MoveResult::Damage(result) => {
                                            on_damage(&mut target.pokemon, &mut client_results, result)
                                        }
                                        MoveResult::Status(effect) => {
                                            target.pokemon.effect = Some(effect);
                                            client_results.push(BattleClientMove::Status(effect));
                                        }
                                        MoveResult::Drain(result, ..) => {
                                            on_damage(&mut target.pokemon, &mut client_results, result)
                                        }
                                        MoveResult::StatStage(stat) => {
                                            target.pokemon.base.change_stage(stat);
                                            client_results.push(BattleClientMove::StatStage(stat));
                                        }
                                        MoveResult::Flinch => target.flinch = true,
                                        MoveResult::NoHit(result) => match result {
                                            NoHitResult::Ineffective => client_results.push(
                                                BattleClientMove::Effective(Effective::Ineffective),
                                            ),
                                            NoHitResult::Miss => {
                                                client_results.push(BattleClientMove::Miss);
                                                continue;
                                            }
                                            NoHitResult::Todo => client_results.push(BattleClientMove::Fail),
                                        },
                                    }
    
                                    if target.pokemon.fainted() {
                                        let experience = target.pokemon.exp_from();
                                        let experience =
                                            match matches!(self.data.type_, BattleType::Wild) {
                                                true => experience.saturating_mul(3) / 2,
                                                false => experience,
                                            };
    
                                        #[cfg(debug_assertions)]
                                        let experience = experience.saturating_mul(7);
    
                                        client_results.push(BattleClientMove::Faint(PokemonIndex {
                                            team: target_party.party.id,
                                            index,
                                        }));
    
                                        let inactive = target_party.party.any_inactive();
                                        if let Some(active) = target_party.party.active.get_mut(index) {
                                            if inactive {
                                                *active = ActivePokemon::ToReplace;
                                            } else {
                                                *active = ActivePokemon::None;
                                            }
                                        }
    
                                        if target_party.party.id != instance.pokemon.team {
                                            let user =
                                                match instance.pokemon.team == self.player1.party.id {
                                                    true => (&mut self.player1),
                                                    false => (&mut self.player2),
                                                };
    
                                            if user.settings.gains_exp {
                                                let pokemon = &mut user
                                                    .party
                                                    .active_mut(instance.pokemon.index)
                                                    .unwrap();
    
                                                pokemon
                                                    .learnable_moves
                                                    .extend(pokemon.pokemon.add_exp(experience));
    
                                                client_results.push(BattleClientMove::SetExp(
                                                    experience,
                                                    pokemon.pokemon.level,
                                                ));
                                            }
                                        }
                                    }
                                }
                            }

                            target_results.push((target_instance, client_results));
                        }

                        player_queue.push(ActionInstance {
                            pokemon: instance.pokemon,
                            action: BattleClientAction::Move(turn.pokemon_move, target_results),
                        });
                    }
                    BattleMove::UseItem(item, target) => {
                        if match &item.usage {
                            ItemUseType::Script(script) => {
                                user.party
                                    .active_mut(instance.pokemon.index)
                                    .unwrap()
                                    .pokemon
                                    .execute_item_script(script);
                                true
                            }
                            ItemUseType::Pokeball => match self.data.type_ {
                                BattleType::Wild => {
                                    let (target, active) = match target {
                                        MoveTargetLocation::Opponent(index) => (other, index),
                                        _ => unreachable!(),
                                    };
                                    if let Some(active) = target.party.active.get_mut(active) {
                                        let active = std::mem::replace(active, ActivePokemon::None);
                                        if let Some(index) = active.index() {
                                            if target.party.pokemon.len() > index {
                                                let mut pokemon =
                                                    target.party.pokemon.remove(index);
                                                pokemon.caught = true;
                                                user.client.send(ServerMessage::PokemonRequest(
                                                    index,
                                                    pokemon.pokemon.deref().clone(),
                                                ));
                                                if let Err(err) =
                                                    user.party.pokemon.try_push(pokemon)
                                                {
                                                    warn!("Could not catch pokemon because the player's party has reached maximum capacity and PCs have not been implemented.");
                                                    warn!("Error: {}", err);
                                                }
                                            }
                                        }
                                    }
                                    true
                                }
                                _ => {
                                    info!("Cannot use pokeballs in trainer battles!");
                                    false
                                }
                            },
                            ItemUseType::None => true,
                        } {
                            player_queue.push(ActionInstance {
                                pokemon: instance.pokemon,
                                action: BattleClientAction::UseItem(item, target),
                            });
                        }
                    }
                    BattleMove::Switch(new) => {
                        user.party.replace(instance.pokemon.index, Some(new));
                        let unknown = user
                            .party
                            .active_index(instance.pokemon.index)
                            .map(|index| user.party.know(index))
                            .flatten();
                        player_queue.push(ActionInstance {
                            pokemon: instance.pokemon,
                            action: BattleClientAction::Switch(new, unknown),
                        });
                    }
                }
            }
        }
        player_queue
    }
}
